# OMJF-11111 去云化实现计划

**目标：** 移除云 agent、云存储、登录/注册及非 git 远程能力；Notebook/Workflow/EnvVar/MCP 等 Warp Drive UI 一并移除；保留本地终端、CLI agent、Code Review git 及稳定 5.0 核心 UI。

**架构：** 分 6 阶段渐进删除——先从 Cargo default features 与启动链剥离，再删 UI 入口与模块，最后删 crate 与 stub 网络层；每阶段保证 `cargo check` 通过。

**技术栈：** Rust / warpui / Cargo feature flags

---

## 文件清单（按阶段）

| 阶段 | 主要目录/文件 | 操作 |
|------|---------------|------|
| 0 | `app/Cargo.toml`、`app/src/features.rs` | 修改 default features |
| 1 | `app/src/remote_server/`、`remote_search/`、`remote_server_executor.rs` | 删除引用与模块 |
| 2 | `app/src/terminal/view/ambient_agent/`、`ai/ambient_agents/`、`ai/agent_sdk/`（cloud 路径） | 删除 |
| 3 | `terminal/shared_session/`、`ai/blocklist/handoff/`、`workspace/auto_handoff.rs` | 删除 |
| 4 | `app/src/drive/`、`cloud_object/`、`notebooks/`、`workflows/`、`env_vars/` UI 入口 | 删除/瘦身 |
| 5 | `app/src/auth/`、`crates/firebase/`、`workspaces/`、`lib.rs`、`root_view.rs` | 删除 auth 链 |
| 6 | `crates/cloud_*`、`graphql` 子集、`warp_server_client` | 删 crate 或 stub |

---

## 任务拆分

### 任务 1：Feature 剥离与编译开关

**目标：** default build 不再编译 cloud/auth/remote/session 相关 feature。

**文件：**
- 修改：`app/Cargo.toml`（`default` features 列表）
- 修改：`app/src/features.rs`
- 修改：`crates/warp_features/src/lib.rs`（如有 dead flag 清理）

**实现要点：**
- 从 `default` 移除：`cloud_mode*`、`ambient_agents_*`、`scheduled_ambient_agents`、`cloud_conversations`、`cloud_environments`、`handoff_*`、`oz_handoff`、`viewing_shared_sessions`、`creating_shared_sessions`、`shared_with_me`、`agent_shared_sessions`、`hoa_remote_control`、`remote_codebase_indexing`、`remote_code_review`、`loginless_conversion`、`api_key_*`、`cute_managed_secrets`、`sync_ambient_plans`、`git_credential_refresh` 等
- 保留：`agent_mode`、`mcp_server`（仅本地 MCP）、`git_operations_in_code_review`、`skip_login` 或等价本地模式
- 新增/启用：`skip_login` 进入 default，确保无登录 gate

---

### 任务 2：启动链去云化（lib.rs / root_view）

**目标：** 应用启动不再初始化 Auth、CloudModel、UpdateManager、Drive、Team。

**文件：**
- 修改：`app/src/lib.rs`
- 修改：`app/src/root_view.rs`
- 修改：`app/src/workspace/view.rs`（auth 事件订阅、login-gated actions）

**实现要点：**
- 移除 `AuthManager::new` 及 handle 注册（或替换为 no-op stub trait）
- 移除 `CloudModel::new`、`UpdateManager::new`、`TeamUpdateManager::new`
- 移除 `drive::index::init`
- 移除 `ScheduledAgentManager` 等 cloud agent singleton
- `root_view` 不再渲染登录 slide / auth modal
- 清理 `LoginGatedFeature` 调用，改为始终可用或删除对应入口

---

### 任务 3：移除 Cloud Agent / Ambient Agent UI

**目标：** 删除 Cloud Mode、Agent 管理（cloud task）、环境管理 pane。

**文件：**
- 删除：`app/src/terminal/view/ambient_agent/`（整目录）
- 删除：`app/src/ai/ambient_agents/`（整目录）
- 修改：`app/src/ai/agent_sdk/`（移除 cloud driver、cloud_provider、git_credentials、ambient 集成）
- 修改：`app/src/ai/agent_management/`（移除 cloud task 列表，保留本地 agent 如有）
- 删除：`app/src/pane_group/pane/environment_management_pane.rs` 及引用
- 修改：`app/src/settings_view/environments_page.rs`（删除或隐藏）
- 修改：`app/src/ui_components/icon_with_status.rs`（移除 Oz cloud / is_ambient 分支，保留 CLI agent）
- 修改：`app/src/workspace/view/vertical_tabs.rs`（移除 OzAgent ambient、cloud badge）

**实现要点：**
- `Indicator::AmbientAgent`、`AgentActive` cloud 路径改为仅本地 CLI/Oz
- 命令面板移除 cloud agent / environment 相关命令

---

