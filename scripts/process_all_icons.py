#!/usr/bin/env python3
"""Remove white borders from icons for all channels."""

from PIL import Image
import os

CHANNELS = ["dev", "local", "oss", "preview", "stable", "cute"]
SIZES = [16, 32, 48, 64, 128, 256, 512]


def remove_white_border(img, threshold=240):
    """Remove white/transparent border from image."""
    if img.mode != 'RGBA':
        img = img.convert('RGBA')

    width, height = img.size

    left = width
    top = height
    right = 0
    bottom = 0

    for y in range(height):
        for x in range(width):
            r, g, b, a = img.getpixel((x, y))
            if a > 10 and (r < threshold or g < threshold or b < threshold):
                if x < left:
                    left = x
                if x > right:
                    right = x
                if y < top:
                    top = y
                if y > bottom:
                    bottom = y

    content_width = right - left
    content_height = bottom - top
    padding_x = int(content_width * 0.05)
    padding_y = int(content_height * 0.05)

    left = max(0, left - padding_x)
    top = max(0, top - padding_y)
    right = min(width, right + padding_x)
    bottom = min(height, bottom + padding_y)

    return img.crop((left, top, right, bottom))


def process_channel(channel):
    """Process icons for a single channel."""
    icon_dir = f"app/channels/{channel}/icon/no-padding"

    # Find source image (prefer original.png, else largest size)
    original_path = os.path.join(icon_dir, "original.png")
    if not os.path.exists(original_path):
        # Find largest existing icon
        for size in reversed(SIZES):
            path = os.path.join(icon_dir, f"{size}x{size}.png")
            if os.path.exists(path):
                original_path = path
                break

    if not os.path.exists(original_path):
        print(f"  No source image found, skipping")
        return

    print(f"  Processing {original_path}...")
    source = Image.open(original_path)
    cropped = remove_white_border(source)

    for size in SIZES:
        resized = cropped.resize((size, size), Image.Resampling.LANCZOS)
        output_path = os.path.join(icon_dir, f"{size}x{size}.png")
        resized.save(output_path, optimize=True)

    print(f"  Done!")


def main():
    for channel in CHANNELS:
        print(f"\n[{channel}]")
        process_channel(channel)

    print("\n✓ All icons processed!")


if __name__ == "__main__":
    main()
