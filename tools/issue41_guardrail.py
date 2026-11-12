#!/usr/bin/env python3
"""issue #41 静态检查闸门。

目标：阻止同域新增第二套执行范式，并对新增/整改 API 做最小持续性检查。

默认聚焦业务 crate：
- openlark-docs
- openlark-meeting
- openlark-communication
- openlark-hr
"""

from __future__ import annotations

import argparse
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable

_REPO_ROOT = Path(__file__).resolve().parents[1]
if str(_REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(_REPO_ROOT))

from tools.verify_api_fields import _mask_rust_non_code

DEFAULT_TARGET_CRATES = [
    "openlark-docs",
    "openlark-meeting",
    "openlark-communication",
    "openlark-hr",
]

EXCLUDED_FILE_NAMES = {
    "mod.rs",
    "lib.rs",
    "models.rs",
    "responses.rs",
    "service.rs",
    "macros.rs",
}


@dataclass
class Finding:
    severity: str
    code: str
    path: str
    line: int
    message: str


@dataclass
class RequestContext:
    name: str
    line: int
    struct_text: str
    impl_text: str


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="issue #41 Rust API 静态检查闸门（最小可用版）"
    )
    parser.add_argument(
        "--repo-root",
        type=Path,
        default=Path(__file__).resolve().parents[1],
        help="仓库根目录（默认自动推断）",
    )
    parser.add_argument(
        "--crates",
        nargs="+",
        default=DEFAULT_TARGET_CRATES,
        help="目标业务 crate 名称列表（默认 docs/meeting/communication/hr）",
    )
    parser.add_argument(
        "--strict-warn",
        action="store_true",
        help="将 WARN 也视为失败（默认仅 ERROR 失败）",
    )
    return parser.parse_args()


def iter_target_files(repo_root: Path, crate_names: list[str]) -> Iterable[Path]:
    for crate_name in crate_names:
        crate_src = repo_root / "crates" / crate_name / "src"
        if not crate_src.exists():
            print(f"[WARN] crate 路径不存在，已跳过: {crate_src}")
            continue

        for file in crate_src.rglob("*.rs"):
            if file.name in EXCLUDED_FILE_NAMES:
                continue
            if "/common/" in file.as_posix():
                continue
            yield file


def line_of(text: str, index: int) -> int:
    return text.count("\n", 0, max(index, 0)) + 1


def find_block_end(text: str, start_brace_idx: int) -> int:
    depth = 0
    for i in range(start_brace_idx, len(text)):
        ch = text[i]
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                return i
    return -1


def extract_execute_functions(text: str) -> list[tuple[int, str]]:
    results: list[tuple[int, str]] = []
    for m in re.finditer(r"pub\s+async\s+fn\s+execute\s*\(", text):
        brace_idx = text.find("{", m.end())
        if brace_idx < 0:
            continue
        end = find_block_end(text, brace_idx)
        if end < 0:
            continue
        results.append((m.start(), text[brace_idx + 1 : end]))
    return results


def find_transport_request_none_calls(text: str) -> list[int]:
    positions: list[int] = []
    needle = "Transport::request("
    idx = 0
    while True:
        start = text.find(needle, idx)
        if start < 0:
            break

        open_paren_idx = start + len("Transport::request")
        end = find_matching_paren(text, open_paren_idx)
        if end < 0:
            idx = start + len(needle)
            continue

        call_body = text[open_paren_idx + 1 : end]
        if re.search(r",\s*None\s*$", call_body, re.DOTALL):
            positions.append(start)

        idx = end + 1

    return positions


def find_matching_paren(text: str, open_paren_idx: int) -> int:
    if open_paren_idx < 0 or open_paren_idx >= len(text) or text[open_paren_idx] != "(":
        return -1

    depth = 0
    for i in range(open_paren_idx, len(text)):
        ch = text[i]
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
            if depth == 0:
                return i
    return -1


def seems_request_file(text: str) -> bool:
    return bool(
        re.search(r"pub\s+struct\s+\w+Request\b", text)
        and re.search(r"pub\s+async\s+fn\s+execute\b", text)
    )


def extract_request_contexts(text: str) -> list[RequestContext]:
    contexts: list[RequestContext] = []
    for m in re.finditer(r"pub\s+struct\s+(\w+Request)\b", text):
        request_name = m.group(1)
        struct_brace_idx = text.find("{", m.end())
        if struct_brace_idx < 0:
            continue

        struct_end = find_block_end(text, struct_brace_idx)
        if struct_end < 0:
            continue

        struct_text = text[m.start() : struct_end + 1]
        impl_text = ""

        impl_pattern = re.compile(rf"impl\s+{re.escape(request_name)}\s*\{{")
        impl_match = impl_pattern.search(text)
        if impl_match:
            impl_brace_idx = text.find("{", impl_match.end() - 1)
            if impl_brace_idx >= 0:
                impl_end = find_block_end(text, impl_brace_idx)
                if impl_end >= 0:
                    impl_text = text[impl_match.start() : impl_end + 1]

        contexts.append(
            RequestContext(
                name=request_name,
                line=line_of(text, m.start()),
                struct_text=struct_text,
                impl_text=impl_text,
            )
        )

    return contexts


