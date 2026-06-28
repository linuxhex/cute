# OMJF-11111 改动记录

## 需求决策

| 决策项 | 结论 |
|--------|------|
| Warp Drive 对象（Notebook/Workflow/EnvVar/MCP） | **方案 B**：移除 UI 与相关功能 |
| 本地 git | 保留 |
| UI 约束 | 保留功能范围内（终端、CLI agent、侧栏、Code Review git）展示与稳定 5.0 一致 |

---

## 方案审查

### 业务逻辑推演

| 审查项 | 结果 |
|--------|------|
| 业务流程推演 | ✓ 本地终端 → CLI agent → Code Review git 主路径闭环；云路径全部切断 |
| 业务规则推演 | ✓ 无登录 gate 后本地功能默认可用 |
| 业务状态推演 | ✓ 移除 cloud task/session 状态机，仅保留本地 ConversationStatus |
| 业务数据推演 | ✓ 无 CloudModel 同步；本地 prefs/terminal 数据不受影响 |
| 业务异常推演 | ⚠ 历史 workspace 若含 Notebook/Workflow pane，需迁移/忽略打开（minor） |
| 业务边界推演 | ✓ 空 auth、无 network 时应用仍可启动 |
| 业务依赖关系 | ✓ git 与 cloud 路径已分离 |
| 业务异常恢复 | ✓ 无 cloud handoff 补偿逻辑需求 |

### 技术方案审查

| 审查项 | 结果 |
|--------|------|
| 文件路径正确 | ✓ 路径已对照代码库 |
| 依赖关系合理 | ✓ 分阶段自 feature → init → UI → crate，避免循环 |
| 技术方案可行 | ✓ fork 已有部分 stub，可渐进删除 |
| 接口契约一致 | ✓ 单工程，无跨服务契约 |
| 配置项完整 | ✓ `skip_login` + default feature 调整覆盖 |

### 执行可行性审查

| 审查项 | 结果 |
|--------|------|
| 步骤无遗漏 | ✓ 9 任务覆盖 cloud/auth/drive/remote/agent |
| 步骤无冲突 | ✓ 先 feature 剥离再删模块，顺序合理 |
| 资源可获取 | ✓ 本地 Rust 工具链 |
| 环境可支持 | ✓ macOS dev build |

### 审查结论

- 发现问题：1 个 minor
  - [minor] 用户旧 workspace 可能引用已删 pane 类型，打开时需 graceful fallback
- 处理方式：任务 5 中 TypedPane 打开逻辑增加忽略/关闭无效 pane
- **审查通过，可进入执行计划阶段**

---

## 执行进度

### 任务 1：Feature 剥离 ✓

- 从 `app/Cargo.toml` default 移除 50 个 cloud/auth/remote 相关 feature
- 新增 `skip_login` 到 default
- `cargo check --bin cute --features default` 通过

### 任务 2：启动链去云化（部分完成）

- `lib.rs`：注释 `auth::init`、`drive::index::init`、`drive::sharing::dialog::init`、`auto_handoff::init`
- `lib.rs`：禁用 `initialize_cloud_preferences_syncer`
- `lib.rs`：`skip_login` 下不注册系统登录项（`login_item`）
- `root_view.rs`：`skip_login` 时直接进入 workspace，不展示登录页
- `workspaces/update_manager.rs`：`skip_login` 下禁用 workspace 元数据轮询
- `workspaces/team_tester.rs`：`skip_login` 下 `initiate_data_pollers` 为 no-op
- `cargo check` 通过

### 任务 3–5：UI 入口关闭（部分完成）

- `drive/settings.rs`：`is_warp_drive_enabled()` 恒为 `false`（Notebook/Workflow/Warp Drive 入口隐藏）
- `connected_self_hosted_workers.rs`：stub，不再拉取远程 worker
- `ai/mod.rs`：禁用 `agent_management::init`
- `pane_group/mod.rs`：恢复 workspace 时跳过云 pane（Notebook/Workflow/EnvVar/AmbientAgent）；`AmbientAgent` layout 降级为本地终端
- `workspace/view.rs`：`open_notebook` / `open_workflow_*` / `open_env_var_collection` 在 `skip_login` 下 no-op
- `cargo check` 通过

### 待续（任务 3–8 物理删除）

- 物理删除 `auth/`、`drive/`、`ambient_agent/` 等模块与 `cloud_*` crate
- CloudModel / UpdateManager 启动链瘦身
- 保留本地 git 路径

### execution_phase2_runtime_guards 执行进度

#### 修复编译错误
- 修复 `app/src/workspace/one_time_modal_model.rs:52` 闭包参数名错误（`|_, event, ctx|` → `|me, event, ctx|`）
- 原因：skip_login 分支提取后闭包参数未同步更新，导致 `me` 变量未找到

