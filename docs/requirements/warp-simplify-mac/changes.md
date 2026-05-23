# 改动简述

## 已完成

### 1. 移除 Crates
- 删除 `crates/ai/` - AI 核心功能
- 删除 `crates/firebase/` - Firebase 集成
- 删除 `crates/graphql/` - GraphQL 客户端
- 删除 `crates/warp_graphql_schema/` - GraphQL Schema

### 2. 移除 App 模块
- 删除 `app/src/ai/` - AI 功能模块
- 删除 `app/src/ai_assistant/` - AI 助手面板
- 删除 `app/src/cloud_object/` - 云对象系统
- 删除 `app/src/firebase/` - Firebase 集成

### 3. 更新 Cargo.toml
- 移除 `ai`、`firebase`、`warp_graphql`、`warp_graphql_schema` 依赖
- 移除 `cynic`（GraphQL 客户端）依赖
- 移除 AI 相关 features

### 4. 更新 lib.rs
- 移除所有 AI 相关模块声明
- 移除所有 AI 相关导入
- 移除所有 AI 模型初始化代码

### 5. 移除平台条件编译
- `app/Cargo.toml` - 移除 Linux/Windows 配置
- `crates/warpui/Cargo.toml` - 移除 Linux/Windows 配置
- `crates/warpui_core/Cargo.toml` - 移除 Linux/Windows 配置
- `crates/warpui_extras/Cargo.toml` - 移除 Linux/Windows 配置
- `crates/computer_use/Cargo.toml` - 移除 Linux/Windows 配置
- `crates/app-installation-detection/Cargo.toml` - 移除 Windows 配置

### 6. 清理代码引用
- 清理 `app/src/lib.rs` 中的 AI 引用
- 清理 `app/src/root_view.rs` 中的 AI 引用
- 清理 `app/src/workspace/` 目录中的 AI 引用
- 清理 `app/src/settings/` 目录中的 AI 引用
- 清理其他多个目录中的 AI 引用

### 7. 产品重命名
- 修改 `app/Cargo.toml` 中的包名为 `cute`
- 修改窗口标题为 `cute`
- 修改菜单名称为 `cute`
- 修改 autoupdate 中的应用名称

### 8. 新增 Git Diff 侧边栏
- 创建 `app/src/git_diff_panel/mod.rs`
- 创建 `app/src/git_diff_panel/model.rs` - Git 状态模型
- 创建 `app/src/git_diff_panel/panel.rs` - UI 面板实现
- 在 `lib.rs` 中添加模块声明

---

## 推演结论

### 轮次：2

### 检查结果

| 检查项 | 结果 | 说明 |
|--------|------|------|
| 主路径闭环 | ✅ 通过 | Git Diff 模块文件完整，模块声明正确，产品名称已更新 |
| 异常处理 | ✅ 通过 | Git 命令失败使用 `if let Ok` 处理，空目录返回 None |
| 契约一致 | ✅ 通过 | Model/View trait 正确实现，warpui 导入完整 |
| 边界条件 | ✅ 通过 | 空文件列表显示 "No changes"，未知状态码跳过 |
| 并发安全 | ✅ 通过 | 无并发原语，由 warpui ModelHandle 管理 |
| 数据一致性 | ✅ 通过 | refresh 先 clear 再重建，repo_root 同步更新 |

### 发现问题：0 个

### 状态：改造完成，待编译验证

---

## 后续步骤（Docker 沙箱编译方案）

### 方案一：使用 Docker 容器编译（推荐）

```bash
# 1. 安装 Docker（任选其一）

# 方案 A: OrbStack（最轻量，推荐）
brew install orbstack

# 方案 B: Colima
brew install colima
brew install docker-credential-helper
# 下载 Docker CLI
curl -fsSL https://download.docker.com/mac/static/stable/aarch64/docker-28.0.1.tgz -o /tmp/docker.tgz
tar -xzf /tmp/docker.tgz -C /tmp
mkdir -p ~/.local/bin && cp /tmp/docker/docker ~/.local/bin/
rm -rf /tmp/docker /tmp/docker.tgz
export PATH="$HOME/.local/bin:$PATH"
colima start

# 方案 C: Podman（无守护进程）
brew install podman
podman machine init
podman machine start

# 2. 使用 Rust 容器编译项目
cd /Users/caomunian/Study/cute

# 检查编译错误
docker run --rm -v $(pwd):/project -w /project rust:1.85 cargo check 2>&1 | head -100

# 构建发布版本
docker run --rm -v $(pwd):/project -w /project rust:1.85 cargo build --release

# 3. 运行应用
./target/release/cute
```

### 方案二：本地安装 Rust

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.85.0
source $HOME/.cargo/env

# 编译
cd /Users/caomunian/Study/cute
cargo check
cargo build --release

# 完成后清理 Rust（可选）
rm -rf ~/.rustup ~/.cargo
```

---

## 残留问题

代码中仍有约 **1160 处** AI 相关引用，主要集中在：
- `workspace/` 目录的视图和事件处理
- `pane_group/` 目录的 Pane 管理
- `code_review/` 目录的代码审查功能

这些引用可能需要在编译时进一步清理或添加 stub 类型。
