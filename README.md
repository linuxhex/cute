<p align="center">
  <img width="128" alt="Cute Terminal" src="https://raw.githubusercontent.com/linuxhex/cute/main/app/assets/cute.png" />
</p>

<h1 align="center">Cute</h1>

<p align="center">
  一款面向开发者的现代终端模拟器
</p>

## 简介

Cute 是一款基于 Rust 构建的 GPU 加速终端模拟器，提供流畅的终端体验和丰富的开发者工具。

## 功能特性

### 终端

- **GPU 加速渲染** — 基于自研 wgpu 渲染引擎，滚动流畅、响应迅速
- **分屏支持** — 支持水平和垂直分屏，灵活组织工作区
- **多标签页** — 现代化的标签页管理，轻松切换多个终端会话
- **主题定制** — 自由配置颜色、字体和外观
- **连字渲染** — 支持编程连字，代码显示更美观

### AI 智能助手

- **内置编码助手** — 在终端中直接与 AI 交互
- **多 Agent 支持** — 支持接入 Claude Code、Qoder、Trae 等 CLI 工具
- **上下文感知** — AI 可理解终端输出、文件系统和项目上下文
- **Agent 模式** — 完整的 AI 对话体验，支持文件编辑、命令执行等

### 命令补全

- **智能补全** — 上下文感知的 Shell 命令补全建议
- **路径补全** — 智能文件和目录路径补全
- **历史搜索** — 模糊搜索命令历史记录

### 编辑器

- **内置编辑器** — 在终端中直接编辑文件
- **语法高亮** — 支持多种编程语言
- **LSP 集成** — 语言服务器协议支持，提供智能代码编辑

## 安装

### 下载安装包

前往 [Releases](https://github.com/linuxhex/cute/releases) 页面下载最新版本。

### 从源码编译

```bash
# 环境准备（首次运行）
./script/bootstrap

# 编译并运行（推荐）
./script/run

# 开发检查
./script/presubmit   # 格式化、Clippy 和测试
```

**注意：**
- 推荐使用 `./script/run` 运行，它会构建完整的 `.app` 包并自动打开
- 直接使用 `cargo run --bin cute` 可能因缺少打包资源而无法正常运行

## 许可证

Cute 的 UI 框架基于 [MIT 许可证](LICENSE-MIT)。

其余代码基于 [AGPL v3](LICENSE-AGPL) 许可证。

## 开源致谢

Cute 基于以下优秀的开源项目构建：

- [Tokio](https://github.com/tokio-rs/tokio)
- [NuShell](https://github.com/nushell/nushell)
- [Fig Completion Specs](https://github.com/withfig/autocomplete)
- [Alacritty](https://github.com/alacritty/alacritty)
- [Hyper HTTP library](https://github.com/hyperium/hyper)
- [FontKit](https://github.com/servo/font-kit)
- [Core-foundation](https://github.com/servo/core-foundation-rs)
- [Smol](https://github.com/smol-rs/smol)