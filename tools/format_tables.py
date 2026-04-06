import re
import sys
from pathlib import Path


SEP_RE = re.compile(r"^\s*\|?\s*[:\- ]+(\|\s*[:\- ]+)+\|?\s*$")


def parse_row(line: str) -> list[str]:
    parts = [cell.strip() for cell in line.strip().strip("|").split("|")]
    return parts


def is_table_line(line: str) -> bool:
    return "|" in line and not line.strip().startswith(">")


def is_sep_line(line: str) -> bool:
    return bool(SEP_RE.match(line))


def format_table(lines: list[str], force_no_colons: bool) -> list[str]:
    rows = [parse_row(line) for line in lines]
    sep_idx = next((i for i, line in enumerate(lines) if is_sep_line(line)), None)

    aligns = []
    if force_no_colons:
        aligns = ["none"] * len(rows[0])
    elif sep_idx is not None:
        sep_cells = parse_row(lines[sep_idx])
        for cell in sep_cells:
            left = cell.startswith(":")
            right = cell.endswith(":")
            if left and right:
                aligns.append("center")
            elif right:
                aligns.append("right")
            elif left:
                aligns.append("left")
            else:
                aligns.append("none")
    else:
        aligns = ["none"] * len(rows[0])

    data_rows = [row for i, row in enumerate(rows) if i != sep_idx]
    cols = max(len(row) for row in data_rows)
    for row in data_rows:
        if len(row) < cols:
            row.extend([""] * (cols - len(row)))
    widths = [0] * cols
    for row in data_rows:
        for i, cell in enumerate(row):
            widths[i] = max(widths[i], len(cell))

    def format_row(row: list[str]) -> str:
        return "| " + " | ".join(row[i].ljust(widths[i]) for i in range(cols)) + " |"

    def format_sep() -> str:
        parts = []
        for i, align in enumerate(aligns):
            dashes = "-" * widths[i]
            if align == "center":
                parts.append(f":{dashes}:")
            elif align == "right":
                parts.append(f"{dashes}:")
            elif align == "left":
                parts.append(f":{dashes}")
            else:
                parts.append(dashes)
        return "| " + " | ".join(parts) + " |"

    output = []
    output.append(format_row(data_rows[0]))
    output.append(format_sep())
    for row in data_rows[1:]:
        output.append(format_row(row))
    return output


def format_markdown(text: str, force_no_colons: bool) -> str:
    lines = text.splitlines()
    output = []
    in_code = False
    i = 0
    while i < len(lines):
        line = lines[i]
        if line.strip().startswith("```"):
            in_code = not in_code
            output.append(line)
            i += 1
            continue

        if not in_code and is_table_line(line):
            block = []
            while (
                i < len(lines)
                and is_table_line(lines[i])
                and not lines[i].strip().startswith("```")
            ):
                block.append(lines[i])
                i += 1
            if any(is_sep_line(item) for item in block):
                output.extend(format_table(block, force_no_colons))
            else:
                output.extend(block)
            continue

        output.append(line)
        i += 1

    return "\n".join(output) + ("\n" if text.endswith("\n") else "")


def main(paths: list[str], force_no_colons: bool) -> None:
    for item in paths:
        path = Path(item)
        if path.is_dir():
            for md in path.rglob("*.md"):
                md.write_text(
                    format_markdown(md.read_text(encoding="utf-8"), force_no_colons),
                    encoding="utf-8",
                )
        else:
            path.write_text(
                format_markdown(path.read_text(encoding="utf-8"), force_no_colons),
                encoding="utf-8",
            )


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print(
            "Usage: python tools/format_tables.py [--keep-colons] <path-or-md-file> [more...]"
        )
        raise SystemExit(2)
    args = sys.argv[1:]
    force_no_colons = True
    if "--keep-colons" in args:
        force_no_colons = False
        args = [arg for arg in args if arg != "--keep-colons"]
    if not args:
        print(
            "Usage: python tools/format_tables.py [--keep-colons] <path-or-md-file> [more...]"
        )
        raise SystemExit(2)
    main(args, force_no_colons)
