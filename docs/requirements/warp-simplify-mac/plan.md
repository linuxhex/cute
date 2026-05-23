# cute 终端实现计划

**目标：** 基于 Warp 终端改造一个精简版 Mac 终端，名为 cute

**架构：** 移除 AI、云同步、多平台支持，新增 Git Diff 侧边栏和窗口 + 号功能

**技术栈：** Rust + warpui（自研 UI 框架），仅支持 macOS

---

## 文件结构概览

### 需要删除的目录

**Crates（子模块）**：
- `crates/ai/` - AI 核心功能
- `crates/firebase/` - Firebase 集成
- `crates/graphql/` - GraphQL 客户端
- `crates/warp_graphql_schema/` - GraphQL Schema
- `crates/remote_server/` - 远程服务器

**App 模块**：
- `app/src/ai/` - AI 功能模块
- `app/src/ai_assistant/` - AI 助手面板
- `app/src/cloud_object/` - 云对象系统
- `app/src/firebase/` - Firebase 集成

### 需要修改的文件

- `Cargo.toml` - 移除相关 workspace members
- `app/Cargo.toml` - 移除相关依赖和 features
- `app/src/lib.rs` - 移除模块引用
- `app/src/root_view.rs` - 移除 AI 相关引用
- `app/src/workspace/view/left_panel.rs` - 移除 AI 相关面板
- `app/src/workspace/view/right_panel.rs` - 移除 AI 相关面板

### 需要新增的文件

- `app/src/git_diff_panel/` - Git Diff 侧边栏模块
- `app/src/git_diff_panel/mod.rs`
- `app/src/git_diff_panel/panel.rs`
- `app/src/git_diff_panel/model.rs`

---

## 任务列表

### 任务 1：移除 AI 相关 Crates

**文件：**
- 修改：`Cargo.toml`
- 删除：`crates/ai/`

- [ ] **步骤 1：从 workspace 移除 AI crate**

修改 `Cargo.toml`，从 members 中移除 `crates/ai`，从 dependencies 中移除 `ai` 相关依赖。

```toml
# 移除以下内容：
members = [
  "crates/*",  # ai 会被自动包含，需要改为显式列出除 ai 外的所有 crates
]

# 或者在 crates/*/Cargo.toml 中移除对 ai 的依赖
```

- [ ] **步骤 2：删除 crates/ai 目录**

```bash
rm -rf crates/ai
```

---

### 任务 2：移除 Firebase 相关 Crates

**文件：**
- 修改：`Cargo.toml`
- 删除：`crates/firebase/`

- [ ] **步骤 1：从 workspace 移除 firebase crate**

修改 `Cargo.toml`，移除 firebase 相关依赖。

- [ ] **步骤 2：删除 crates/firebase 目录**

```bash
rm -rf crates/firebase
```

---

### 任务 3：移除 GraphQL 相关 Crates

**文件：**
- 修改：`Cargo.toml`
- 删除：`crates/graphql/`
- 删除：`crates/warp_graphql_schema/`

- [ ] **步骤 1：从 workspace 移除 graphql crates**

修改 `Cargo.toml`，移除 graphql 相关依赖。

- [ ] **步骤 2：删除 graphql 相关目录**

```bash
rm -rf crates/graphql crates/warp_graphql_schema
```

---

### 任务 4：移除 App 中的 AI 模块

**文件：**
- 删除：`app/src/ai/`
- 删除：`app/src/ai_assistant/`
- 修改：`app/src/lib.rs`
- 修改：`app/Cargo.toml`

- [ ] **步骤 1：删除 AI 模块目录**

```bash
rm -rf app/src/ai app/src/ai_assistant
```

- [ ] **步骤 2：修改 lib.rs 移除模块引用**

移除 `mod ai;` 和 `mod ai_assistant;` 声明。

- [ ] **步骤 3：修改 app/Cargo.toml 移除 AI 依赖**

