#!/usr/bin/env python3
"""
为 Cute 应用生成带厚白边的 macOS 风格图标
用于打包后的 .app 和 .dmg 显示
"""

from PIL import Image, ImageDraw
import os

SOURCE_ICON = "app/channels/cute/icon/no-padding/original_cropped.png"
OUTPUT_DIR = "app/channels/cute/icon/with-white-border"

SIZES = [16, 32, 48, 64, 128, 256, 512, 1024]

WHITE_BORDER_RATIO = 0.32  # 白边比例：从 0.24 增加到 0.32，使白边更宽
CORNER_RADIUS_RATIO = 0.20  # 圆角比例：稍微减小以配合更宽的白边


def create_rounded_rectangle(draw, bbox, radius, fill):
    x0, y0, x1, y1 = bbox
    diameter = radius * 2
    
    draw.pieslice([x0, y0, x0 + diameter, y0 + diameter], 180, 270, fill=fill)
    draw.pieslice([x1 - diameter, y0, x1, y0 + diameter], 270, 360, fill=fill)
    draw.pieslice([x0, y1 - diameter, x0 + diameter, y1], 90, 180, fill=fill)
    draw.pieslice([x1 - diameter, y1 - diameter, x1, y1], 0, 90, fill=fill)
    
    draw.rectangle([x0 + radius, y0, x1 - radius, y1], fill=fill)
    draw.rectangle([x0, y0 + radius, x1, y1 - radius], fill=fill)


def extract_dark_pattern(source_img, threshold=245):
    """
    从源图标中提取深色图案，将浅色背景设为透明
    
    参数:
        source_img: 源图标 (RGBA)
        threshold: 浅色阈值，高于此值的像素将被设为透明
    
    返回:
        仅包含深色图案的图标（透明背景）
    """
    if source_img.mode != 'RGBA':
        source_img = source_img.convert('RGBA')
    
    result = source_img.copy()
    width, height = result.size
    
    for y in range(height):
        for x in range(width):
            r, g, b, a = result.getpixel((x, y))
            if a > 0 and r > threshold and g > threshold and b > threshold:
                result.putpixel((x, y), (0, 0, 0, 0))
    
    return result


def add_white_border(source_img, target_size, border_ratio=WHITE_BORDER_RATIO, corner_radius_ratio=CORNER_RADIUS_RATIO):
    """
    为图标添加厚白边和圆角背景
    """
    if source_img.mode != 'RGBA':
        source_img = source_img.convert('RGBA')
    
    core_icon = extract_dark_pattern(source_img)
    
    bbox = core_icon.getbbox()
    if bbox is None:
        return Image.new('RGBA', (target_size, target_size), (255, 255, 255, 255))
    
    left, top, right, bottom = bbox
    content_width = right - left
    content_height = bottom - top
    content_size = max(content_width, content_height)
    
    border_size = int(content_size * border_ratio)
    new_content_size = content_size + 2 * border_size
    
    scale = target_size / new_content_size
    
    new_width = int(content_width * scale)
    new_height = int(content_height * scale)
    scaled_icon = core_icon.crop((left, top, right, bottom)).resize((new_width, new_height), Image.Resampling.LANCZOS)
    
    bg_size = target_size
    background = Image.new('RGBA', (bg_size, bg_size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(background)
    
    corner_radius = int(target_size * corner_radius_ratio)
    
    # 单层白边：白底铺满圆角容器，不再额外留透明外圈，避免视觉上像“双层边框”。
    padding = 0
    bg_bbox = [padding, padding, bg_size - padding, bg_size - padding]
    create_rounded_rectangle(draw, bg_bbox, corner_radius, (255, 255, 255, 255))
    
    x_offset = (bg_size - new_width) // 2
    y_offset = (bg_size - new_height) // 2
    background.paste(scaled_icon, (x_offset, y_offset), scaled_icon)
    
    return background


def generate_all_sizes():
    """为所有尺寸生成带白边的图标"""
    print("🎨 开始生成带厚白边的 Cute 图标...")
    
    # 加载源图标
    if not os.path.exists(SOURCE_ICON):
        print(f"❌ 源图标不存在: {SOURCE_ICON}")
        return
    
    source_img = Image.open(SOURCE_ICON)
    print(f"✅ 加载源图标: {SOURCE_ICON}")
    print(f"   原始尺寸: {source_img.size}")
    
    # 创建输出目录
    os.makedirs(OUTPUT_DIR, exist_ok=True)
    
    # 为每个尺寸生成图标
    for size in SIZES:
        print(f"\n📐 生成 {size}x{size}...")
        
        # 添加白边
        result = add_white_border(source_img, size)
        
        # 保存
        output_path = os.path.join(OUTPUT_DIR, f"{size}x{size}.png")
        result.save(output_path, 'PNG', optimize=True)
        print(f"   ✅ {output_path}")
    
    # 生成用于 macOS 的 .icns 需要的所有尺寸
    print(f"\n📦 图标已生成到: {OUTPUT_DIR}")
    print("\n可用尺寸:")
    for size in SIZES:
        path = os.path.join(OUTPUT_DIR, f"{size}x{size}.png")
        if os.path.exists(path):
            file_size = os.path.getsize(path)
            print(f"   ✓ {size}x{size}.png ({file_size/1024:.1f} KB)")
    
    print("\n💡 下一步:")
    print("   1. 更新 Cargo.toml 中的 icon 配置")
    print("   2. 重新打包应用: script/macos/bundle")
    print("   3. 查看效果: 打开 target/*/bundle/osx/dmg/cute/Cute.dmg")


if __name__ == "__main__":
    generate_all_sizes()
