# Git分支选择器半弹框 - 改动简述

## 需求ID
OMJF-10001

## 实现状态
已完成

## 已完成任务

### 任务1：定义数据结构和状态 ✅
- 新增 `app/src/workspace/branch_selector/state.rs`
  - 定义 `BranchInfo` 结构体（分支名称、是否当前、是否远程、最后提交等）
  - 定义 `CommitInfo` 结构体（hash、消息、作者、时间）
  - 定义 `ChangedFile` 结构体（路径、状态、改动行数）
  - 定义 `DiffHunk`、`DiffLine`、`DiffLineType` 结构体
  - 定义 `FileDiff` 结构体
  - 定义 `FileStatus` 枚举（Modified、Added、Deleted、Renamed）
  - 定义 `ContextMenuItem` 枚举
  - 定义 `BranchSelectorState` 结构体，管理整个选择器的状态

### 任务2：添加Git操作方法 ✅
- 修改 `app/src/util/git.rs`
  - 新增 `list_all_branches` 方法：获取所有分支列表
  - 新增 `get_branch_last_commit` 方法：获取分支最后提交信息
  - 新增 `get_changed_files_between_branches` 方法：获取两个分支间的改动文件
  - 新增 `get_file_diff` 方法：获取文件的diff内容
  - 新增 `get_branch_ahead_behind` 方法：获取分支统计（领先/落后提交数）
  - 新增 `create_branch` 方法：创建新分支

### 任务3：添加WorkspaceAction ✅
- 修改 `app/src/workspace/action.rs`
  - 新增 `OpenBranchSelector` Action
  - 新增 `CloseBranchSelector` Action
  - 新增 `SelectBranch` Action
  - 新增 `OpenDiffViewer` Action
  - 新增 `CloseDiffViewer` Action
  - 新增 `OpenBranchContextMenu` Action
  - 新增 `CloseBranchContextMenu` Action
  - 新增 `SwitchToBranch` Action
  - 新增 `MergeBranch` Action
  - 新增 `CreateBranch` Action
  - 新增 `DeleteBranchAction` Action
  - 新增 `SetBranchSearchFilter` Action
  - 新增 `ToggleShowRemoteBranches` Action
  - 新增 `RefreshBranchList` Action
  - 更新 `should_save_app_state_on_action` 方法

### 任务4-9：实现UI组件 ✅
- 新增 `app/src/workspace/branch_selector/mod.rs`
  - 实现 `render_branch_selector` 函数
  - 实现三栏布局（分支列表 | 分支详情 | 文件列表）
  - 实现半弹框容器（使用 Stack + OffsetPositioning）
  - 实现背景遮罩
  - 实现分支列表渲染
  - 实现分支详情渲染
  - 实现文件列表渲染
  - 使用索引遍历和边界检查确保安全性

### 任务10：集成到Workspace ✅
- 修改 `app/src/workspace/mod.rs`
  - 新增 `branch_selector` 模块
  - 导出 `BranchInfo`、`BranchSelectorState`、`ChangedFile`、`CommitInfo`、`FileDiff`、`FileStatus`
- 修改 `app/src/workspace/view.rs`
  - 在 `on_action` 方法中处理所有新增的 Action
  - 在 `render` 方法中渲染分支选择器

## 文件改动清单

| 文件 | 操作 | 说明 |
|------|------|------|
| app/src/workspace/branch_selector/mod.rs | 新增 | 分支选择器主模块，包含UI渲染 |
| app/src/workspace/branch_selector/state.rs | 新增 | 状态管理，数据结构定义 |
| app/src/util/git.rs | 修改 | 新增Git操作方法 |
| app/src/workspace/action.rs | 修改 | 新增分支选择器相关Action |
| app/src/workspace/mod.rs | 修改 | 导入分支选择器模块 |
| app/src/workspace/view.rs | 修改 | 处理分支选择器Action，渲染UI |

## 推演结论

### 轮次：3

### 发现问题：2 个
- [critical] 问题1：迭代器遍历可能导致引用问题（已修复：使用索引遍历）
- [critical] 问题2：数组访问可能越界（已修复：使用 get() 方法边界检查）

### 检查项明细
- 主路径闭环：✓ 正常流程可正确执行
  - 打开分支选择器 → 显示分支列表 → 选择分支 → 显示改动文件 → 点击文件 → 打开diff视图
  - 关闭分支选择器 → 清理状态
- 异常处理：✓ 空数据判断完整
  - 无分支时显示空列表
  - 无选中分支时显示提示文字
  - 无改动文件时显示空列表
- 契约一致：✓ Action 数据结构一致
  - OpenBranchSelector 需要 pane_id
  - SelectBranch 需要 index
  - OpenDiffViewer 需要 file_index
- 边界条件：✓ 索引边界检查完整
  - 使用 `get()` 方法安全访问数组
  - 使用索引遍历替代迭代器
- 并发防重：✓ 异步操作使用 ctx.spawn
  - checkout_branch 异步执行
  - merge_branch 异步执行
  - create_branch 异步执行
  - delete_branch 异步执行
- 数据一致性：✓ 数据来源正确
  - 分支列表从 git 获取
  - 改动文件从 git diff 获取
  - diff 内容从 git diff 获取

### 技术要点
1. **安全访问**：使用 `get()` 方法替代直接索引访问，避免越界
2. **索引遍历**：使用 `for idx in 0..len` 替代迭代器，避免引用问题
3. **字符串克隆**：在需要 `'static` 生命周期时克隆字符串
4. **OffsetPositioning**：使用 `offset_from_parent` 实现半弹框从底部拉起效果
5. **异步操作**：使用 `ctx.spawn(future, callback)` 模式执行异步 git 操作

### 待优化项（非必需）
- [ ] 动画过渡效果（滑入/滑出）
- [ ] 键盘快捷键（上下键、Enter、ESC）
- [ ] 搜索过滤功能
- [ ] 右键菜单完整实现
- [ ] 虚拟滚动（大量分支/文件时）
- [ ] 点击交互（hover 效果和点击处理）
