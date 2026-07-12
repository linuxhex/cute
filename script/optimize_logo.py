#!/usr/bin/env python3
"""
优化 logoimg.png：
1. 去除白边
2. 添加阴影和光晕效果增加质感
3. 平滑边缘
"""

from PIL import Image, ImageFilter, ImageDraw, ImageEnhance
import numpy as np
from pathlib import Path

def remove_white_border(img, threshold=240):
    """去除白色边框"""
    # 转為 numpy 数组
    arr = np.array(img)

    # 找到非白色区域的边界
    if arr.shape[2] == 4:  # RGBA
        # 检查 RGB 通道，忽略 alpha
        rgb = arr[:, :, :3]
        alpha = arr[:, :, 3]
        # 非白色或非透明区域
        mask = (rgb < threshold).any(axis=2) | (alpha > 10)
    else:  # RGB
        mask = (arr < threshold).any(axis=2)

    # 找到边界
    rows = np.any(mask, axis=1)
    cols = np.any(mask, axis=0)

    if not rows.any() or not cols.any():
        return img

    rmin, rmax = np.where(rows)[0][[0, -1]]
    cmin, cmax = np.where(cols)[0][[0, -1]]

    # 添加一点边距
    padding = 5
    rmin = max(0, rmin - padding)
    rmax = min(img.height - 1, rmax + padding)
    cmin = max(0, cmin - padding)
    cmax = min(img.width - 1, cmax + padding)

    return img.crop((cmin, rmin, cmax + 1, rmax + 1))


def add_shadow_and_glow(img, shadow_offset=8, shadow_blur=15, glow_radius=20):
    """添加阴影和光晕效果"""
    # 确保是 RGBA 模式
    if img.mode != 'RGBA':
        img = img.convert('RGBA')

    # 创建更大的画布以容纳阴影
    padding = max(shadow_offset, glow_radius) + shadow_blur
    new_size = (img.width + padding * 2, img.height + padding * 2)

    # 创建最终画布
    result = Image.new('RGBA', new_size, (0, 0, 0, 0))

    # 创建阴影层
    shadow = Image.new('RGBA', new_size, (0, 0, 0, 0))
    shadow_alpha = img.split()[3]  # 获取 alpha 通道
    shadow_layer = Image.new('RGBA', img.size, (30, 30, 30, 255))  # 深灰色阴影
    shadow_layer.putalpha(shadow_alpha)

    # 将阴影放到画布上（带偏移）
    shadow.paste(shadow_layer, (padding + shadow_offset, padding + shadow_offset), shadow_layer)
    # 模糊阴影
    shadow = shadow.filter(ImageFilter.GaussianBlur(shadow_blur))

    # 创建光晕效果
    glow = Image.new('RGBA', new_size, (0, 0, 0, 0))
    # 创建光晕层（淡蓝色光晕，符合终端风格）
    glow_layer = Image.new('RGBA', img.size, (100, 180, 255, 100))
    glow_layer.putalpha(shadow_alpha)
    glow.paste(glow_layer, (padding, padding), glow_layer)
    glow = glow.filter(ImageFilter.GaussianBlur(glow_radius))

    # 合并图层：光晕 -> 阴影 -> 原图
    result = Image.alpha_composite(result, glow)
    result = Image.alpha_composite(result, shadow)
    result.paste(img, (padding, padding), img)

    return result, padding


def enhance_colors(img):
    """增强颜色对比度和饱和度"""
    # 增强对比度
    enhancer = ImageEnhance.Contrast(img)
    img = enhancer.enhance(1.15)

    # 增强饱和度
    enhancer = ImageEnhance.Color(img)
    img = enhancer.enhance(1.1)

    return img


def smooth_edges(img, radius=1):
    """平滑边缘"""
    # 轻微模糊后再锐化，可以平滑锯齿边缘
    img = img.filter(ImageFilter.GaussianBlur(radius))
    img = img.filter(ImageFilter.UnsharpMask(radius=radius * 2, percent=150, threshold=3))
    return img


def main():
    # 路径设置
    script_dir = Path(__file__).parent
    project_root = script_dir.parent
    input_path = project_root / "images" / "logoimg.png"
    output_path = project_root / "images" / "logoimg_optimized.png"

    print(f"读取原图: {input_path}")
    img = Image.open(input_path)
    print(f"原图尺寸: {img.size}, 模式: {img.mode}")

    # 步骤 1: 去除白边
    print("\n1. 去除白色边框...")
    img = remove_white_border(img, threshold=245)
    print(f"   裁剪后尺寸: {img.size}")

    # 步骤 2: 平滑边缘
    print("\n2. 平滑边缘...")
    img = smooth_edges(img, radius=1)

    # 步骤 3: 增强颜色
    print("\n3. 增强颜色对比度和饱和度...")
    img = enhance_colors(img)

    # 步骤 4: 添加阴影和光晕
    print("\n4. 添加阴影和光晕效果...")
    img, padding = add_shadow_and_glow(img, shadow_offset=6, shadow_blur=12, glow_radius=15)
    print(f"   添加效果后尺寸: {img.size}")

    # 保存结果
    print(f"\n保存优化后的图片: {output_path}")
    img.save(output_path, 'PNG', optimize=True)

    # 同时保存到 bundled 目录
    bundled_path = project_root / "app" / "assets" / "bundled" / "png" / "logoimg.png"
    print(f"更新 bundled 资源: {bundled_path}")
    img.save(bundled_path, 'PNG', optimize=True)

    print("\n✅ 优化完成！")
    print(f"   - 去除了白色边框")
    print(f"   - 添加了阴影效果增加立体感")
    print(f"   - 添加了淡蓝色光晕效果")
    print(f"   - 增强了颜色对比度和饱和度")
    print(f"   - 平滑了边缘锯齿")


if __name__ == "__main__":
    main()