### 任务 4：移除 Shared Session / Handoff / Remote Control

**目标：** 删除会话分享、local↔cloud handoff、远程控制 chip。

**文件：**
- 删除：`app/src/terminal/shared_session/`（整目录）
- 删除：`app/src/terminal/view/shared_session/`
- 删除：`app/src/ai/blocklist/handoff/`（整目录）
- 删除：`app/src/workspace/auto_handoff.rs`
- 修改：`app/src/pane_group/pane/view/header/sharing.rs`（删除分享 UI）
- 修改：`app/src/terminal/view/inline_banner/shared_sessions.rs`

**实现要点：**
- 保留本地 terminal session，删除 sharer/viewer 网络层
- 删除 `FeatureFlag::HOARemoteControl` 相关 UI

---

### 任务 5：移除 Warp Drive 及 Notebook/Workflow/EnvVar/MCP UI（方案 B）

**目标：** 按用户决策 B，移除所有 Warp Drive 对象 UI 与同步逻辑。

**文件：**
- 删除：`app/src/drive/`（整目录）
- 删除：`app/src/cloud_object/`（整目录）
- 删除：`app/src/server/cloud_objects/`
- 修改：`app/src/notebooks/`（移除 manager/UI 入口，保留 agent 引用处 stub）
- 修改：`app/src/workflows/`（同上）
- 修改：`app/src/env_vars/`（同上）
- 修改：`app/src/search/command_palette/warp_drive/`
- 修改：`app/src/settings_view/warp_drive_page.rs`
- 修改：`app/src/workspace/view/vertical_tabs.rs`（移除 TypedPane::Notebook/Workflow/EnvVarCollection/AIFact 等）
- 修改：`app/src/pane_group/`（移除对应 pane 类型）

**实现要点：**
- 从 `TypedPane` enum 移除或 unreachable 云对象 pane 类型
- 命令面板、侧栏、tab 不再出现 Notebook/Workflow/EnvVar/MCP 入口
- MCP：移除 cloud gallery/sync；保留本地 `mcp_server` 配置（若无需登录）

---

### 任务 6：移除登录 / 注册 / Firebase / Teams

**目标：** 彻底删除用户账号体系与登录 UI。

**文件：**
- 删除：`app/src/auth/`（整目录）
- 删除：`crates/firebase/`（整目录及 workspace 引用）
- 删除：`app/src/workspaces/`（teams 同步）
- 删除：`app/src/login_item/`
- 修改：`app/src/server/server_api/auth.rs`、`managed_secrets.rs`
- 修改：`app/src/settings_view/platform/`（API Key UI）
- 修改：`app/src/menu.rs`（登录/账号菜单项）

**实现要点：**
- ServerApi 请求不再附加 Firebase token
- 删除 anonymous user、SSO、paste token、web handoff
- 设置页移除账号、团队、API Key 管理

---

### 任务 7：Remote stub 与 cloud git 清理

**目标：** 删除已 stub 的 remote 模块引用及 cloud sandbox git。

**文件：**
- 删除：`app/src/remote_server/`
- 删除：`app/src/code_review/diff_state/remote.rs`
- 删除：`app/src/ai/get_relevant_files/remote_search/`
- 删除：`app/src/terminal/writeable_pty/remote_server_controller.rs`
- 删除：`app/src/ai/agent_sdk/driver/git_credentials.rs`
- 修改：`app/src/code_review/diff_state/mod.rs`（仅保留 local）

**保留：**
- `app/src/util/git.rs`
- `app/src/code_review/git_dialog/`
- `app/src/code_review/diff_state/local.rs`
- `crates/repo_metadata/`

---

### 任务 8：CLI 与 crate 清理

**目标：** 移除 `warp_cli` cloud 子命令及独立 cloud crate。

**文件：**
- 修改：`crates/warp_cli/`（移除 agent run/task/environment/schedule/share 等 cloud 命令）
- 删除或 stub：`crates/cloud_objects/`、`cloud_object_models/`、`cloud_object_client/`
- 修改：`crates/warp_server_client/`（移除或 no-op）
- 修改：`crates/graphql/`（删除 cloud/agent/drive/auth 相关 query/mutation）
- 修改：根 `Cargo.toml` workspace members
- 删除：`app/src/integration_testing/cloud_object/`

---

### 任务 9：UI 回归与编译验证

**目标：** 确认稳定 5.0 核心 UI 不变且可运行。

**验证项：**
- Tab / 侧栏 / Footer CLI agent 图标与 InProgress 状态（稳定 5.0）
- 本地终端新建、输入、block 渲染
- CLI agent rich input
- Code Review git commit/push/PR
- 分支选择器
- 无登录弹窗、无 Cloud Mode 入口、无 Warp Drive 入口
- `cargo check --bin cute --features default`
- `./script/run` 启动成功

**注意：** 不编写单元测试；由推演收敛替代。
