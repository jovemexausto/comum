from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]


def md_files(*roots: str):
    for root in roots:
        path = ROOT / root
        if path.exists():
            yield from path.rglob("*.md")


def read_text(rel: str | Path) -> str:
    path = ROOT / rel if isinstance(rel, str) else rel
    return path.read_text(encoding="utf-8")


def assert_exists(rel: str) -> None:
    if not (ROOT / rel).exists():
        raise AssertionError(f"missing: {rel}")


def assert_not_exists(rel: str) -> None:
    if (ROOT / rel).exists():
        raise AssertionError(f"should not exist: {rel}")


def assert_no_refs(pattern: str, roots: list[str]) -> None:
    regex = re.compile(pattern)
    hits: list[str] = []
    for file in md_files(*roots):
        if regex.search(read_text(file)):
            hits.append(str(file.relative_to(ROOT)))
    if hits:
        raise AssertionError(f"forbidden refs /{pattern}/ in: {hits}")


def assert_only_children(base: str, allowed: list[str]) -> None:
    path = ROOT / base
    if not path.exists():
        raise AssertionError(f"missing dir: {base}")
    actual = {child.name for child in path.iterdir()}
    extra = sorted(actual - set(allowed))
    if extra:
        raise AssertionError(f"unexpected children in {base}: {extra}")


def assert_max_lines(root: str, max_lines: int) -> None:
    bad: list[tuple[str, int]] = []
    for file in md_files(root):
        lines = read_text(file).count("\n") + 1
        if lines > max_lines:
            bad.append((str(file.relative_to(ROOT)), lines))
    if bad:
        raise AssertionError(f"files over line cap {max_lines}: {bad}")


def check_phase_1() -> None:
    assert_exists(".archive/README.md")
    assert_exists(".archive/INDEX.md")
    assert_exists("docs/corpus/README.md")
    assert_exists("docs/implementation/README.md")
    assert_exists("docs/project/README.md")
    assert_no_refs(r"\.archive/", ["README.md", "ABSTRACT.md", "docs", "spec"])


def check_phase_2() -> None:
    assert_exists("README.md")
    assert_exists("ABSTRACT.md")
    assert_not_exists("docs/teoria-do-protocolo.md")
    assert_exists(".archive/docs/01-theory-transition/00-teoria-do-protocolo.md")
    readme = read_text("README.md")
    if "docs/corpus/" not in readme:
        raise AssertionError("README must reference docs/corpus/")
    if "spec/cips/CIP-0001.md" not in readme:
        raise AssertionError("README must reference spec/cips/CIP-0001.md")
    assert_no_refs(
        r"docs/teoria-do-protocolo\.md", ["README.md", "ABSTRACT.md", "docs", "spec"]
    )


def check_phase_3() -> None:
    assert_exists("docs/corpus/00-o-que-e-o-comum.md")
    assert_not_exists("docs/livro")
    assert_no_refs(r"docs/livro/", ["README.md", "ABSTRACT.md", "docs", "spec"])
    expected = {
        "00-o-que-e-o-comum.md",
        "01-apresentacao.md",
        "02-o-problema-da-coordenacao.md",
        "03-do-centro-ao-comum.md",
        "04-testemunho-e-legibilidade-publica.md",
        "05-contexto-prova-e-legitimidade.md",
        "06-autoridade-emergente-e-revogabilidade.md",
        "07-par-triade-e-escala-institucional.md",
        "08-comunidade-fronteira-e-federacao.md",
        "09-valor-plural-e-interoperabilidade.md",
        "10-compensacao-sem-equivalencia-unica.md",
        "11-infraestrutura-e-colonizacao-abstrata.md",
        "12-instituicoes-programaveis-e-limites.md",
        "13-conclusoes-e-perguntas-abertas.md",
        "README.md",
    }
    actual = {p.name for p in (ROOT / "docs/corpus").glob("*.md")}
    if actual != expected:
        raise AssertionError(
            f"docs/corpus mismatch: expected {sorted(expected)}, got {sorted(actual)}"
        )


