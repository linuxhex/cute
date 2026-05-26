# Cute 编译说明

## 项目简介

Cute 是一个基于 Rust 的现代化终端模拟器，使用自定义 UI 框架 CuteUI。

## 环境要求

- **Rust**: 通过 `rustup` 安装（项目会自动处理）
- **平台**: macOS / Windows / Linux

## 编译命令

### 1. 首次设置（必须）

```bash
./script/bootstrap
```

这会安装：
- Rust 工具链
- 平台特定依赖
- 构建所需工具

### 2. 编译运行

```bash
# 方式一：使用脚本（推荐）
./script/run

# 方式二：直接使用 Cargo
cargo run
```

### 3. 仅编译（不运行）

```bash
# Debug 版本（编译快，体积大）
cargo build

# Release 版本（编译慢，性能好）
cargo build --release
```

### 4. 提交前检查

```bash
./script/presubmit
```

包含：
- `cargo fmt` - 代码格式化
- `cargo clippy` - 静态检查
- 测试运行

## 常用命令

| 命令 | 说明 |
|------|------|
| `cargo run` | 编译并运行 |
| `cargo build` | 仅编译 |
| `cargo build --release` | 发布版本编译 |
| `cargo test` | 运行测试 |
| `cargo fmt` | 代码格式化 |
| `cargo clippy` | 静态检查 |
| `cargo clean` | 清理构建产物 |

## 项目结构

```
├── app/              # 主程序入口
├── crates/           # 60+ 子 crate
│   ├── cuteui/       # UI 框架
│   ├── cute_core/    # 核心工具
│   ├── cute_terminal/# 终端模拟
│   ├── cute_completer/# 命令补全
│   ├── editor/       # 文本编辑
│   └── ...
├── script/           # 构建脚本
├── resources/        # 资源文件
└── Cargo.toml        # 工作区配置
```

## 主要 Crates

| Crate | 说明 |
|-------|------|
| `cute_core` | 核心工具和平台抽象 |
| `cute_terminal` | 终端模拟逻辑 |
| `cute_completer` | 命令补全引擎 |
| `cuteui` | 自定义 UI 框架 |
| `cuteui_core` | UI 框架核心 |
| `cute_assets` | 资源管理 |
| `cute_logging` | 日志系统 |

## 编译配置

项目定义了多个编译 profile：

- `dev` - 开发版本（默认）
- `release` - 发布版本
- `release-lto` - 启用 LTO 优化
- `release-cli` - CLI 专用，体积更小

使用方式：
```bash
cargo build --profile release-lto
```

## 常见问题

### 编译内存不足
Release 编译可能需要大量内存。可以：
```bash
# 使用 Debug 版本开发
cargo build

# 或限制并行编译数
CARGO_BUILD_JOBS=4 cargo build --release
```

### 依赖下载慢
可以配置国内镜像源，编辑 `~/.cargo/config`：
```toml
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
```

## 从 Warp 迁移

本项目从 Warp 重命名为 Cute，主要变更：
- 所有 `warp_*` crates 重命名为 `cute_*`
- `warpui*` 重命名为 `cuteui*`
- 品牌名称从 Warp 改为 Cute
- 配置目录从 `.warp` 改为 `.cute`
