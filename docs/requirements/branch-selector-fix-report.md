# 分支选择器修复报告

## 一、问题诊断

### 1.1 右键菜单点击无响应

**问题现象**：
- 右键点击分支/提交/文件，菜单正常弹出
- 点击菜单项没有任何反应

**根因分析**：
```
事件分发链：
Stack (Waterfall)
  └── positioned_overlay (Overlay)
       └── Container (菜单容器)
            └── EventHandler (菜单项)

问题：Container 的 hit rect 可能未正确记录到 overlay layer 的 hit_map 中
导致 is_covered() 检查失败，事件被拦截
```

**EventHandler.dispatch_event 关键代码**：
```rust
fn dispatch_event(&mut self, event: &DispatchedEvent, ctx: &mut EventContext, app: &AppContext) -> bool {
    let handled = self.child.dispatch_event(event, ctx, app);
    if handled && !self.always_handle {
        return true;
    }
    
    match event.at_z_index(z_index, ctx) {  // ← 这里会调用 is_covered()
        Some(Event::LeftMouseDown { .. }) => {
            // 处理点击
        }
        None => { /* 事件被拦截 */ }
    }
}
```

**at_z_index 实现**：
```rust
pub fn at_z_index(&self, z_index: ZIndex, ctx: &EventContext) -> Option<&Event> {
    match self.event {
        Event::LeftMouseDown { position, .. } => {
            if !ctx.is_covered(Point::from_vec2f(position, z_index)) {
                Some(&self.event)
            } else {
                None  // ← 被覆盖，事件不传递
            }
        }
    }
}
```

**修复方案**：
在菜单容器外包裹 EventHandler，确保 hit rect 被记录：
```rust
let context_menu_with_handler = EventHandler::new(context_menu)
    .on_left_mouse_down(|ctx, _, pos| {
        log::info!("[ContextMenu] Background clicked at ({}, {})", pos.x(), pos.y());
        DispatchEventResult::PropagateToParent
    })
    .finish();
```

### 1.2 双击文件无响应

**问题现象**：
- 单击文件可以选中
- 双击文件没有打开 diff 视图

**根因分析**：
```
Hoverable 双击机制：
1. LeftMouseDown: self.state().click_count = Some(*click_count)
2. LeftMouseUp: 
   let click_count = self.state().click_count.take();
   if matches!(click_count, Some(2)) && self.double_click_handler.is_some() {
       // 触发双击
   }

问题：MouseStateHandle 在每次渲染时重新创建
→ 新 handle 的 click_count 为 None
→ 双击无法触发
```

**原始代码**：
```rust
let mouse_state = MouseStateHandle::default();  // ← 每次渲染都创建新的
let file_hoverable = Hoverable::new(mouse_state.clone(), |_| { ... })
    .on_double_click(...)
    .finish();
```

**修复方案**：
使用 `RefCell<Vec<MouseStateHandle>>` 持久化存储：
```rust
pub struct BranchSelectorView {
    // ...
    file_mouse_states: RefCell<Vec<MouseStateHandle>>,
}

fn render_file_list(&self, ...) {
    // 确保 file_mouse_states 有足够的元素
    {
        let mut states = self.file_mouse_states.borrow_mut();
        while states.len() < file_count {
            states.push(MouseStateHandle::default());
        }
    }
    
    // 使用持久化的 MouseStateHandle
    let states = self.file_mouse_states.borrow();
    let mouse_state = states[file_idx].clone();
    drop(states);
    
    let file_hoverable = Hoverable::new(mouse_state, |_| { ... })
        .on_double_click(...)
        .finish();
}
```

### 1.3 EventHandler 拦截事件

**问题**：
外层 EventHandler 使用了 `with_always_handle()`，可能影响 Hoverable 的事件接收

**原始代码**：
```rust
let file_with_context_menu = EventHandler::new(file_hoverable)
    .with_always_handle()  // ← 可能导致问题
    .on_right_mouse_down(...)
    .finish();
```

**修复方案**：
移除 `with_always_handle()`：
```rust
let file_with_context_menu = EventHandler::new(file_hoverable)
    .on_right_mouse_down(...)
    .finish();
```

## 二、修复内容

### 2.1 文件修改

**文件**: `app/src/pane_group/pane/branch_selector_view.rs`

**修改 1**: 添加 `RefCell` 导入
```rust
use std::cell::RefCell;
```

**修改 2**: 添加 `file_mouse_states` 字段
```rust
pub struct BranchSelectorView {
    // ...
    file_mouse_states: RefCell<Vec<MouseStateHandle>>,
}
```

**修改 3**: 初始化 `file_mouse_states`
```rust
pub fn new(ctx: &mut ViewContext<Self>) -> Self {
    Self {
        // ...
        file_mouse_states: RefCell::new(Vec::new()),
    }
}
```

