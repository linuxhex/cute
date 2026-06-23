# Cute 应用构建和运行脚本

## 📋 概述

本项目**仅支持 Cute 应用**,不再支持 Warp 相关的构建和运行。

## 🚀 运行应用

### 开发模式运行

```bash
# 默认运行 Cute 应用
./script/run

# 带参数运行
./script/run -- --no-gpu

# Release 模式
./script/run --release

# 指定 profile
./script/run --profile release-lto
```

### 说明

- ✅ 默认编译并运行 `cute` 应用
- ✅ 使用 `oss` 渠道配置
- ✅ macOS 上会生成 `.app` 包并自动打开
- ❌ 不再支持 `warp` 或 `warp-local`

## 📦 打包应用

### 创建 DMG 安装包

```bash
# 默认打包 (Release 模式, Universal Binary)
./script/macos/bundle

# Debug 模式
./script/macos/bundle --debug

# 跳过构建(使用已有二进制)
./script/macos/bundle --skip-build

# 打包后打开
./script/macos/bundle -o

# 自签名
./script/macos/bundle --selfsign

# 指定架构
./script/macos/bundle --arch aarch64
./script/macos/bundle --arch x86_64
```

### 输出位置

打包完成后,DMG 文件位于:
```
target/{arch}/{profile}/bundle/osx/dmg/cute/Cute.dmg
```

例如:
- `target/aarch64-apple-darwin/release-lto/bundle/osx/dmg/cute/Cute.dmg`
- `target/x86_64-apple-darwin/release-lto/bundle/osx/dmg/cute/Cute.dmg`

## 🎨 图标处理

### 生成所有渠道图标

```bash
# 从 images/logoimg.png 生成所有渠道图标
python3 scripts/generate_icons.py

# 去除 cute 渠道图标白边
python3 scripts/remove_icon_border.py

# 批量处理所有渠道图标
python3 scripts/process_all_icons.py
```

### 图标源文件

- **源图标**: `images/logoimg.png` (720x712, 带适当白边)
- **输出位置**: `app/channels/{channel}/icon/no-padding/`
- **支持尺寸**: 16x16, 32x32, 48x48, 64x64, 128x128, 256x256, 512x512

## ⚙️ 配置说明

### Cargo.toml 配置

Cute 应用的打包配置在 `app/Cargo.toml`:

```toml
[package.metadata.bundle.bin.cute]
category = "public.app-category.developer-tools"
copyright = "© 2025, Denver Technologies, Inc"
identifier = "dev.cute.Cute"
name = "Cute"
icon = ["channels/cute/icon/no-padding/512x512.png", "channels/cute/icon/no-padding/icon.ico"]
```

### 环境变量

运行脚本使用的环境变量:

- `WARP_BIN_NAME` - 固定为 `cute`
- `WARP_CHANNEL` - 固定为 `oss`
- `FEATURES` - cargo 特性,默认为 `gui`

## 📝 重要说明

1. **仅支持 Cute**: 所有脚本已移除对 Warp 的支持
2. **图标处理**: 使用 `images/logoimg.png` 作为源文件
3. **打包输出**: DMG 名称固定为 `Cute.dmg`
4. **应用标识**: `dev.cute.Cute`
5. **URL Scheme**: `cute://`

## 🔧 故障排查

### 编译错误

```bash
# 清理后重新编译
cargo clean
./script/run
```

### 图标未更新

```bash
# 重新生成图标
python3 scripts/generate_icons.py

# 清理缓存后打包
rm -rf target/*/bundle/osx/Cute.app
./script/macos/bundle
```

### 运行问题

```bash
# 查看详细日志
./script/run -- --log-level debug

# 查看日志文件
tail -f /tmp/cute-*.log
```

## 📚 相关脚本

- `script/run` - 跨平台运行入口
- `script/macos/run` - macOS 专用运行脚本
- `script/macos/bundle` - macOS 打包脚本
- `scripts/generate_icons.py` - 图标生成
- `scripts/remove_icon_border.py` - 去除白边
- `scripts/process_all_icons.py` - 批量处理图标
