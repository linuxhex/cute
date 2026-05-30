# 恢复终端界面实现计划

**目标：** 创建简化的终端界面，基于 cuteui 框架，不依赖 AI/云同步功能。

**架构：** 使用 cuteui 的 Flex/Stack 布局组件构建终端视图。

**技术栈：** Rust + cuteui UI 框架 + cute_terminal 终端模拟

---

## 实现状态

### 已完成 ✅

1. **终端视图框架**
   - 创建 `app/src/terminal/view/mod.rs`
   - 使用 Flex 垂直布局（标题栏 + 内容区 + 状态栏）
   - 使用 Stack 叠加背景和布局
   - 深色主题配色（#1e1e1e 背景，#2d2d2d 标题栏）

2. **终端模型**
   - 输入缓冲区管理
   - 命令历史（VecDeque，最多 1000 行）
   - 内置命令处理（help, clear, pwd, ls, cd, exit）
   - 外部命令通过 shell 执行
   - 光标位置管理

3. **左侧侧边栏**
   - 文件浏览器视图 (`app/src/file_explorer.rs`)
   - 真实文件系统访问
   - 文件/目录列表显示（带图标）
   - 选择和导航事件
   - 目录切换

4. **右侧 diff 面板**
   - 可收起的侧边栏 (`app/src/diff_panel.rs`)
   - Git diff 内容显示
   - diff 内容结构（hunk、line）
   - hunk 导航
   - 面板宽度可调整

5. **头部信息栏**
   - 当前目录显示（动态）
   - Git 分支显示（动态）

6. **底部状态栏**
   - 命令执行状态（OK/ERR）
   - 执行时间显示

7. **Git 集成**
   - Git 状态检测 (`app/src/git.rs`)
   - 当前分支获取
   - diff 内容获取和解析
   - ahead/behind 计数
   - 文件状态统计

8. **macOS 平台支持**
   - 平台回调存根 (`app/src/terminal/view/mac.rs`)
   - 所有 `warp_*` 函数实现

9. **应用启动**
   - 简化 `app/src/lib.rs`
   - 移除对 AI/auth/drive 等模块的依赖
   - 应用可以编译并运行

### 待实现

1. **PTY 集成**
   - 集成 cute_terminal 的终端模拟
   - 实现真实的 shell 进程
   - 实现键盘输入转发

2. **键盘事件处理**
   - 输入字符处理
   - 快捷键绑定
   - 命令历史导航

3. **鼠标事件处理**
   - 文件选择
   - 滚动
   - 拖拽调整面板大小

---

## 技术决策

### 为什么不恢复原始 Warp 代码

原始 Warp 终端视图代码（`view.rs` 27,635 行）有大量依赖：
- `ai` 模块（AI 功能）
- `auth` 模块（认证功能）
- `drive` 模块（云盘功能）
- `server` 模块（服务端）
- `onboarding` 模块
- 等 50+ 个模块

恢复这些代码需要：
1. 恢复所有依赖模块
2. 移除 AI/云相关代码（大量修改）
3. 修复编译错误（预估 4-8 小时）

### 采用的方案

创建简化的终端视图：
- 直接使用 cuteui 组件构建界面
- 按需添加功能，不引入不必要的依赖
- 保持代码简洁，易于维护

---

## 文件结构

```
app/src/
├── terminal/
│   ├── mod.rs              # 模块声明
│   └── view/
│       ├── mod.rs          # 终端视图
│       └── mac.rs          # macOS 平台回调
├── file_explorer.rs        # 文件浏览器侧边栏
├── diff_panel.rs           # Diff 面板
├── git.rs                  # Git 集成
├── settings/
│   └── mod.rs              # 设置模块存根
└── lib.rs                  # 应用入口
```

---

## 下一步

1. 集成 PTY 实现真实终端输入输出
2. 添加键盘事件处理
3. 添加鼠标事件处理
