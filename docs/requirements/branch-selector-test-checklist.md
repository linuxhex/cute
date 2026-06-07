# 分支选择器测试验证清单

## 编译状态
✅ 编译成功，无新增警告

## 修复内容回顾

### 1. 右键菜单点击修复
- **问题**：点击菜单项无响应
- **根因**：菜单容器 hit rect 未正确记录，is_covered 检查失败
- **修复**：在菜单容器外包裹 EventHandler

### 2. 双击文件修复
- **问题**：双击文件无响应
- **根因**：MouseStateHandle 每次渲染重新创建，click_count 丢失
- **修复**：使用 RefCell<Vec<MouseStateHandle>> 持久化存储

### 3. 事件拦截修复
- **问题**：with_always_handle() 可能影响 Hoverable
- **修复**：移除 with_always_handle()

## 测试验证步骤

### 测试 1：右键菜单功能

**步骤**：
1. 启动程序
2. 打开 Git 分支选择器（通过命令面板或快捷键）
3. 右键点击任意分支
4. 验证菜单是否弹出
5. 点击"切换到此分支"
6. 验证分支是否切换成功

**预期日志**：
```
[ContextMenu] Rendering at position (x, y), items count: n, context_menu_open: true
[ContextMenu] Adding SwitchToBranch item for branch: xxx
[ContextMenu] SwitchToBranch menu item clicked: xxx at position (x, y)
```

**其他菜单项测试**：
- [ ] 合并到当前分支
- [ ] 删除分支
- [ ] 查看提交详情
- [ ] 复制提交哈希
- [ ] 查看 Diff

### 测试 2：双击文件功能

**步骤**：
1. 选择一个有改动文件的分支
2. 在右侧文件列表中单击文件（验证选中）
3. 双击同一文件
4. 验证是否打开 diff 视图

**预期日志**：
```
[FileList] on_click triggered for file 0
[FileList] on_click triggered for file 0  (第二次点击，click_count=2)
[FileList] on_double_click triggered for file 0
[BranchSelectorAction] OpenDiffViewer action received, selected_file_index: Some(0)
```

### 测试 3：滚动功能

**步骤**：
1. 选择一个有很多提交的分支（如 main/master）
2. 在左栏分支列表中滚动鼠标滚轮
3. 验证是否可以滚动
4. 在中栏提交列表中滚动
5. 验证是否可以滚动
6. 在右栏文件列表中滚动
7. 验证是否可以滚动

**预期行为**：
- 滚动条应该出现
- 鼠标滚轮应该可以滚动内容
- 滚动应该平滑

### 测试 4：三栏布局

**步骤**：
1. 打开分支选择器
2. 观察三栏宽度比例
3. 验证左栏（分支列表）宽度约 280px
4. 验证中栏（提交历史）宽度约 400px
5. 验证右栏（文件列表）宽度约 280px

### 测试 5：分支列表闪烁

**步骤**：
1. 打开分支选择器
2. 观察远程分支列表
3. 验证列表是否稳定，无闪烁

**预期**：使用 BTreeSet 后，分支顺序稳定，无闪烁

## 日志查看方式

启动程序时设置日志级别：
```bash
RUST_LOG=info cargo run
```

或运行已编译的程序：
```bash
RUST_LOG=info ./target/debug/warp
```

## 已知限制

1. **MouseStateHandle 内存**：file_mouse_states 会随文件列表增长，但不影响功能
2. **菜单容器 hit rect**：使用 EventHandler 包裹，可能不是最优方案

## 测试结果记录

测试日期：____________________

| 测试项 | 结果 | 备注 |
|--------|------|------|
| 右键菜单 - 切换分支 | ⬜ | |
| 右键菜单 - 合并分支 | ⬜ | |
| 右键菜单 - 删除分支 | ⬜ | |
| 双击文件打开 diff | ⬜ | |
| 左栏滚动 | ⬜ | |
| 中栏滚动 | ⬜ | |
| 右栏滚动 | ⬜ | |
| 三栏布局 | ⬜ | |
| 分支列表无闪烁 | ⬜ | |

## 问题反馈

如果测试发现问题，请记录：
1. 具体操作步骤
2. 预期行为
3. 实际行为
4. 相关日志输出
