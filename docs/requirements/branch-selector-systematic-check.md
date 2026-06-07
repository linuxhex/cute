# 分支选择器系统性检查报告

## 一、事件分发机制分析

### 1.1 Stack 事件分发模式

**代码位置**: `crates/warpui_core/src/elements/stack/mod.rs:288-316`

```rust
EventDispatchMode::Waterfall => {
    // For waterfall, we want to dispatch event to children in the reverse order (top first).
    for child in self.children.iter_mut().rev() {
        // We should not dispatch event to children that are not painted.
        if child.painted && child.element.dispatch_event(event, ctx, app) {
            return true;
        }
    }
}
```

**关键点**：
- Debug 模式下使用 `EventDispatchMode::Waterfall`
- Waterfall 模式按**逆序**分发事件（最后一个 child 先接收）
- 某个 child 处理事件后停止分发

### 1.2 右键菜单元素层级

```
Stack (Waterfall 模式)
  ├── [0] positioned_content (SavePosition 包裹的主内容)
  └── [1] positioned_overlay (Overlay 包裹的菜单) ← Waterfall 先分发到这里
       └── Overlay (start_overlay_layer)
            └── Positioned (OffsetPositioning)
                 └── EventHandler (菜单容器背景)
                      └── Container (菜单背景)
                           └── Flex (menu_column)
                                └── EventHandler (菜单项)
                                     └── Container (菜单项内容)
```

**事件分发顺序**（Waterfall 逆序）：
1. `positioned_overlay` (index 1) 先接收
2. `positioned_content` (index 0) 后接收

### 1.3 Overlay 层的 z-index

**代码位置**: `crates/warpui_core/src/elements/stack/overlay.rs:36-39`

```rust
fn paint(&mut self, origin: Vector2F, ctx: &mut PaintContext, app: &AppContext) {
    ctx.scene.start_overlay_layer(ClipBounds::None);  // 创建 overlay 层
    self.child.paint(origin, ctx, app);
    ctx.scene.stop_layer();
}
```

**关键点**：
- `start_overlay_layer` 创建一个新的 overlay 层
- Overlay 层的 z-index 是 `ZIndex::Overlay(index)`
- Overlay 层在普通层之上

## 二、Hit Test 机制

### 2.1 is_covered 检查

**代码位置**: `crates/warpui_core/src/event.rs:20-36`

```rust
pub fn at_z_index(&self, z_index: ZIndex, ctx: &EventContext) -> Option<&Event> {
    match self.event {
        Event::LeftMouseDown { position, .. } => {
            if !ctx.is_covered(Point::from_vec2f(position, z_index)) {
                Some(&self.event)
            } else {
                None  // 被覆盖，事件不传递
            }
        }
        // ...
    }
}
```

**is_covered 实现** (`scene.rs:416-436`):
```rust
pub fn is_covered(&self, position: Point) -> bool {
    // Does any layer at a higher z-index contain this point?
    let point = [position.x().into(), position.y().into()];
    let predicate = |l: &Layer| !l.click_through && l.hit_map.locate_at_point(&point).is_some();
    
    match position.z_index() {
        ZIndex::Normal(index) => self
            .layers.get((index + 1)..)
            .into_iter()
            .flatten()
            .chain(self.overlay_layers.iter())  // overlay 层总是检查
            .any(predicate),
        ZIndex::Overlay(index) => self
            .overlay_layers.get((index + 1)..)
            .into_iter()
            .flatten()
            .any(predicate),
    }
}
```

**关键点**：
- 检查更高 z-index 的层是否包含点击位置
- Overlay 层总是被检查（即使是 Normal z-index）
- 如果 `is_covered` 返回 true，事件不会传递

### 2.2 Container 的 hit rect 记录

**代码位置**: `container.rs:296-301`

```rust
let rect = ctx
    .scene
    .draw_rect_with_hit_recording(RectF::new(origin, size))  // 记录 hit rect
    .with_background(self.background)
    .with_border(self.border)
    .with_corner_radius(self.corner_radius);
```

**关键点**：
- Container 在 paint 时会记录 hit rect
- hit rect 用于 is_covered 检查

## 三、EventHandler 事件处理

### 3.1 dispatch_event 逻辑

**代码位置**: `event_handler.rs:241-361`

