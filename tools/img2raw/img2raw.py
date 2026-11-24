
#!/usr/bin/env python3

import argparse
import re
from pathlib import Path
from PIL import Image

MAX_WIDTH = 240
MAX_HEIGHT = 272


def rgb565(r, g, b):
    return ((r >> 3) << 11) | ((g >> 2) << 5) | (b >> 3)


def convert(input_path, output_path, name):
    image = Image.open(input_path).convert("RGB")

    # Resize to fit, preserving aspect ratio.
    # No padding is added.
    image.thumbnail(
        (MAX_WIDTH, MAX_HEIGHT),
        Image.Resampling.LANCZOS,
    )

    width, height = image.size
    pixels = [rgb565(r, g, b) for r, g, b in image.getdata()]

    with open(output_path, "w") as f:
        f.write("#pragma once\n\n")
        f.write("#include <stdint.h>\n\n")

        f.write(f"#define {name.upper()}_WIDTH {width}\n")
        f.write(f"#define {name.upper()}_HEIGHT {height}\n\n")

        f.write(f"static const uint16_t {name}[{len(pixels)}] = {{\n")

        for i in range(0, len(pixels), 12):
            row = pixels[i:i + 12]
            f.write("    " + ", ".join(f"0x{p:04X}" for p in row) + ",\n")

        f.write("};\n")

    print(f"Generated: {output_path}")
    print(f"Dimensions: {width}x{height}")
    print(f"RGB565 size: {len(pixels) * 2} bytes")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("input", type=Path)
    parser.add_argument("output", type=Path, nargs="?")
    parser.add_argument("--name", help="C array name")

    args = parser.parse_args()

    name = args.name or re.sub(r"\W", "_", args.input.stem)

    if not name.isidentifier():
        parser.error("Invalid C identifier")

    output = args.output or args.input.with_suffix(".h")

    convert(args.input, output, name)


if __name__ == "__main__":
    main()