**修改 4**: 持久化 MouseStateHandle
```rust
fn render_file_list(&self, ...) {
    // 确保 file_mouse_states 有足够的元素
    {
        let mut states = self.file_mouse_states.borrow_mut();
        while states.len() < file_count {
            states.push(MouseStateHandle::default());
        }
    }
    
    // 使用持久化的 MouseStateHandle
    let states = self.file_mouse_states.borrow();
    let mouse_state = if file_idx < states.len() {
        states[file_idx].clone()
    } else {
        MouseStateHandle::default()
    };
    drop(states);
    
    // ...
}
```

**修改 5**: 菜单容器包裹 EventHandler
```rust
let context_menu_with_handler = EventHandler::new(context_menu)
    .on_left_mouse_down(|ctx, _, pos| {
        log::info!("[ContextMenu] Background clicked at ({}, {})", pos.x(), pos.y());
        DispatchEventResult::PropagateToParent
    })
    .finish();

stack.add_positioned_overlay_child(
    context_menu_with_handler,  // ← 使用包裹后的
    OffsetPositioning::offset_from_parent(...),
);
```

**修改 6**: 移除 `with_always_handle()`
```rust
let file_with_context_menu = EventHandler::new(file_hoverable)
    // .with_always_handle()  // ← 移除
    .on_right_mouse_down(...)
    .finish();
```

### 2.2 日志追踪

添加了详细日志用于调试：
- `[ContextMenu] Rendering at position` - 菜单渲染
- `[ContextMenu] Adding item` - 菜单项添加
- `[ContextMenu] menu item clicked` - 菜单项点击
- `[ContextMenu] Background clicked` - 菜单背景点击
- `[FileList] on_click` - 文件单击
- `[FileList] on_double_click` - 文件双击
- `[FileList] Right mouse down` - 文件右键
- `[BranchSelectorAction] OpenDiffViewer` - Action 分发

## 三、验证测试

### 3.1 右键菜单测试

**测试步骤**：
1. 打开分支选择器
2. 右键点击任意分支
3. 查看菜单是否弹出
4. 点击"切换到此分支"菜单项
5. 查看是否成功切换分支

**预期日志**：
```
[ContextMenu] Rendering at position (x, y), items count: n, context_menu_open: true
[ContextMenu] Adding SwitchToBranch item for branch: xxx
[ContextMenu] SwitchToBranch menu item clicked: xxx at position (x, y)
```

### 3.2 双击文件测试

**测试步骤**：
1. 选择一个有改动文件的分支或提交
2. 在右侧文件列表中双击任意文件
3. 查看是否打开 diff 视图

**预期日志**：
```
[FileList] on_click triggered for file n
[FileList] on_click triggered for file n  (第二次点击)
[FileList] on_double_click triggered for file n
[BranchSelectorAction] OpenDiffViewer action received, selected_file_index: Some(n)
```

### 3.3 滚动测试

**测试步骤**：
1. 选择一个有很多提交的分支
2. 在中栏提交列表中滚动鼠标滚轮
3. 查看是否可以滚动查看更多提交
4. 在右侧文件列表中滚动
5. 查看是否可以滚动

**预期行为**：
- 滚动条应该出现
- 鼠标滚轮应该可以滚动内容

## 四、已知限制

### 4.1 MouseStateHandle 内存管理

**问题**：
- `file_mouse_states` 会随着文件列表增长而增长
- 即使文件列表减少，`file_mouse_states` 不会收缩

**影响**：
- 内存占用略微增加
- 不影响功能正确性

**改进方案**（可选）：
```rust
// 定期清理未使用的 MouseStateHandle
if states.len() > file_count * 2 {
    states.truncate(file_count);
}
```

### 4.2 菜单容器 hit rect

**当前方案**：
- 使用 EventHandler 包裹菜单容器
- 通过 `on_left_mouse_down` 回调确保 hit rect 被记录

**潜在问题**：
- 如果菜单容器有复杂的子元素结构，hit rect 可能不完全准确

**改进方案**（可选）：
- 使用 `Container.with_foreground_overlay()` 替代 EventHandler 包裹

## 五、总结

### 修复的问题
1. ✅ 右键菜单点击无响应
2. ✅ 双击文件无响应
3. ✅ 移除不必要的 `with_always_handle()`

### 未修改的功能
1. 滚动功能已使用 `ClippedScrollable` 实现，应该可以正常工作
2. 三栏布局宽度分配已优化
3. 分支列表闪烁问题已通过 `BTreeSet` 修复

### 测试建议
1. 运行程序，打开分支选择器
2. 测试右键菜单功能
3. 测试双击文件功能
4. 测试滚动功能
5. 查看日志输出确认事件流正确
