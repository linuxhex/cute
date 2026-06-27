# 需求分析（Cute 主工程视角）

## 需求概述

> 在不影响当前 UI 功能与展示的前提下，彻底移除云 agent、云远端、云存储、登录、注册及非 git 的远程能力，使 Cute 成为纯本地终端产品。

## 业务背景

- Cute 基于 Warp 代码库 fork，仍保留大量云端耦合（Firebase 登录、Warp Drive 同步、Cloud Agent、Shared Session、Handoff 等）
- 稳定 5.0 已完成 CLI agent 图标等本地化修复，下一步是架构级「去云化」
- 用户明确要求保留 **本地 git 能力**（code review commit/push/PR、分支选择等）

## 本服务职责

| 职责项 | 说明 |
|--------|------|
| 移除云 agent | Ambient Agent、Cloud Mode UI、cloud task/driver、相关 CLI |
| 移除云存储 | Warp Drive 云端同步、cloud object 服务端交互（需决策本地替代） |
| 移除登录/注册 | Firebase、匿名用户、登录 UI、API Key 账号体系 |
| 移除非 git 远程 | Shared Session、Handoff、Remote Server、Remote Control 等 |
| 保留本地 UI | Tab、终端、CLI agent、侧栏、设置、Code Review git 等现有展示不变 |
| 保留 git | `util/git.rs`、code review git 对话框、repo metadata、分支 UI |

## 约束条件

| 约束 | 来源 |
|------|------|
| 不能影响 UI 现有功能和展示 | 用户硬性要求 |
| 删除云 agent / 云远端 / 云存储 | 用户要求 |
| 删除登录 / 注册 | 用户要求 |
| 删除远程相关（git 除外） | 用户要求 |
| 需求 ID | OMJF-11111 |

## 影响范围（代码地图）

### 云 Agent / Ambient Agent

| 区域 | 代表路径 |
|------|----------|
| Cloud Mode UI | `app/src/terminal/view/ambient_agent/` |
| 任务与调度 | `app/src/ai/ambient_agents/` |
| Agent SDK / Driver | `app/src/ai/agent_sdk/` |
| Agent 管理 | `app/src/ai/agent_management/` |
| 环境管理 | `app/src/pane_group/pane/environment_management_pane.rs` |
| CLI | `crates/warp_cli/` |

### 云存储 / Warp Drive

| 区域 | 代表路径 |
|------|----------|
| Drive UI | `app/src/drive/` |
| Cloud Object | `app/src/cloud_object/` |
| 服务端同步 | `app/src/server/cloud_objects/` |
| 独立 crate | `crates/cloud_objects/`、`cloud_object_models/`、`cloud_object_client/` |

### 登录 / 注册 / 账号

| 区域 | 代表路径 |
|------|----------|
| Auth 模块 | `app/src/auth/` |
| Firebase | `crates/firebase/` |
| 团队 Workspace | `app/src/workspaces/` |
| Server Auth API | `app/src/server/server_api/auth.rs` |

### 远程（非 git，待删）

| 区域 | 代表路径 |
|------|----------|
| Shared Session | `app/src/terminal/shared_session/` |
| Handoff | `app/src/ai/blocklist/handoff/`、`app/src/workspace/auto_handoff.rs` |
| Remote Server（已 stub） | `app/src/remote_server/` |
| Remote Control | `FeatureFlag::HOARemoteControl` 相关 |
| Remote Codebase | `app/src/ai/get_relevant_files/remote_search/` |

### Git（保留）

| 区域 | 代表路径 |
|------|----------|
| 本地 git 工具 | `app/src/util/git.rs`、`crates/warp_util/src/git.rs` |
| Code Review | `app/src/code_review/git_dialog/`、`diff_state/local.rs` |
| Repo 元数据 | `crates/repo_metadata/` |

### Git（随云删除）

| 区域 | 说明 |
|------|------|
| `app/src/ai/agent_sdk/driver/git_credentials.rs` | Cloud sandbox git token |
| Handoff touched_repos | 与 cloud handoff 绑定 |

## 关键产品决策点（待确认）

### 决策 1：Notebook / Workflow / EnvVar / MCP 等 Warp Drive 对象

当前大量走 `CloudModel` + 服务端同步。去云后有两种路径：

| 方案 | UI 影响 | 实现成本 |
|------|---------|----------|
| A. 纯本地化 | 保留现有入口与展示，数据仅存本地 | 高（需替换 CloudModel 持久化） |
| B. 移除相关 UI | 隐藏/删除 Notebook、Workflow 等云对象入口 | 中（可能违反「不影响展示」） |

### 决策 2：ServerApi / GraphQL 整体

| 方案 | 说明 |
|------|------|
| A. 完全移除网络层 | 删 `warp_server_client`、`graphql` 等 crate |
| B. 保留 stub/no-op | 最小改动，避免大面积编译链断裂 |

### 决策 3：MCP OAuth

MCP server OAuth 与用户账号登录不同，是否保留需单独确认。

## 已有 fork 进展

- `remote_server/`、`pricing/` 等已为 stub
- `auth/mod.rs` 部分 cloud 引用已注释
- `settings/ai.rs` 已强制启用本地 AI
- 部分 cloud environment 逻辑已 no-op

## 风险与注意

- ⚠️ **最高风险**：约束「UI 不变」与「删除云存储/登录 gating」存在张力——许多入口受 `LoginGatedFeature` 控制
- ⚠️ **启动路径**：`lib.rs`、`root_view.rs` 强依赖 AuthManager / ServerApi 初始化
- ⚠️ **default Cargo features** 仍含大量 cloud feature，需分阶段从 default 剥离
- 💡 建议分 6 阶段：Feature 剥离 → Remote 清理 → Cloud Agent → Handoff/Session → Drive/Auth → Crate 删除
- 💡 本地 git 与 cloud 路径已分离，删除风险较低

## 成功标准

- 编译通过，Cute 可正常启动运行
- 无登录/注册 UI 与流程
- 无 Cloud Mode / Ambient Agent / Warp Drive 云端同步行为
- 无 Shared Session / Handoff / Remote Control
- Tab、终端、CLI agent、侧栏、Code Review git 等本地功能与稳定 5.0 展示一致
- 本地 git（分支、commit、push、PR）可用
