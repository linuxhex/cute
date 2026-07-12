#!/usr/bin/env python3
"""
图标生成脚本 - 从 logoimg.png 生成所有尺寸的图标
解决白边问题，确保图标填充整个画布
"""

import os
from PIL import Image

# 源图标路径
SOURCE_ICON = "/Users/caomunian/Study/cute/images/logoimg.png"

# 目标渠道和尺寸
CHANNELS = ["stable", "local", "oss", "dev", "preview", "cute"]
SIZES = [16, 32, 48, 64, 128, 256, 512]

# 基础路径
BASE_PATH = "/Users/caomunian/Study/cute/app/channels"


def resize_icon(source_img, size):
    """调整图标大小，保持宽高比，填充整个画布"""
    # 获取原始尺寸
    width, height = source_img.size

    # 计算缩放比例，确保填满目标尺寸
    ratio = max(size / width, size / height)
    new_width = int(width * ratio)
    new_height = int(height * ratio)

    # 缩放图像
    resized = source_img.resize((new_width, new_height), Image.Resampling.LANCZOS)

    # 创建目标图像（透明背景）
    result = Image.new('RGBA', (size, size), (0, 0, 0, 0))

    # 计算居中位置
    x = (size - new_width) // 2
    y = (size - new_height) // 2

    # 粘贴缩放后的图像
    result.paste(resized, (x, y), resized if resized.mode == 'RGBA' else None)

    return result


def create_ico(source_img, output_path):
    """创建 .ico 文件"""
    # 生成多个尺寸用于 ico 文件
    sizes = [16, 32, 48, 64, 128, 256]
    images = []

    for size in sizes:
        img = resize_icon(source_img, size)
        images.append(img)

    # 保存为 ico
    images[0].save(
        output_path,
        format='ICO',
        sizes=[(img.width, img.height) for img in images],
        append_images=images[1:]
    )


def main():
    print("🎨 开始生成图标...")

    # 加载源图标
    source_img = Image.open(SOURCE_ICON)
    print(f"✅ 加载源图标: {SOURCE_ICON}")
    print(f"   原始尺寸: {source_img.size}")

    # 转換為 RGBA 模式
    if source_img.mode != 'RGBA':
        source_img = source_img.convert('RGBA')
        print("   转换为 RGBA 模式")

    # 为每个渠道生成图标
    for channel in CHANNELS:
        channel_path = os.path.join(BASE_PATH, channel, "icon", "no-padding")
        os.makedirs(channel_path, exist_ok=True)

        print(f"\n📁 处理渠道: {channel}")

        # 生成各个尺寸的 PNG
        for size in SIZES:
            output_path = os.path.join(channel_path, f"{size}x{size}.png")
            resized = resize_icon(source_img, size)
            resized.save(output_path, 'PNG')
            print(f"   ✅ {size}x{size}.png")

        # 生成 ico 文件
        ico_path = os.path.join(channel_path, "icon.ico")
        create_ico(source_img, ico_path)
        print(f"   ✅ icon.ico")

    # 同时更新 bundled 目录中的图标
    bundled_path = "/Users/caomunian/Study/cute/app/assets/bundled/png"
    os.makedirs(bundled_path, exist_ok=True)

    # 复制源图标到 bundled 目录
    source_img.save(os.path.join(bundled_path, "logoimg.png"), 'PNG')
    print(f"\n✅ 更新 bundled 目录中的图标")

    print("\n🎉 图标生成完成！")
    print("\n已更新的渠道:")
    for channel in CHANNELS:
        print(f"  - {channel}")
    print(f"  - bundled")


if __name__ == "__main__":
    main()