移除 `ai.workspace = true` 等相关依赖。

---

### 任务 5：移除 App 中的云同步模块

**文件：**
- 删除：`app/src/cloud_object/`
- 删除：`app/src/firebase/`
- 修改：`app/src/lib.rs`

- [ ] **步骤 1：删除云同步模块目录**

```bash
rm -rf app/src/cloud_object app/src/firebase
```

- [ ] **步骤 2：修改 lib.rs 移除模块引用**

移除 `mod cloud_object;` 和 `mod firebase;` 声明。

---

### 任务 6：移除平台条件编译配置

**文件：**
- 修改：`app/Cargo.toml`
- 修改：`crates/warpui/Cargo.toml`
- 修改：`crates/warpui_core/Cargo.toml`
- 修改：`crates/warpui_extras/Cargo.toml`

- [ ] **步骤 1：移除 Windows/Linux 条件编译**

在 `app/Cargo.toml` 中移除以下配置块：

```toml
# 移除这些配置块
[target.'cfg(any(target_os = "linux", target_os = "freebsd", target_os = "windows"))'.dependencies]
[target.'cfg(any(target_os = "linux", target_os = "freebsd"))'.dependencies]
[target.'cfg(target_os = "windows")'.dependencies]
```

- [ ] **步骤 2：在各 crates 中移除平台配置**

遍历所有 crates 的 Cargo.toml，移除 Windows/Linux 相关配置。

---

### 任务 7：清理代码中的 AI 引用

**文件：**
- 修改：`app/src/root_view.rs`
- 修改：`app/src/workspace/view/left_panel.rs`
- 修改：`app/src/workspace/view/right_panel.rs`
- 修改：`app/src/pane_group/mod.rs`

- [ ] **步骤 1：移除 root_view.rs 中的 AI 引用**

移除 `use crate::ai::*` 相关导入，移除 AI 相关逻辑。

- [ ] **步骤 2：移除 left_panel.rs 中的 AI 面板**

移除 `ConversationListView` 相关代码，移除 AI 对话列表按钮。

- [ ] **步骤 3：移除 right_panel.rs 中的 AI 相关代码**

移除 `ReviewDestination` 等与 AI 相关的代码。

- [ ] **步骤 4：移除 pane_group 中的 AI 引用**

移除 `AIConversation`、`AIAgentHarness` 等引用。

---

### 任务 8：重命名产品为 cute

**文件：**
- 修改：`app/Cargo.toml`
- 修改：`app/src/root_view.rs`
- 修改：`about.toml`

- [ ] **步骤 1：修改 app/Cargo.toml**

```toml
[package]
name = "cute"
version = "0.1.0"

[package.metadata.bundle.bin.warp]
name = "Cute"
```

- [ ] **步骤 2：修改窗口标题**

在 `app/src/root_view.rs` 中：

```rust
const WINDOW_TITLE: &str = "cute";
```

- [ ] **步骤 3：修改 about.toml**

更新产品名称和相关信息。

---

### 任务 9：新增 Git Diff 侧边栏模块

**文件：**
- 创建：`app/src/git_diff_panel/mod.rs`
- 创建：`app/src/git_diff_panel/panel.rs`
- 创建：`app/src/git_diff_panel/model.rs`
- 修改：`app/src/lib.rs`

- [ ] **步骤 1：创建 git_diff_panel/mod.rs**

```rust
mod model;
mod panel;

pub use panel::GitDiffPanel;
pub use model::{GitDiffModel, GitDiffModelEvent, GitFileStatus};
```

- [ ] **步骤 2：创建 git_diff_panel/model.rs**

实现 Git 状态模型：
- 获取 git status 输出
- 解析变更文件列表
- 定义 GitFileStatus 枚举（Added/Modified/Deleted/Renamed）

