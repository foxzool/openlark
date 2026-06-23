#!/usr/bin/env python3
"""一次性勘察脚本：拉取若干代表性 API 的真实 apiSchema payload 落盘。

Step 0 用途：揭晓飞书 get_detail 接口返回的 apiSchema 实际包含多少结构化信息
（enum / items / 嵌套 properties / $ref / 顶层 parameters / format / response.data 结构），
从而决定 Codegen IR 的 TypeExpr 支持范围。

复用 tools.api_contracts.official 的 fetch_detail_payload / load_api_identities，
不触网之外的副作用，纯只读勘察。产物落 samples/<api_id>.json（已 gitignore）。

运行：python3 tools/schema_cache/dump_samples.py
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from tools.api_contracts.official import fetch_detail_payload, load_api_identities  # noqa: E402

CSV_PATH = REPO_ROOT / "api_list_export.csv"
OUT_DIR = Path(__file__).resolve().parent / "samples"

# (api_id, 标签, 探查目的)。覆盖 POST body / GET query / GET path / POST path+body /
# 嵌套 object / 深嵌套 / 跨域。
CANDIDATES: list[tuple[str, str, str]] = [
    ("6946222931479527425", "im/message/create", "POST body + query(receive_id_type)，对标 create.rs"),
    ("6946222931479560193", "im/message/list", "GET + 分页 query + 可能 array response"),
    ("6946222929790451740", "im/message/get", "GET + path param(:message_id)"),
    ("6946222929790500892", "im/message/reply", "POST + path param + body"),
    ("6943913881476857883", "contact/department/create", "POST + 嵌套 object body"),
    ("6943913881476988955", "contact/department/get", "GET + path param(:department_id)"),
    ("6943913881476939803", "contact/user/create", "POST + 深嵌套 object 探针（user 字段多）"),
    ("7270433540692639747", "moments/post/get", "跨域（moments）+ GET"),
]


def main() -> int:
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    identities = load_api_identities(CSV_PATH, skip_old_versions=False)
    by_id = {a.api_id: a for a in identities}

    summary: list[dict] = []
    for api_id, label, purpose in CANDIDATES:
        api = by_id.get(api_id)
        if api is None:
            summary.append({"api_id": api_id, "label": label, "ok": False,
                            "error": "api_id 不在 CSV（可能 old 版本被跳过或 id 变更）"})
            print(f"[SKIP ] {label:<28} {api_id}  不在 CSV")
            continue
        try:
            payload = fetch_detail_payload(api, timeout=30, retries=2)
            out_path = OUT_DIR / f"{api_id}.json"
            out_path.write_text(json.dumps(payload, ensure_ascii=False, indent=2), encoding="utf-8")
            schema_keys = list((payload.get("data", {}).get("schema", {}).get("apiSchema") or {}).keys())
            summary.append({"api_id": api_id, "label": label, "ok": True,
                            "full_path": api.full_path, "url": api.url,
                            "api_schema_keys": schema_keys,
                            "bytes": out_path.stat().st_size})
            print(f"[DUMP ] {label:<28} {api_id}  -> {out_path.name}  ({out_path.stat().st_size} B)")
        except Exception as exc:  # noqa: BLE001 - 勘察脚本要继续跑完
            summary.append({"api_id": api_id, "label": label, "ok": False, "error": str(exc)})
            print(f"[ERROR] {label:<28} {api_id}  {exc}")

    (OUT_DIR / "_summary.json").write_text(
        json.dumps(summary, ensure_ascii=False, indent=2), encoding="utf-8")
    ok = sum(1 for s in summary if s.get("ok"))
    print(f"\n完成 {ok}/{len(CANDIDATES)}，产物在 {OUT_DIR}/")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