#### 云模块引用深度评估

| 排序 | 模块/Crate | 外部引用次数 | 外部文件数 | 安全删除难度 |
|------|-----------|------------|-----------|------------|
| 1 | cloud_object_client crate | 12 | 8 | 低（但 warp_server_client 依赖） |
| 2 | cloud_object_models crate | 33 | 23 | 中 |
| 3 | ambient_agents 模块 | 124 | 73 | 中 |
| 4 | auth 模块 | 127 | 76 | 高（AuthStateProvider trait 全项目使用） |
| 5 | firebase crate | 145（1处use导入） | 35 | 中 |
| 6 | drive 模块 | 156+ | 84+ | 高 |
| 7 | cloud_objects crate | 202+ | 95+ | 高 |
| 8 | cloud_object 模块 | 231+ | 96+ | 高 |
| 9 | ambient_agent 模块 | 547+ | 86+ | 极高 |
| 10 | handoff 模块 | 671 | 57 | 极高 |
| 11 | shared_session 模块 | 956 | 79 | 极高（深度嵌入终端核心） |

#### lib.rs 云模块初始化代码分析

以下初始化代码仍需清理：
- `AuthManager::new(...)` (第1041行) - 认证管理器
- `GitHubAuthNotifier::new()` (第1416行) - GitHub认证通知器
- `CloudModel::new(...)` (第1452行) - 云对象模型
- `SyncQueue::new(...)` (第1471行) - 同步队列
- `TeamUpdateManager::new(...)` (第1542行) - 团队更新管理器
- `UpdateManager::new(...)` (第1550行) - 更新管理器
- `ScheduledAgentManager::new` (第1648行) - 已被FeatureFlag条件包裹

#### 安全删除策略

由于云模块引用深度高（76-96个文件），物理删除需分多次执行：
1. **第一步**：用 stub/no-op 替换云模块初始化代码（lib.rs）
2. **第二步**：逐文件清理引用，用条件编译或默认值替换
3. **第三步**：物理删除模块文件和 crate
4. 每步保证 `cargo check` 通过

#### 结论
- 当前编译状态：通过（0 errors, 328 warnings）
- 物理删除云模块是大规模重构，需后续多次执行
- 本次执行完成：编译错误修复 + 引用深度评估 + 删除策略制定

---

## 推演结论

### 第 1 轮

| 检查域 | 结果 |
|--------|------|
| 主路径闭环 | ✓ skip_login + test user → 直接进 workspace |
| 异常处理 | ✓ 未登录分支已被 skip_login 短路 |
| 契约一致 | ✓ 单工程 |
| 边界条件 | ✓ feature 剥离后 cloud 代码不参与 default 编译 |

- 发现问题：0 个 critical
- **第 1 轮收敛**

### 第 2 轮（execution_phase2_runtime_guards）

| 检查域 | 结果 |
|--------|------|
| 主路径闭环 | ✓ CLI agent rich input 功能已修复，本地终端主路径正常 |
| 异常处理 | ✓ 闭包参数错误已修复，编译通过 |
| 契约一致 | ✓ 单工程 |
| 边界条件 | ✓ 云模块初始化代码仍存在但被 FeatureFlag/skip_login 条件保护 |
| 编译状态 | ✓ cargo check --bin cute 通过（0 errors） |
| 运行时保护 | ✓ skip_login 模式下云功能被短路 |
| CLI agent 功能 | ✓ auto_open_rich_input_on_cli_agent_start 默认true，循环依赖已解除 |

- 发现问题：0 个 critical
- minor: 云模块文件未物理删除（引用深度高，需后续分批处理）
- **第 2 轮收敛**

---

## execution_phase3_physical_deletion 执行进度

### 已删除 crate

| crate | 引用次数 | 文件数 | 删除方式 | 状态 |
|-------|---------|--------|---------|------|
| firebase | 1处use导入 | 35个文件 | 内联2个类型到 auth.rs，删除crate | ✅ |
| cloud_object_client | 12次引用 | 8个文件 | 在 object.rs 创建14个stub类型，删除crate | ✅ |
| cloud_object_models | 33次引用 | 23个文件 | 内联15个源文件到 app/src/cloud_object/models/，删除crate | ✅ |

### 编译验证
- `cargo check --bin cute`：通过（0 errors, 332 warnings）
- 检查时间：2026-06-27

---

## 编译/构建检查

- `cargo check --bin cute`：通过（0 errors, 332 warnings）
- 检查时间：2026-06-27
  - `docs/requirements/OMJF-11111/changes.md` - 更新执行进度
  - `docs/requirements/OMJF-11111/workflow-state.json` - 更新阶段状态