def has_required_markers(request_context_text: str) -> bool:
    markers = [
        r"必填",
        r"不能为空",
        r"\brequired\b",
    ]
    return any(re.search(marker, request_context_text, re.IGNORECASE) for marker in markers)


def extract_if_blocks(text: str) -> list[tuple[int, str, str]]:
    blocks: list[tuple[int, str, str]] = []
    for m in re.finditer(r"\bif\s+", text):
        brace_idx = text.find("{", m.end())
        if brace_idx < 0:
            continue

        end = find_block_end(text, brace_idx)
        if end < 0:
            continue

        condition = text[m.end() : brace_idx]
        block = text[brace_idx + 1 : end]
        blocks.append((m.start(), condition, block))
    return blocks


def has_empty_check_expr(text: str) -> bool:
    return bool(
        re.search(r"\.[A-Za-z_][A-Za-z0-9_]*\s*\.\s*is_empty\s*\(\)", text)
        or re.search(r"\bis_empty\s*\(\)", text)
        or re.search(
            r"\b(?:len\s*\(\)|chars\s*\(\)\s*\.\s*count\s*\(\))\s*(?:==|<=)\s*0",
            text,
        )
        or re.search(
            r"\b0\s*(?:==|>=)\s*(?:[A-Za-z_][A-Za-z0-9_]*\s*\.\s*)?(?:len\s*\(\)|chars\s*\(\)\s*\.\s*count\s*\(\))",
            text,
        )
    )


def has_composite_required_guard(impl_text: str, if_pos: int, condition: str) -> bool:
    prior_window = impl_text[max(0, if_pos - 1200) : if_pos]
    vars_in_cond = set(re.findall(r"\b([A-Za-z_][A-Za-z0-9_]*)\b", condition))
    keywords = {"if", "let", "true", "false", "and", "or"}
    var_names = [v for v in vars_in_cond if v not in keywords and len(v) > 2]
    if not var_names:
        return False

    for var_name in var_names:
        assign_match = re.search(
            rf"let\s+{re.escape(var_name)}\s*=\s*[^;]{{0,260}};",
            prior_window,
            re.DOTALL,
        )
        if not assign_match:
            continue

        assign_expr = assign_match.group(0)
        if has_empty_check_expr(assign_expr):
            return True

    return False


def has_manual_required_validation(impl_text: str) -> bool:
    validation_error_markers = r"validation_error|validation_msg|CoreError::validation"

    for m in re.finditer(
        r"self\.[A-Za-z_][A-Za-z0-9_]*(?:\s*\.\s*(?:as_ref|as_deref)\(\))?\s*\.ok_or_else\s*\(",
        impl_text,
    ):
        snippet = impl_text[m.start() : m.start() + 400]
        if re.search(validation_error_markers, snippet):
            return True

    for m in re.finditer(
        r"if\s+[^{;\n]{0,240}self\.[^\n{;]{0,160}(?:trim\(\)\s*\.\s*)?is_empty\s*\(\)[^{;\n]*\{",
        impl_text,
        re.DOTALL,
    ):
        snippet = impl_text[m.start() : m.start() + 500]
        if re.search(validation_error_markers, snippet):
            return True

    for if_pos, condition, block in extract_if_blocks(impl_text):
        if not re.search(validation_error_markers, block):
            continue

        if has_empty_check_expr(condition):
            return True

        if has_composite_required_guard(impl_text, if_pos, condition):
            return True

    for m in re.finditer(
        r"if\s+[^{;\n]{0,240}self\.[^\n{;]{0,160}(?:len\(\)|chars\(\)\s*\.\s*count\(\))\s*(?:==|<=)\s*0[^{;\n]*\{",
        impl_text,
        re.DOTALL,
    ):
        snippet = impl_text[m.start() : m.start() + 500]
        if re.search(validation_error_markers, snippet):
            return True

    len_var_pattern = re.compile(
        r"let\s+([A-Za-z_][A-Za-z0-9_]*)\s*=\s*self\.[A-Za-z_][A-Za-z0-9_]*(?:\s*\.\s*trim\(\))?\s*\.\s*(?:len\(\)|chars\(\)\s*\.\s*count\(\))\s*;"
    )
    for m in len_var_pattern.finditer(impl_text):
        var_name = re.escape(m.group(1))
        if_match = re.search(
            rf"if\s+[^{'{'};\n]{{0,120}}\b{var_name}\b[^{'{'};\n]{{0,120}}(?:==|<=)\s*0|if\s+[^{'{'};\n]{{0,120}}0\s*(?:==|>=)\s*\b{var_name}\b",
            impl_text[m.end() : m.end() + 320],
            re.DOTALL,
        )
        if if_match:
            base = m.end()
            snippet = impl_text[base + if_match.start() : base + if_match.start() + 500]
            if re.search(validation_error_markers, snippet):
                return True

    return False


