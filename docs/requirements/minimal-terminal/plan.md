# 最小化终端实现计划

**目标：** 将 Cute 终端简化为最小化本地终端，移除所有云/远程功能相关代码引用。

**架构：** 单服务本地终端，仅保留本地 shell 会话、本地文件编辑、本地终端模拟功能。

**技术栈：** Rust, cuteui, tokio

---

## 文件结构

**需要修改的文件：**
- `app/src/lib.rs` — 移除模块声明和 LaunchMode 变体
- `app/Cargo.toml` — 移除云相关 features
- `app/src/features.rs` — 移除云相关 feature flags
- `app/src/workspace/view.rs` — 移除 resource_center/billing 引用
- `app/src/workspace/mod.rs` — 移除相关 action bindings
- `app/src/workspace/util.rs` — 移除 resource_center 相关字段
- `app/src/global_resource_handles.rs` — 移除 TipsCompleted 引用
- `app/src/settings/init.rs` — 移除 TipsCompleted 引用
- `app/src/root_view.rs` — 移除 billing 引用
- `app/src/terminal/event.rs` — 移除 RemoteServerSetupState 导出
- `app/src/coding_panel_enablement_state.rs` — 移除 has_remote_server 字段
- `app/src/integration_testing/mod.rs` — 移除 remote_server 模块

**需要删除的文件：**
- `app/src/integration_testing/remote_server.rs`
- `app/src/terminal/writeable_pty/remote_server_controller.rs`

---

## 任务 1：清理 app/Cargo.toml 的 default features

**文件：**
- 修改：`app/Cargo.toml:397-582`

- [ ] **步骤 1：从 default features 中移除云相关 features**

从 default features 列表中移除以下 features：
- `drive_objects_as_context`
- `mcp_oauth`
- `api_key_authentication`
- `api_key_management`
- `cloud_environments`
- `create_environment_slash_command`
- `cute_managed_secrets`
- `cloud_conversations`
- `cloud_mode`
- `cloud_mode_from_local_session`
- `cloud_mode_image_context`
- `cloud_mode_input_v2`
- `cloud_mode_setup_v2`
- `hoa_remote_control`
- `handoff_local_cloud`
- `handoff_cloud_cloud`
- `remote_codebase_indexing`
- `billing_and_usage_page_v2`

---

## 任务 2：清理 app/src/lib.rs 的模块声明和 LaunchMode

**文件：**
- 修改：`app/src/lib.rs:118`
- 修改：`app/src/lib.rs:1959-1975`

- [ ] **步骤 1：移除 resource_center 模块声明**

删除第 118 行：
```rust
pub mod resource_center;
```

- [ ] **步骤 2：移除 LaunchMode::RemoteServerProxy 和 RemoteServerDaemon 变体**

删除 `launch` 函数中的相关 match 分支（约 1959-1975 行）：
```rust
// 删除以下代码
LaunchMode::RemoteServerProxy => {
    log::error!("Proxy mode should not use the launch() path");
    std::process::exit(1);
}
#[cfg(unix)]
LaunchMode::RemoteServerDaemon { identity_key } => {
    remote_server::unix::launch_daemon(&identity_key, ctx);
}
#[cfg(not(unix))]
LaunchMode::RemoteServerDaemon { .. } => {
    log::error!("RemoteServerDaemon is not supported on this platform");
    std::process::exit(1);
}
```

---

## 任务 3：清理 app/src/features.rs 的云相关 feature flags

**文件：**
- 修改：`app/src/features.rs`

- [ ] **步骤 1：移除或注释云相关 feature flag 定义**

移除或注释以下 feature flags：
- `CloudMode`
- `CloudModeFromLocalSession`
- `CloudModeImageContext`
- `CloudModeInputV2`
- `CloudModeSetupV2`
- `CloudConversations`
- `CloudEnvironments`
- `RemoteCodebaseIndexing`
- `HandoffLocalCloud`
- `HandoffCloudCloud`
- `HoaRemoteControl`
- `DriveObjectsAsContext`
- `McpOAuth`
- `ApiKeyAuthentication`
- `ApiKeyManagement`
- `CuteManagedSecrets`
- `BillingAndUsagePageV2`

---

## 任务 4：清理 workspace/view.rs 的引用

**文件：**
- 修改：`app/src/workspace/view.rs`

