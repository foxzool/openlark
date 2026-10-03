# OpenLark API Skill References

`SKILL.md` 已压缩为速查，本目录用于“按需加载”。

## 文件说明

- `standard-example.md`：标准示例（推荐直接照抄结构再改字段/路径）
- `file-layout.md`：落盘路径口径、目录反查方法
- `csv-mapping.md`：`api_list_export.csv` 的最小提取规则（method/path + 落盘信息）

## 官方文档抓取（唯一在线入口）

飞书文档是 SPA。**在线抓取必须用 playwright**：

```bash
# 推荐：按 CSV api-id（脚本内用 fullPath）
node .agents/skills/openlark-api-field-verify/scripts/fetch_doc.js \
  --from-csv <API_ID> --out /tmp/doc.txt

# 或：https://open.feishu.cn + CSV fullPath
node .agents/skills/openlark-api-field-verify/scripts/fetch_doc.js \
  "https://open.feishu.cn${FULL_PATH}" /tmp/doc.txt
```

完整流程与字段解析见 `Skill(openlark-api-field-verify)`。实现后核对门禁：

```bash
python3 tools/verify_api_fields.py --api-id <API_ID> --fetch-docs
```

## 离线 HTML 解析（deprecated 脚本，仅兜底）

`scripts/fetch_docpath.py` **禁止用于在线抓取**（SPA 常返回空壳）。仅当已有本地 HTML 时：

```bash
python3 .agents/skills/openlark-api/scripts/fetch_docpath.py unused \
  --html-file /path/to/page.html --format md --out /tmp/doc.md
```
