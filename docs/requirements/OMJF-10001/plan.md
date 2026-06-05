# Git分支选择器半弹框实现计划

## 目标
实现一个从底部拉起的半弹框，用于Git分支管理，支持三栏布局、分支切换/合并、文件diff对比等功能。

## 架构概述
- **容器层**：BranchSelectorSheet作为半弹框容器，管理显示状态和动画
- **布局层**：Flex::row()实现左中右三栏布局
- **组件层**：BranchList、BranchDetail、ChangedFileList、DiffViewer等独立组件
- **数据层**：通过util/git模块调用Git命令获取数据

## 技术方案
- **框架**：warpui（现有UI框架）
- **布局**：Flex::row() + Flex::column()
- **动画**：OffsetPositioning实现滑入/滑出动画
- **样式**：玻璃态效果（backdrop-filter + 半透明背景）
- **状态管理**：WorkspaceState中添加分支选择器状态

## 文件清单

| 文件 | 操作 | 职责 |
|------|------|------|
| app/src/workspace/branch_selector/mod.rs | 新增 | 分支选择器主模块 |
| app/src/workspace/branch_selector/sheet.rs | 新增 | 半弹框容器组件 |
| app/src/workspace/branch_selector/branch_list.rs | 新增 | 左栏分支列表组件 |
| app/src/workspace/branch_selector/branch_detail.rs | 新增 | 中栏分支详情组件 |
| app/src/workspace/branch_selector/file_list.rs | 新增 | 右栏文件列表组件 |
| app/src/workspace/branch_selector/diff_viewer.rs | 新增 | Diff对比视图组件 |
| app/src/workspace/branch_selector/context_menu.rs | 新增 | 右键菜单组件 |
| app/src/workspace/branch_selector/state.rs | 新增 | 状态管理 |
| app/src/workspace/action.rs | 修改 | 添加分支选择器相关Action |
| app/src/workspace/view.rs | 修改 | 集成分支选择器渲染 |
| app/src/util/git.rs | 修改 | 添加获取改动文件和diff的方法 |

---

## 任务拆分

### 任务 1：定义数据结构和状态

**目标**：定义分支选择器所需的数据结构和状态管理

**文件**：
- 新增：`app/src/workspace/branch_selector/state.rs`
- 新增：`app/src/workspace/branch_selector/mod.rs`

**实现要点**：
- 定义BranchInfo结构体（分支名称、是否当前、是否远程、最后提交等）
- 定义CommitInfo结构体（hash、消息、作者、时间）
- 定义ChangedFile结构体（路径、状态、改动行数）
- 定义DiffHunk结构体（差异块）
- 定义BranchSelectorState结构体（管理整个选择器的状态）
- 实现Default trait

**核心逻辑示意**：
```rust
pub struct BranchSelectorState {
    pub is_open: bool,
    pub branches: Vec<BranchInfo>,
    pub selected_branch_index: Option<usize>,
    pub changed_files: Vec<ChangedFile>,
    pub selected_file_index: Option<usize>,
    pub diff_viewer_open: bool,
    pub current_diff: Option<FileDiff>,
    pub loading: bool,
}
```

---

### 任务 2：添加Git操作方法

**目标**：在util/git中添加获取分支详情、改动文件、diff的方法

**文件**：
- 修改：`app/src/util/git.rs`

**修改内容**：
- 添加`get_branch_info`方法：获取分支详细信息
- 添加`get_changed_files_between_branches`方法：获取两个分支间的改动文件
- 添加`get_file_diff`方法：获取文件的diff内容
- 添加`get_branch_stats`方法：获取分支统计（领先/落后提交数）

**注意**：使用现有的`run_git_command`包装Git命令调用

---

### 任务 3：添加WorkspaceAction

**目标**：添加打开/关闭分支选择器的Action

**文件**：
- 修改：`app/src/workspace/action.rs`

**修改内容**：
- 添加`OpenBranchSelector` Action
- 添加`CloseBranchSelector` Action
- 添加`SelectBranch` Action（选择分支）
- 添加`OpenDiffViewer` Action（打开diff视图）
- 添加`CloseDiffViewer` Action（关闭diff视图）
- 实现各Action的处理逻辑

---

### 任务 4：实现分支列表组件

**目标**：实现左栏分支列表渲染和交互

**文件**：
- 新增：`app/src/workspace/branch_selector/branch_list.rs`

**实现要点**：
- 渲染本地分支列表（带当前分支标记）
- 渲染远程分支列表（可折叠）
- 每个分支项显示：图标、名称、最后提交时间
- 实现单击选择分支
- 实现右键弹出菜单
- 实现悬停高亮
- 实现键盘导航（上下键）

**核心逻辑示意**：
```rust
fn render_branch_list(state: &BranchSelectorState, app: &AppContext) -> Box<dyn Element> {
    Flex::column()
        .with_child(render_local_branches(state))
        .with_child(render_remote_branches(state))
        .finish()
}
```

---

### 任务 5：实现分支详情组件

**目标**：实现中栏分支详情展示

**文件**：
- 新增：`app/src/workspace/branch_selector/branch_detail.rs`

**实现要点**：
- 显示分支名称（大标题）
- 显示最后提交信息（hash、消息、作者、时间）
- 显示分支统计（领先/落后提交数、文件改动统计）
- 实现点击hash复制到剪贴板
- 使用图标增强可读性

---

### 任务 6：实现文件列表组件

**目标**：实现右栏改动文件列表渲染和交互