def has_known_required_validation(impl_text: str) -> bool:
    known_patterns = [
        r"validate_required!",
        r"validate_required_list!",
        r"validate_allow_empty_list!",
        r"validate_required_field",
        r"\bvalidate_required\s*\(",
        r"\bself\.validate\s*\(\)\s*\?",
        r"\b[A-Za-z_][A-Za-z0-9_]*\.validate\s*\(\)\s*\?",
    ]
    code = _mask_rust_non_code(impl_text)
    if any(re.search(pattern, code) for pattern in known_patterns):
        return True
    return has_manual_required_validation(impl_text)


def find_w001_candidate_line(text: str) -> int | None:
    if not seems_request_file(text):
        return None

    request_contexts = extract_request_contexts(text)
    for context in request_contexts:
        request_context_text = context.struct_text + "\n" + context.impl_text
        if not has_required_markers(request_context_text):
            continue

        if has_known_required_validation(context.impl_text):
            continue

        return context.line

    return None


def scan_file(path: Path, repo_root: Path) -> list[Finding]:
    findings: list[Finding] = []
    text = path.read_text(encoding="utf-8")
    rel = path.relative_to(repo_root).as_posix()

    execute_functions = extract_execute_functions(text)
    has_execute = len(execute_functions) > 0
    has_execute_with_options = bool(
        re.search(r"pub\s+async\s+fn\s+execute_with_options\s*\(", text)
    )

    if has_execute and not has_execute_with_options:
        first_execute_idx = execute_functions[0][0]
        findings.append(
            Finding(
                severity="ERROR",
                code="E001",
                path=rel,
                line=line_of(text, first_execute_idx),
                message="存在 execute() 但缺失 execute_with_options()",
            )
        )

    for execute_start, execute_body in execute_functions:
        delegated = "execute_with_options" in execute_body
        has_default_option = "RequestOption::default()" in execute_body
        if not (delegated and has_default_option):
            findings.append(
                Finding(
                    severity="ERROR",
                    code="E002",
                    path=rel,
                    line=line_of(text, execute_start),
                    message="execute() 未委托 execute_with_options(RequestOption::default())",
                )
            )

    for pos in find_transport_request_none_calls(text):
        findings.append(
            Finding(
                severity="ERROR",
                code="E003",
                path=rel,
                line=line_of(text, pos),
                message="检测到 Transport::request(..., None)，应透传 Some(option)",
            )
        )

    w001_line = find_w001_candidate_line(text)
    if w001_line is not None:
        findings.append(
            Finding(
                severity="WARN",
                code="W001",
                path=rel,
                line=w001_line,
                message="启发式判断存在必填场景，但未发现已识别的必填校验模式（如 validate_required!/list!、validate_allow_empty_list!、self.validate()?、is_empty/len==0 手工校验；请人工确认）",
            )
        )

    return findings


def print_report(findings: list[Finding], scanned_files: int) -> None:
    errors = [f for f in findings if f.severity == "ERROR"]
    warns = [f for f in findings if f.severity == "WARN"]

    print("=" * 88)
    print("Issue #41 Guardrail Report")
    print("=" * 88)
    print(f"扫描文件数: {scanned_files}")
    print(f"ERROR: {len(errors)}")
    print(f"WARN : {len(warns)}")
    print("-" * 88)

    if not findings:
        print("✅ 未发现问题")
        return

    for f in findings:
        print(f"[{f.severity}] {f.code} {f.path}:{f.line} - {f.message}")


def main() -> int:
    args = parse_args()
    repo_root: Path = args.repo_root.resolve()
    crate_names: list[str] = args.crates

    target_files = list(iter_target_files(repo_root, crate_names))
    findings: list[Finding] = []
    for file in target_files:
        findings.extend(scan_file(file, repo_root))

    print_report(findings, scanned_files=len(target_files))

    has_error = any(f.severity == "ERROR" for f in findings)
    has_warn = any(f.severity == "WARN" for f in findings)

    if has_error or (args.strict_warn and has_warn):
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