- [ ] **步骤 1：移除 resource_center 导入**

删除或注释第 289 行的导入：
```rust
use crate::resource_center::{...};
```

- [ ] **步骤 2：移除 billing 导入**

删除第 223 行的导入：
```rust
use crate::billing::shared_objects_creation_denied_modal::{...};
```

- [ ] **步骤 3：移除 resource_center_view 字段**

从 WorkspaceView 结构体中移除 `resource_center_view` 字段（约第 984 行）。

- [ ] **步骤 4：移除 build_resource_center_view 方法**

删除 `build_resource_center_view` 方法（约第 1562-1575 行）。

- [ ] **步骤 5：移除 TOGGLE_RESOURCE_CENTER_KEYBINDING_NAME**

删除第 565 行的常量定义。

---

## 任务 5：清理 workspace/mod.rs 的 action bindings

**文件：**
- 修改：`app/src/workspace/mod.rs`

- [ ] **步骤 1：移除 resource_center toggle action**

删除第 1091 行的 action binding：
```rust
"workspace:toggle_resource_center",
```

- [ ] **步骤 2：移除 billing action**

删除第 1464-1466 行的 action binding：
```rust
"workspace:show_settings_billing_and_usage_page",
BindingDescription::new("Open Settings: Billing and usage"),
WorkspaceAction::ShowSettingsPage(SettingsSection::BillingAndUsage),
```

---

## 任务 6：清理 workspace/util.rs

**文件：**
- 修改：`app/src/workspace/util.rs`

- [ ] **步骤 1：移除 resource_center_icon 字段**

删除第 28 行的字段定义：
```rust
pub(super) resource_center_icon: MouseStateHandle,
```

- [ ] **步骤 2：移除 is_resource_center_open 字段**

删除第 98 行的字段定义：
```rust
pub is_resource_center_open: bool,
```

- [ ] **步骤 3：修改 is_panel_open 方法**

修改第 213 行的方法，移除 resource_center 检查：
```rust
self.is_ai_assistant_panel_open
```

---

## 任务 7：清理其他文件的引用

**文件：**
- 修改：`app/src/global_resource_handles.rs`
- 修改：`app/src/settings/init.rs`
- 修改：`app/src/root_view.rs`
- 修改：`app/src/terminal/event.rs`
- 修改：`app/src/coding_panel_enablement_state.rs`

- [ ] **步骤 1：移除 global_resource_handles.rs 的 TipsCompleted 导入**

删除第 8 行：
```rust
use crate::resource_center::TipsCompleted;
```

- [ ] **步骤 2：移除 settings/init.rs 的 TipsCompleted 导入**

删除第 21 行：
```rust
use crate::resource_center::TipsCompleted;
```

- [ ] **步骤 3：移除 root_view.rs 的 billing 导入**

删除第 22 行：
```rust
use cute_graphql::billing::StripeSubscriptionPlan;
```

- [ ] **步骤 4：移除 terminal/event.rs 的 RemoteServerSetupState 导出**

删除第 9 行：
```rust
pub use remote_server::setup::RemoteServerSetupState;
```

- [ ] **步骤 5：简化 coding_panel_enablement_state.rs**

移除 `has_remote_server` 字段相关代码。

---

## 任务 8：删除 integration_testing/remote_server.rs

**文件：**
- 删除：`app/src/integration_testing/remote_server.rs`
- 修改：`app/src/integration_testing/mod.rs`

- [ ] **步骤 1：删除 remote_server.rs 文件**

- [ ] **步骤 2：移除 mod.rs 的模块声明**

删除 `app/src/integration_testing/mod.rs` 第 27 行：
```rust
pub mod remote_server;
```

---

## 任务 9：删除 remote_server_controller.rs

**文件：**
- 删除：`app/src/terminal/writeable_pty/remote_server_controller.rs`

- [ ] **步骤 1：删除文件**

---

## 任务 10：编译验证

- [ ] **步骤 1：运行 cargo check 验证编译**

```bash
cargo check --package cute 2>&1 | head -100
```

如有编译错误，根据错误信息修复剩余引用。

---

## 风险点

1. **大量代码引用需要清理** — 需要逐步修复编译错误
2. **features 之间有依赖关系** — 移除时需要注意依赖顺序
3. **编译验证需要逐步进行** — 每次修改后验证编译状态