**文件**：
- 新增：`app/src/workspace/branch_selector/file_list.rs`

**实现要点**：
- 渲染文件列表，每个文件显示：状态图标、路径、改动行数
- 状态图标颜色：M=黄色、A=绿色、D=红色、R=蓝色
- 实现单击高亮
- 实现双击打开diff视图
- 实现右键菜单
- 文件路径使用等宽字体

---

### 任务 7：实现Diff对比视图

**目标**：实现文件diff对比的左右分栏视图

**文件**：
- 新增：`app/src/workspace/branch_selector/diff_viewer.rs`

**实现要点**：
- 左右分栏布局（旧版本 vs 新版本）
- 高亮差异行（删除=红色、新增=绿色、修改=黄色）
- 行号对齐
- 实现滚动同步
- 实现关闭按钮
- 实现ESC键关闭

**核心逻辑示意**：
```rust
fn render_diff_viewer(diff: &FileDiff, app: &AppContext) -> Box<dyn Element> {
    Flex::row()
        .with_child(render_old_version(diff))
        .with_child(render_new_version(diff))
        .finish()
}
```

---

### 任务 8：实现右键菜单

**目标**：实现分支操作的右键菜单

**文件**：
- 新增：`app/src/workspace/branch_selector/context_menu.rs`

**实现要点**：
- 菜单项：切换分支、合并分支、创建新分支、删除分支、在浏览器中查看
- 根据分支状态动态显示/隐藏菜单项
- 危险操作（删除）用红色标识
- 显示快捷键提示
- 实现点击菜单项执行对应操作

---

### 任务 9：实现半弹框容器

**目标**：实现从底部拉起的半弹框容器，管理显示/隐藏动画

**文件**：
- 新增：`app/src/workspace/branch_selector/sheet.rs`

**实现要点**：
- 使用Stack::add_positioned_overlay_child实现overlay
- 使用OffsetPositioning实现从底部滑入动画
- 实现半透明背景遮罩
- 实现点击遮罩关闭
- 实现玻璃态效果（backdrop-filter: blur(20px)）
- 实现顶部圆角和阴影
- 高度占屏幕60-70%，底部留空间给状态栏

**核心逻辑示意**：
```rust
fn render_branch_selector_sheet(state: &BranchSelectorState, app: &AppContext) -> Box<dyn Element> {
    Stack::new()
        .add_child(render_backdrop())
        .add_positioned_overlay_child(
            render_sheet_content(state, app),
            OffsetPositioning::offset_from_parent(
                vec2f(0., calculate_slide_offset(state)),
                ParentOffsetBounds::WindowBySize,
                ParentAnchor::BottomLeft,
                ChildAnchor::BottomLeft,
            ),
        )
        .finish()
}
```

---

### 任务 10：集成到Workspace

**目标**：将分支选择器集成到Workspace中

**文件**：
- 修改：`app/src/workspace/view.rs`
- 修改：`app/src/workspace/mod.rs`

**修改内容**：
- 在WorkspaceState中添加BranchSelectorState字段
- 在Workspace::render中渲染分支选择器（如果打开）
- 处理OpenBranchSelector、CloseBranchSelector等Action
- 连接右下角分支按钮的点击事件

---

### 任务 11：实现动画和过渡效果

**目标**：实现流畅的滑入/滑出动画

**文件**：
- 修改：`app/src/workspace/branch_selector/sheet.rs`

**实现要点**：
- 滑入动画：translateY(100%) → translateY(0), duration: 300ms, ease-out
- 滑出动画：translateY(0) → translateY(100%), duration: 200ms, ease-in
- 遮罩淡入淡出
- 使用warpui的动画系统

---

### 任务 12：实现键盘快捷键

**目标**：支持键盘导航和快捷键

**文件**：
- 修改：`app/src/workspace/branch_selector/sheet.rs`

**实现要点**：
- 上下键：选择分支
- Enter：切换到选中分支
- ESC：关闭选择器或diff视图
- Cmd+Shift+B：打开分支选择器（全局快捷键）

---

### 任务 13：实现响应式布局

**目标**：适配不同屏幕尺寸

**文件**：
- 修改：`app/src/workspace/branch_selector/sheet.rs`

**实现要点**：
- 桌面（>1024px）：三栏布局，左20% + 中40% + 右40%
- 平板（768-1024px）：三栏布局，左25% + 中35% + 右40%
- 移动（<768px）：单栏布局，堆叠显示

---

### 任务 14：优化性能

**目标**：处理大型仓库的性能问题

**文件**：
- 修改：`app/src/workspace/branch_selector/branch_list.rs`
- 修改：`app/src/workspace/branch_selector/file_list.rs`

**实现要点**：
- 使用虚拟滚动（ClippedScrollable）处理大量分支/文件
- 缓存分支列表和文件列表
- 异步加载分支详情和diff内容
- 显示加载状态

---

### 任务 15：错误处理和用户反馈

**目标**：处理Git操作失败的情况

**文件**：
- 修改：`app/src/workspace/action.rs`
- 修改：`app/src/workspace/branch_selector/sheet.rs`

**实现要点**：
- Git操作失败时显示友好的错误提示
- 处理合并冲突的情况
- 处理权限不足的情况
- 提供重试按钮

---

## 注意事项
- 不需要编写测试用例，测试由后续的逻辑推演替代
- 所有Git操作需要异步执行，避免阻塞UI
- 需要处理并发操作（防止重复点击）
- 需要考虑多仓库场景（每个tab可能有不同的仓库）
