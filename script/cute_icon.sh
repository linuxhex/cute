#!/bin/bash
#
# Cute 应用图标的唯一来源配置。
# 所有 run / bundle 脚本在打包前必须 source 本文件并调用 verify_cute_icon_assets。
#
# 图标资源：
#   - cargo bundle: app/channels/cute/icon/with-white-border/
#   - macOS 自适应图标: app/channels/cute/icon/AppIcon.icon (由 compile_icon 编译)

: "${REPO_ROOT:?REPO_ROOT must be set before sourcing script/cute_icon.sh}"

CUTE_ICON_CHANNEL="cute"
CUTE_ICON_VARIANT="with-white-border"
CUTE_ICON_BASE="${REPO_ROOT}/app/channels/${CUTE_ICON_CHANNEL}/icon"
CUTE_ICON_ASSET_DIR="${CUTE_ICON_BASE}/${CUTE_ICON_VARIANT}"
CUTE_ICON_APPICON_BUNDLE="${CUTE_ICON_BASE}/AppIcon.icon"

export CUTE_ICON_CHANNEL CUTE_ICON_VARIANT

verify_cute_icon_assets() {
    local missing=0

    for path in \
        "${CUTE_ICON_ASSET_DIR}/512x512.png" \
        "${CUTE_ICON_ASSET_DIR}/icon.ico"; do
        if [[ ! -f "$path" ]]; then
            echo "error: missing Cute icon asset: $path" >&2
            missing=1
        fi
    done

    if [[ ! -d "$CUTE_ICON_APPICON_BUNDLE" ]]; then
        echo "error: missing Cute AppIcon.icon bundle: $CUTE_ICON_APPICON_BUNDLE" >&2
        missing=1
    fi

    if [[ "$missing" -ne 0 ]]; then
        echo "hint: regenerate icons with:" >&2
        echo "  python3 script/generate_macos_icons.py" >&2
        return 1
    fi

    return 0
}

# 将 Cute 官方图标写入 .app bundle（macOS）。
apply_cute_app_icon() {
    local app_bundle_path="$1"
    if [[ -z "$app_bundle_path" ]]; then
        echo "error: apply_cute_app_icon requires an .app bundle path" >&2
        return 1
    fi

    verify_cute_icon_assets || return 1
    "${REPO_ROOT}/script/compile_icon" "$CUTE_ICON_CHANNEL" "$app_bundle_path"
}

# Linux / Windows 打包使用的 PNG/ICO 目录（单层白边版本）。
cute_icon_asset_dir_for_channel() {
    local channel="${1:-$CUTE_ICON_CHANNEL}"
    local variant_dir="${REPO_ROOT}/app/channels/${channel}/icon/${CUTE_ICON_VARIANT}"
    if [[ -d "$variant_dir" && -f "${variant_dir}/512x512.png" ]]; then
        echo "$variant_dir"
        return 0
    fi
    echo "${REPO_ROOT}/app/channels/${channel}/icon/no-padding"
}
