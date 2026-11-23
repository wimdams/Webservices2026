import re
import struct
import sys
import zlib
from pathlib import Path

NAME_LEN = 32
ENTRY_FMT = f"<{NAME_LEN}sII"
ENTRY_SIZE = struct.calcsize(ENTRY_FMT)


def extract_fs(c_file_path: str, output_dir: str | None = None) -> None:
    text = Path(c_file_path).read_text(encoding="utf-8")

    match = re.search(r"fs_image\s*\[\s*\d*\s*\]\s*=\s*\{([^}]*)\}", text, re.DOTALL)
    if not match:
        raise ValueError("Could not find fs_image array in the provided C file.")

    hex_tokens = re.findall(r"0x[0-9a-fA-F]{1,2}", match.group(1))
    image = bytes(int(tok, 16) for tok in hex_tokens)

    if len(image) < 5:
        raise ValueError("Image data is too short to contain header and checksum.")

    payload = image[:-4]
    (stored_crc,) = struct.unpack("<I", image[-4:])
    computed_crc = zlib.crc32(payload) & 0xFFFFFFFF

    if stored_crc != computed_crc:
        raise ValueError(
            f"Checksum mismatch: stored 0x{stored_crc:08x}, computed 0x{computed_crc:08x}"
        )

    file_count = image[0]
    out_path = Path(output_dir) if output_dir else None
    if out_path:
        out_path.mkdir(parents=True, exist_ok=True)

    for i in range(file_count):
        offset = 1 + i * ENTRY_SIZE
        raw_name, start_addr, size = struct.unpack_from(ENTRY_FMT, image, offset)
        filename = raw_name.split(b"\x00", 1)[0].decode("utf-8", errors="replace")
        content = image[start_addr : start_addr + size]

        if out_path:
            (out_path / filename).write_bytes(content)
            print(f"Extracted: {filename} ({size} bytes)")
        else:
            print(f"=== {filename} ({size} bytes) ===")
            print(content.decode("utf-8", errors="replace"))


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print(f"Usage: python {sys.argv[0]} <fs.c> [output_dir]")
        sys.exit(1)

    extract_fs(sys.argv[1], sys.argv[2] if len(sys.argv) > 2 else None)
