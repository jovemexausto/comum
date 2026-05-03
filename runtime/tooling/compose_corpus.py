import re
from pathlib import Path


CHAPTER_RE = re.compile(r"^(\d\d)-(.+)\.md$")
HEADING_RE = re.compile(r"^(#{1,6})\s+(.*)$")


def chapter_key(path: Path) -> tuple[int, str]:
    match = CHAPTER_RE.match(path.name)
    if match:
        return (int(match.group(1)), path.name)
    return (10_000, path.name)


def collect_sections(corpus_dir: Path) -> list[Path]:
    return sorted(
        [path for path in corpus_dir.glob("*.md") if path.name != "README.md"],
        key=chapter_key,
    )


def strip_first_heading(text: str) -> str:
    lines = text.splitlines()
    for index, line in enumerate(lines):
        if HEADING_RE.match(line):
            return "\n".join(lines[index + 1 :]).strip()
    return text.strip()


def demote_headings(text: str, levels: int = 1) -> str:
    lines = text.splitlines()
    output: list[str] = []
    in_code = False

    for line in lines:
        if line.strip().startswith("```"):
            in_code = not in_code
            output.append(line)
            continue

        if not in_code:
            match = HEADING_RE.match(line)
            if match:
                hashes, title = match.groups()
                output.append("#" * min(6, len(hashes) + levels) + f" {title}")
                continue

        output.append(line)

    return "\n".join(output).strip()


def build_document(corpus_dir: Path) -> str:
    readme = corpus_dir / "README.md"
    sections = collect_sections(corpus_dir)

    parts: list[str] = []
    parts.append("# Corpus consolidado")

    if readme.exists():
        intro = strip_first_heading(readme.read_text(encoding="utf-8"))
        if intro:
            parts.append("## Prefacio editorial")
            parts.append(demote_headings(intro))

    for path in sections:
        text = path.read_text(encoding="utf-8").strip()
        title = path.stem
        body = strip_first_heading(text)
        first_line = text.splitlines()[0].lstrip("# ").strip() if text else path.stem
        parts.append(f"## {title} - {first_line}")
        if body:
            parts.append(demote_headings(body))

    return "\n\n".join(parts).rstrip() + "\n"


def main() -> None:
    repo_root = Path(__file__).resolve().parents[2]
    corpus_dir = repo_root / "docs" / "corpus"
    output_path = corpus_dir / "corpus-unificado.md"

    if not corpus_dir.is_dir():
        raise SystemExit(f"Corpus directory not found: {corpus_dir}")

    output_path.write_text(build_document(corpus_dir), encoding="utf-8")
    print(output_path)


if __name__ == "__main__":
    main()
