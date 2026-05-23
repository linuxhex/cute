# 需求分析

## 需求背景

用户希望基于 Warp 终端代码改造一个精简版终端，名为 `cute`。主要目标是移除不需要的功能，专注于 Mac 平台，并新增 Git Diff 侧边栏功能。

## 改造目标

### 1. 只保留 Mac 运行

**移除内容**：
- Windows 平台支持（cfg(target_os = "windows")）
- Linux 平台支持（cfg(target_os = "linux")、cfg(target_os = "freebsd")）
- 相关平台特定代码和依赖

**涉及文件**：
- `app/Cargo.toml` - 平台条件编译配置
- `crates/*/Cargo.toml` - 各 crate 的平台配置
- `app/src/` 中的平台特定代码

### 2. 移除协作云同步

**移除内容**：
- Firebase 集成（`crates/firebase/`、`app/src/firebase/`）
- 云对象系统（`app/src/cloud_object/`）
- Warp Drive 云同步功能
- GraphQL 云 API（`crates/graphql/`、`crates/warp_graphql_schema/`）
- 远程服务器相关（`crates/remote_server/`）

**保留内容**：
- 本地文件系统功能
- 本地配置存储

### 3. 移除集成 AI

**移除内容**：
- AI 助手面板（`app/src/ai_assistant/`）
- AI 核心功能（`app/src/ai/`、`crates/ai/`）
- AI 相关 features
- Agent 模式

**保留内容**：
- 终端 CLI 功能
- 命令补全（本地）

### 4. 新增 Git Diff 侧边栏

**功能描述**：
- 在右侧添加类似左侧目录树的侧边栏
- 显示 git 变动文件列表
- 支持收起/展开
- 点击文件可查看 diff

**实现思路**：
- 参考 `app/src/drive/panel.rs` 的侧边栏实现
- 新建 `app/src/git_diff_panel/` 模块
- 集成到 `app/src/workspace/view.rs`

### 5. 性能优化

**优化方向**：
- 减少编译依赖，缩短编译时间
- 移除不必要的后台服务
- 优化启动速度
- 减少内存占用

### 6. 窗口管理优化

**功能描述**：
- 已打开窗口上方显示 + 号
- 点击 + 号新开窗口指向当前目录
- 侧边栏不增加目录项

**实现思路**：
- 修改 `app/src/pane_group/pane/view/header/` 头部组件
- 添加新窗口按钮
- 实现目录继承逻辑

## 产品重命名

将所有 "Warp" 命名改为 "cute"：
- 窗口标题
- 应用名称
- 配置目录

## 技术栈

- **语言**：Rust
- **UI 框架**：warpui（自研）
- **平台**：macOS only
- **构建系统**：Cargo workspace

## 风险点

1. **依赖耦合**：AI 和云同步功能可能与核心功能有耦合，需要仔细解耦
2. **编译配置**：大量 features 和 cfg 需要系统清理
3. **UI 布局**：新增侧边栏需要调整现有布局系统
4. **性能测量**：需要建立性能基准来验证优化效果

## 成功标准

1. 编译成功，运行稳定
2. 只支持 macOS，无其他平台代码残留
3. 无 AI、云同步相关功能入口
4. Git Diff 侧边栏功能正常
5. 性能优于原版（启动时间、内存占用）
6. 窗口 + 号功能正常
