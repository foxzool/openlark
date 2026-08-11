import json
import os
import sys
import time


mode = os.environ.get("OPENLARK_TEST_RENDERED_WORKER_MODE", "success")
request_count = 0

if mode == "nonzero":
    raise SystemExit(7)

for line in sys.stdin:
    request = json.loads(line)
    if request.get("type") == "shutdown":
        break
    request_count += 1
    if mode == "timeout":
        time.sleep(5)
        continue
    if mode == "missing":
        raise SystemExit(0)
    if mode == "malformed":
        print("not-json", flush=True)
        continue
    if mode == "unavailable":
        response = {
            "id": request["id"],
            "status": "unavailable",
            "code": "document_not_found",
        }
    else:
        if mode == "unhealthy":
            content = "404 Not Found"
        elif mode == "innertext":
            content = (
                ("Rendered Feishu API document content.\n" * 20)
                + "HTTP Method\n"
                + "POST\n"
                + "HTTP URL\n"
                + "https://open.feishu.cn/open-apis/rendered/innertext\n"
                + "Request header\n"
                + "Authorization\n"
                + "string\n"
                + "Yes\n"
                + "Bearer tenant_access_token or user_access_token\n"
                + "Path parameters\n"
                + "Path parameters\n"
                + "app_token\n"
                + "string\n"
                + "Official path field description\n"
                + "Request body\n"
                + "Request body\n"
                + "rendered_only\n"
                + "string\n"
                + "Yes\n"
                + "Official field description\n"
                + "Request example\n"
                + "Response body example\n"
                + "Response body example\n"
                + '{"code": 0, "data": {"record_id": "rec"}}\n'
                + "Error code\n"
            )
        elif mode == "innertext_query_once":
            content = (
                ("Rendered Feishu API document content.\n" * 20)
                + "Query parameters\n"
                + "page_token\n"
                + "string\n"
                + "No\n"
                + "Pagination cursor\n"
                + "Request example\n"
                + "Response body example\n"
                + "Response body example\n"
                + '{"code": 0, "data": {}}\n'
                + "Error code\n"
            )
        elif mode == "innertext_empty_query":
            content = (
                ("Rendered Feishu API document content.\n" * 20)
                + "Query parameters\n"
                + "Query parameters\n"
                + "Request example\n"
            )
        elif mode == "innertext_missing_fields":
            content = (
                ("Rendered Feishu API document content.\n" * 20)
                + "Response body example\n"
                + "Response body example\n"
                + "unsupported rendered content\n"
                + "Error code\n"
            )
        else:
            content = (
                "# Official API Documentation\n"
                "## Endpoint\n"
                f"POST /rendered/{request_count}\n"
                "## Request Fields\n"
                "| Path | Location | Required | Type |\n"
                "| rendered_only | request_body | unknown | string |\n"
            )
        response = {
            "id": request["id"],
            "status": "ok",
            "source_uri": request["url"],
            "content": content,
        }
    print(json.dumps(response), flush=True)
