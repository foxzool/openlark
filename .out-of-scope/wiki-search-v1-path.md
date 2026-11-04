# 搜索 Wiki 不改到 v1

不要把「搜索 Wiki」（id `6927492937392324635`）的 HTTP 路径从 `POST /open-apis/wiki/v2/nodes/search` 改成 `POST /open-apis/wiki/v1/nodes/search`。

SDK 与仓库基线 CSV 已经调用 v2。这与中文文档一致，不是待补的缺口。

## 为什么拒绝

变动检测报告里的 v1 不是文档正文，也不是 `schema.apiSchema.path`。

`tools/export_server_api_list.py` 写 `url` 的顺序是：

1. `document/get_detail` 的 `schema.apiSchema.path`
2. 目录树 `apiSummary.apiPath`
3. 从 `content` 解析 HTTP URL

`get_detail` 不带语言参数（`http_get_doc_detail`）。

2026-11-03 对这条 API 的核对：

- `api_catalog` 的 `apiSummary.apiPath` 是 `/open-apis/wiki/v1/nodes/search`，`apiIdentity.version` 是 `v1`。
- `get_detail?fullPath=/ukTMukTMukTM/uEzN0YjLxcDN24SM3QjN/search_wiki` 的 `data` 没有 `schema`。正文里的 HTTP URL 是 `https://open.feishu.cn/open-apis/wiki/v2/nodes/search`。
- `schema` 为空时，导出器采用第 2 步，记录 v1，不会读到正文里的 v2。
- 中文文档（`lang=zh-CN`）和 Lark 国际文档也是 v2。不带语言的英文 Feishu markdown 是 v1，与 `apiPath` 相同，与中文正文不同。

因此「目录 url 已改为 v1」只说明 `apiSummary.apiPath` 与文档正文不一致。在文档改为 v1 之前，不跟随 `apiPath` 改 Rust 契约或基线 CSV。

维持基线 v2 时，目录巡检会继续为这一字段差异开 Issue。这是已知噪声，直到文档与 `apiSummary` 一致，或有意调整导出/比较顺序。

## 先前请求

- #762：只报告这一条 `url` 变化
- #771：同一条 `url` 变化，另外还有 3 个新增千人千面菜单 API。这 3 个 API 已补进 SDK 和基线 CSV。Wiki 的 `url` 仍保持 v2，不随 `apiPath` 改。
