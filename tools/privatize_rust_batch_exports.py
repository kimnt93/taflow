"""Make lowercase stream reexports crate-only while retaining public TA classes."""

import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
TARGET = ROOT / "crates/taflow-core/src/stream/mod.rs"


def visibility(name: str) -> str:
    return "public" if name.lstrip().startswith(tuple("ABCDEFGHIJKLMNOPQRSTUVWXYZ")) else "crate"


def rewrite(match: re.Match[str]) -> str:
    module, raw = match.group("module"), match.group("items")
    items = [item.strip() for item in raw.split(",") if item.strip()]
    public = [item for item in items if visibility(item) == "public"]
    crate = [item for item in items if visibility(item) == "crate"]
    lines = []
    if public:
        lines.append(f"pub use {module}::{{{', '.join(public)}}};")
    if crate:
        lines.append("#[allow(unused_imports)]")
        lines.append(f"pub(crate) use {module}::{{{', '.join(crate)}}};")
    return "\n".join(lines)


def main() -> None:
    text = TARGET.read_text()
    text = re.sub(
        r"pub use (?P<module>[a-zA-Z0-9_]+)::\{(?P<items>.*?)\};",
        rewrite,
        text,
        flags=re.S,
    )
    text = re.sub(
        r"^pub use (?P<module>[a-zA-Z0-9_]+)::(?P<item>[a-z_][a-zA-Z0-9_]*);$",
        lambda m: (f"#[allow(unused_imports)]\n"
                   f"pub(crate) use {m.group('module')}::{m.group('item')};"),
        text,
        flags=re.M,
    )
    TARGET.write_text(text)


if __name__ == "__main__":
    main()