```rust
fn dispatch_event(&mut self, event: &DispatchedEvent, ctx: &mut EventContext, app: &AppContext) -> bool {
    let handled = self.child.dispatch_event(event, ctx, app);  // 先分发给子元素
    if handled && !self.always_handle {
        return true;  // 子元素处理了，且不是 always_handle，直接返回
    }
    
    // 检查事件是否应该被处理
    match event.at_z_index(z_index, ctx) {  // at_z_index 会做 is_covered 检查
        Some(Event::LeftMouseDown { position, .. }) => {
            if self.dispatch_callback(self.left_mouse_down.as_ref(), ctx, *position, app) {
                return true;
            }
        }
        // ...
    }
    handled
}
```

**关键点**：
1. 先分发给子元素
2. 如果子元素处理了且不是 `always_handle`，直接返回
3. 调用 `event.at_z_index` 检查是否应该处理（会做 is_covered 检查）
4. 如果 is_covered 返回 true，事件不会被处理

## 四、Hoverable 双击机制

### 4.1 dispatch_event 逻辑

**代码位置**: `hoverable.rs:558-692`

```rust
fn dispatch_event(&mut self, event: &DispatchedEvent, ctx: &mut EventContext, app: &AppContext) -> bool {
    let handled = self.child.dispatch_event(event, ctx, app);
    if self.disabled {
        return handled;
    }
    
    match event.raw_event() {
        Event::LeftMouseDown { click_count, position, .. } => {
            self.state().click_count = Some(*click_count);  // 保存 click_count
            
            // 如果有 double_click_handler 且 click_count == 2，标记为 handled
            if self.click_handler.is_some()
                || (*click_count == 2 && self.double_click_handler.is_some())
            {
                ctx.notify();
                return true;
            }
        }
        Event::LeftMouseUp { position, .. } => {
            let click_count = self.state().click_count.take();  // 取出 click_count
            
            if matches!(click_count, Some(2)) && self.double_click_handler.is_some() {
                // 触发双击回调
                handler(ctx, app, *position);
                return true;
            }
        }
    }
    handled
}
```

**关键点**：
1. LeftMouseDown 时保存 click_count
2. LeftMouseUp 时检查 click_count 是否为 2
3. 如果是 2 且有 double_click_handler，触发双击回调

## 五、问题排查

### 5.1 右键菜单点击无响应

**可能原因**：

1. **Hit rect 未正确记录**
   - 菜单项的 Container 可能没有正确的 hit rect
   - 导致 is_covered 检查失败

2. **z-index 问题**
   - 菜单项的 z-index 可能不正确
   - 导致 at_z_index 检查失败

3. **事件被父元素拦截**
   - 外层 EventHandler 可能拦截了事件

**已添加修复**：
- 在菜单容器外包裹 EventHandler，确保 hit rect 被记录

### 5.2 双击文件无响应

**可能原因**：

1. **EventHandler 拦截了事件**
   - 外层 EventHandler 使用了 `with_always_handle()`
   - 可能影响 Hoverable 的事件接收

2. **MouseStateHandle 问题**
   - click_count 可能未正确追踪

3. **is_mouse_over_element 检查失败**
   - Hoverable 在 LeftMouseUp 时会检查鼠标是否在元素内
   - 如果检查失败，不会触发双击

**需要验证**：
- Hoverable 的 is_mouse_over_element 是否返回 true
- click_count 是否正确传递

## 六、已添加的日志追踪

### 6.1 右键菜单
- `[ContextMenu] Rendering at position` - 菜单渲染位置
- `[ContextMenu] Adding SwitchToBranch item` - 菜单项添加
- `[ContextMenu] SwitchToBranch menu item clicked` - 菜单项点击
- `[ContextMenu] Background clicked` - 菜单背景点击

### 6.2 文件列表
- `[FileList] on_click triggered` - 文件单击
- `[FileList] on_double_click triggered` - 文件双击

### 6.3 Action 分发
- `[BranchSelectorAction] OpenDiffViewer action received` - OpenDiffViewer action

## 七、下一步行动

1. **运行程序**，触发以下场景：
   - 右键点击分支 → 查看是否有 `[ContextMenu] Rendering` 日志
   - 点击菜单项 → 查看是否有 `[ContextMenu] SwitchToBranch menu item clicked` 日志
   - 双击文件 → 查看是否有 `[FileList] on_double_click` 日志

2. **分析日志输出**，定位事件在哪个环节中断

3. **根据日志结果**，针对性修复：
   - 如果没有 Rendering 日志 → 检查菜单渲染逻辑
   - 如果有点击日志但没有 action 日志 → 检查 action 分发
   - 如果没有任何日志 → 检查事件是否到达 Stack