```rust
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq)]
pub enum GitFileStatus {
    Added,
    Modified,
    Deleted,
    Renamed,
}

#[derive(Clone, Debug)]
pub struct GitFile {
    pub path: PathBuf,
    pub status: GitFileStatus,
}

pub struct GitDiffModel {
    files: Vec<GitFile>,
}

impl GitDiffModel {
    pub fn new() -> Self { ... }
    pub fn refresh(&mut self, repo_path: &Path) { ... }
    pub fn files(&self) -> &[GitFile] { ... }
}
```

- [ ] **步骤 3：创建 git_diff_panel/panel.rs**

参考 `app/src/drive/panel.rs` 实现 Git Diff 面板：

```rust
pub struct GitDiffPanel {
    model: ModelHandle<GitDiffModel>,
    width_handle: ResizableStateHandle,
}

impl GitDiffPanel {
    pub fn new(ctx: &mut ViewContext<Self>) -> Self { ... }
    fn render_file_list(&self, ctx: &mut ViewContext<Self>) -> Element { ... }
}

impl View for GitDiffPanel {
    fn view(&mut self, ctx: &mut ViewContext<Self>) -> Element { ... }
}
```

- [ ] **步骤 4：在 lib.rs 中添加模块**

```rust
mod git_diff_panel;
```

---

### 任务 10：集成 Git Diff 侧边栏到 Workspace

**文件：**
- 修改：`app/src/workspace/view/mod.rs`
- 修改：`app/src/workspace/view/right_panel.rs`

- [ ] **步骤 1：修改 right_panel.rs 添加 Git Diff 面板**

在右侧面板中集成 GitDiffPanel，替换原有的 CodeReview 相关内容。

- [ ] **步骤 2：添加切换快捷键**

定义 `TOGGLE_GIT_DIFF_PANEL_BINDING_NAME` 快捷键。

---

### 任务 11：新增 Tab 栏 + 号功能

**文件：**
- 修改：`app/src/workspace/view/mod.rs`
- 修改：`app/src/pane_group/pane/view/header/mod.rs`

- [ ] **步骤 1：在 Tab 栏添加 + 按钮**

在 workspace view 的 Tab 栏右侧添加 + 按钮。

- [ ] **步骤 2：实现新窗口逻辑**

点击 + 号时：
1. 获取当前活动 Tab 的工作目录
2. 打开新窗口
3. 新窗口继承该工作目录
4. 侧边栏不增加目录项

---

### 任务 12：清理 Features 配置

**文件：**
- 修改：`app/Cargo.toml`
- 修改：`app/src/features.rs`

- [ ] **步骤 1：移除 AI 相关 features**

移除 `agent_mode`、`ai_resume_button` 等 AI 相关 features。

- [ ] **步骤 2：移除云同步相关 features**

移除 `cloud_object_initial_load`、`shared_with_me` 等云同步相关 features。

---

### 任务 13：清理依赖引用

**文件：**
- 修改：多个 `app/src/**/*.rs` 文件

- [ ] **步骤 1：全局搜索并移除 AI 引用**

```bash
grep -r "use crate::ai" app/src/ --include="*.rs" -l
```

逐个文件移除 AI 相关导入和代码。

- [ ] **步骤 2：全局搜索并移除云同步引用**

```bash
grep -r "use crate::cloud_object\|use crate::firebase" app/src/ --include="*.rs" -l
```

逐个文件移除云同步相关导入和代码。

---

### 任务 14：编译验证

- [ ] **步骤 1：执行 cargo check**

```bash
cargo check
```

修复所有编译错误。

- [ ] **步骤 2：执行 cargo build**

```bash
cargo build --release
```

确保编译成功。

---

## 执行顺序

```
任务 1-5：移除模块（可并行）
    ↓
任务 6：移除平台配置
    ↓
任务 7：清理代码引用
    ↓
任务 8：重命名产品
    ↓
任务 9-11：新增功能
    ↓
任务 12-13：清理配置
    ↓
任务 14：编译验证
```
