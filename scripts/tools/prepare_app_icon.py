#!/usr/bin/env python3
"""Square-crop source image to 1024x1024 for Tauri icon generator."""
import sys
from pathlib import Path

try:
    from PIL import Image
except ImportError:
    print("pip install pillow")
    raise

def main() -> None:
    src = Path(sys.argv[1])
    out = Path(sys.argv[2])
    img = Image.open(src).convert("RGBA")
    w, h = img.size
    side = min(w, h)
    left = (w - side) // 2
    top = (h - side) // 2
    img = img.crop((left, top, left + side, top + side))
    img = img.resize((1024, 1024), Image.Resampling.LANCZOS)
    out.parent.mkdir(parents=True, exist_ok=True)
    img.save(out, format="PNG")
    print(f"Wrote {out} ({out.stat().st_size} bytes)")

if __name__ == "__main__":
    main()
