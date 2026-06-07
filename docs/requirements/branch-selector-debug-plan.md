# 分支选择器功能排查计划

## 问题概述

根据之前的调试，存在以下问题：
1. 右键菜单项点击无反应
2. 双击文件不打开 diff 视图
3. 滚动功能未实现

## 事件分发机制分析

### Stack 事件分发
- Debug 模式下使用 `EventDispatchMode::Waterfall`
- Waterfall 模式按**逆序**分发事件（最后一个 child 先接收）
- 某个 child 处理事件后停止分发

### 右键菜单结构
```
Stack
  ├── positioned_content (SavePosition 包裹的主内容)
  └── positioned_overlay (Overlay 包裹的菜单)
       └── Container (菜单容器)
            └── Flex (菜单列)
                 └── EventHandler (菜单项)
```

### 文件列表结构
```
ClippedScrollable
  └── Flex
       └── EventHandler (处理右键)
            └── Hoverable (处理双击)
                 └── Container (文件项)
```

## 排查任务

### 任务 1：排查右键菜单点击问题

**检查点**：
1. 菜单是否正确渲染（通过日志确认 paint 被调用）
2. 菜单项的 bounds 是否正确（hit test 能否命中）
3. 事件是否到达菜单项的 EventHandler
4. EventHandler 的回调是否被调用

**可能原因**：
- 菜单的 bounds 计算错误，hit test 失败
- 事件坐标与菜单位置不匹配
- z-index 问题导致事件无法到达

### 任务 2：排查双击文件问题

**检查点**：
1. Hoverable 的 on_double_click 是否正确注册
2. MouseStateHandle 是否正确追踪点击计数
3. LeftMouseUp 事件是否到达 Hoverable
4. 外层 EventHandler 是否拦截了事件

**可能原因**：
- EventHandler 的 `with_always_handle()` 可能影响事件传递
- Hoverable 的 click_count 未正确更新
- 事件被父元素拦截

### 任务 3：实现滚动功能

**当前状态**：
- 已使用 ClippedScrollable 包裹三栏
- 滚动状态已在 state 中定义

**需要检查**：
- ClippedScrollable 是否正确工作
- 滚动条是否可见
- 鼠标滚轮事件是否被正确处理

## 执行顺序

1. 先添加详细日志追踪事件流
2. 运行程序，触发问题场景
3. 分析日志输出，定位问题
4. 逐个修复问题
5. 验证修复效果
