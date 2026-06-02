#!/usr/bin/env python3
"""Remove white borders from icons and regenerate all sizes."""

from PIL import Image
import os

# Paths
SOURCE_PATH = "app/channels/cute/icon/no-padding/original.png"
OUTPUT_DIR = "app/channels/cute/icon/no-padding"

# Target sizes
SIZES = [16, 32, 48, 64, 128, 256, 512]


def remove_white_border(img, threshold=240):
    """Remove white/transparent border from image."""
    # Convert to RGBA if needed
    if img.mode != 'RGBA':
        img = img.convert('RGBA')

    # Get image dimensions
    width, height = img.size

    # Find the bounding box of non-white content
    left = width
    top = height
    right = 0
    bottom = 0

    for y in range(height):
        for x in range(width):
            r, g, b, a = img.getpixel((x, y))
            # Check if pixel is not white (allowing some tolerance)
            if a > 10 and (r < threshold or g < threshold or b < threshold):
                if x < left:
                    left = x
                if x > right:
                    right = x
                if y < top:
                    top = y
                if y > bottom:
                    bottom = y

    # Add small padding (5% of the content size)
    content_width = right - left
    content_height = bottom - top
    padding_x = int(content_width * 0.05)
    padding_y = int(content_height * 0.05)

    left = max(0, left - padding_x)
    top = max(0, top - padding_y)
    right = min(width, right + padding_x)
    bottom = min(height, bottom + padding_y)

    # Crop
    return img.crop((left, top, right, bottom))


def main():
    # Load source image
    print(f"Loading {SOURCE_PATH}...")
    source = Image.open(SOURCE_PATH)

    # Remove white border
    print("Removing white border...")
    cropped = remove_white_border(source)

    # Save cropped original
    cropped_original_path = os.path.join(OUTPUT_DIR, "original_cropped.png")
    cropped.save(cropped_original_path)
    print(f"Saved cropped original to {cropped_original_path}")

    # Generate all sizes
    for size in SIZES:
        print(f"Generating {size}x{size}...")
        # Resize with high quality
        resized = cropped.resize((size, size), Image.Resampling.LANCZOS)

        # Save
        output_path = os.path.join(OUTPUT_DIR, f"{size}x{size}.png")
        resized.save(output_path, optimize=True)
        print(f"  Saved to {output_path}")

    print("\nDone! All icons regenerated without white borders.")


if __name__ == "__main__":
    main()