def check_phase_4() -> None:
    assert_exists("docs/project/roadmap.md")
    assert_exists("docs/project/implementation-plan.md")
    assert_exists("docs/project/releases")
    assert_not_exists("docs/context.md")
    assert_exists(".archive/docs/04-project-ops/00-contexto-de-evolucao.md")
    assert_no_refs(r"docs/context\.md", ["README.md", "ABSTRACT.md", "docs", "spec"])
    assert_no_refs(r"docs/release-notes/", ["README.md", "ABSTRACT.md", "docs", "spec"])
    assert_no_refs(r"docs/roadmap\.md", ["README.md", "ABSTRACT.md", "docs", "spec"])
    assert_no_refs(
        r"docs/implementation-plan\.md", ["README.md", "ABSTRACT.md", "docs", "spec"]
    )


def check_phase_5() -> None:
    assert_not_exists("spec/rfcs")
    assert_exists(".archive/spec/00-rfcs")
    assert_no_refs(r"spec/rfcs/", ["README.md", "ABSTRACT.md", "docs", "spec"])
    assert_exists("spec/cips")
    assert_exists("spec/registries")
    assert_exists("spec/test-vectors")


def check_phase_6() -> None:
    assert_exists("docs/notes/README.md")
    assert_exists("docs/notes/INDEX.md")
    assert_max_lines("docs/notes", 180)
    forbidden_prefixes = ("dump-", "sintese-", "mapa-", "plano-")
    allowed = {
        "README.md",
        "INDEX.md",
        "00-metodo-epistemologico.md",
    }
    bad = []
    for file in (ROOT / "docs/notes").glob("*.md"):
        if file.name in allowed:
            continue
        if file.name.startswith(forbidden_prefixes):
            bad.append(file.name)
    if bad:
        raise AssertionError(
            f"forbidden long-form note names still alive: {sorted(bad)}"
        )
    note_count = len(list((ROOT / "docs/notes").glob("*.md")))
    if note_count > 18:
        raise AssertionError(f"too many live notes: {note_count}")


def check_phase_7() -> None:
    assert_exists("docs/epistemologia/README.md")
    assert_exists("docs/epistemologia/guia-editorial.md")
    assert_exists("docs/epistemologia/arvore-editorial-mestra.md")
    assert_only_children(
        "docs/epistemologia",
        [
            "README.md",
            "guia-editorial.md",
            "arvore-editorial-mestra.md",
        ],
    )


def check_phase_8() -> None:
    assert_exists("docs/implementation/README.md")
    # At least one technical doc beyond README should exist by this phase.
    md_names = {p.name for p in (ROOT / "docs/implementation").glob("*.md")}
    if len(md_names) < 2:
        raise AssertionError(
            "docs/implementation must contain more than README by phase 8"
        )


def check_phase_final() -> None:
    assert_exists("README.md")
    assert_exists("ABSTRACT.md")
    assert_exists("docs/corpus")
    assert_exists("docs/implementation")
    assert_exists("docs/project")
    assert_exists("docs/notes")
    assert_not_exists("docs/livro")
    assert_not_exists("docs/teoria-do-protocolo.md")
    assert_not_exists("spec/rfcs")
    assert_not_exists("docs/context.md")
    assert_no_refs(r"docs/livro/", ["README.md", "ABSTRACT.md", "docs", "spec"])
    assert_no_refs(
        r"docs/teoria-do-protocolo\.md", ["README.md", "ABSTRACT.md", "docs", "spec"]
    )
    assert_no_refs(r"spec/rfcs/", ["README.md", "ABSTRACT.md", "docs", "spec"])
    assert_no_refs(r"\.archive/", ["README.md", "ABSTRACT.md", "docs", "spec"])


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--phase", required=True)
    args = parser.parse_args()

    checks = {
        "1": check_phase_1,
        "2": check_phase_2,
        "3": check_phase_3,
        "4": check_phase_4,
        "5": check_phase_5,
        "6": check_phase_6,
        "7": check_phase_7,
        "8": check_phase_8,
        "final": check_phase_final,
    }
    try:
        checks[args.phase]()
    except KeyError:
        print(f"unknown phase: {args.phase}", file=sys.stderr)
        return 2
    except AssertionError as exc:
        print(f"AUDIT FAILED: {exc}", file=sys.stderr)
        return 1

    print(f"phase {args.phase}: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
