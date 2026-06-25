# OpenLark API Skill References

`SKILL.md` 已压缩为 200 行速查，本目录用于“按需加载”。

## 文件说明

- `standard-example.md`：标准示例（推荐直接照抄结构再改字段/路径）
- `file-layout.md`：落盘路径口径、目录反查方法
- `csv-mapping.md`：`api_list_export.csv` 的最小提取规则（method/path + 落盘信息）

## docPath 抓取脚本

- 在线抓取（需要网络）：`python3 .agents/skills/openlark-api/scripts/fetch_docpath.py "<docPath>" --format md --out /tmp/doc.md`
- 离线解析（无网络/受限环境）：先把页面保存为 HTML，再用 `--html-file` 解析：
  - `python3 .agents/skills/openlark-api/scripts/fetch_docpath.py "<docPath>" --html-file /path/to/page.html --format md --out /tmp/doc.md`
  - 或直接从 stdin 输入 HTML：`cat /path/to/page.html | python3 .agents/skills/openlark-api/scripts/fetch_docpath.py "<docPath>" --stdin --format md`
