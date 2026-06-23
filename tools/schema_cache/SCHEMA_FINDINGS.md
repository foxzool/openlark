# apiSchema 真实结构发现（Step 0 勘察结论）

基于 8 个代表性 API 的真实 `get_detail` payload（见 `samples/`，2026-06 勘察）。
**推翻了早期基于测试 fixture 的判断**——飞书 apiSchema 信息极其丰富。

## 顶层结构

```
payload.data.schema.apiSchema = {
  httpMethod, path, id, domain,
  parameters,        # ← query/path 参数（结构化，含 options 枚举）
  requestBody,       # → content.application/json.schema.properties (list 形式)
  responses["200"],  # → content.application/json.schema.properties (list，含 data)
  security,          # ← Token 类型 + 权限范围 + 限流
  ...
}
```

## 关键发现与决策

| 维度 | 真实情况 | 决策 |
|---|---|---|
| `properties` 形式 | **list**（`[{name,required,type,description,example,options,...}]`） | parser 主路径 list，dict 作 fallback（`official.extract_schema_properties` 已支持两种） |
| `$ref` | **0 次**（全内联） | **IR 不实现 $ref**（简化） |
| `enum` | 0 次，但用 **`options:[{name,value,description}]` + `enumName`** 表达枚举 | enum 提取认 `options`；MVP 先生成 String+注释，enum 生成留增强 |
| `items` | 频繁出现，带元素 schema | `TypeArray` 递归 ✅ |
| 嵌套 object | 有（contact/user/create 深度=3） | 生成嵌套 `StructDef`，`MAX_STRUCT_DEPTH=3` |
| `format` | `int32` / **`user_id_type`**（飞书枚举类型）/ `date-time`(少见) | PRIMITIVE_MAP + `user_id_type` 当 String（其 options 决定可选值） |
| `parameters` | **有**（`in:query/path` + `schema` + `options` + `description`） | **MVP 纳入 query/path param 生成**（对标 create.rs 的 receive_id_type 必需） |
| `description` | 45-227 次/API（含中文） | G6 文案数据齐全，可生成 doc comment |
| `maxLength`/`pattern`/`maximum`/`minimum` | 都有 | 校验约束数据齐全 |
| `security.supportedAccessToken` | **有**（`["tenant_access_token","user_access_token"]`） | **G5 Token 类型可自动判定**（之前的高风险点解除） |
| response.data 子结构 | 完整（list 形式 properties） | typed response 数据齐全（MVP 仍先 Value，IR 完整解析存起来） |

## property 结构样本（im/message/create requestBody）

```json
{
  "name": "receive_id", "required": true, "type": "string",
  "description": "消息接收者的 ID，ID 类型与查询参数 receive_id_type 的取值一致...",
  "example": "ou_xxx", "options": []
}
```

## parameters 结构样本（query 枚举参数）

```json
{
  "in": "query",
  "schema": {
    "type": "string", "format": "user_id_type", "default": "open_id",
    "options": [{"name":"open_id","value":"open_id","description":"..."}, ...]
  }
}
```

## response.data 定位

`responses["200"].content["application/json"].schema.properties` 是 list，
找 `name=="data"` 的元素 → 其 `properties`（list）是 data 子字段。
data property 还带 `objectName`（如 `CreateMessageResp`，可作为 typed struct 命名参考）。

## 对 MVP 的影响

- **纳入 query/path param 生成**（计划原留 G7，但数据齐全且对标 create.rs 必需）。
- IR 层**完整解析所有维度**（含 security/options/maxLength），渲染器 MVP 只消费子集
  （body_struct + query/path param + required 校验），后续 G5/G6/typed response/enum 只扩渲染器。
- `user_id_type` 类 query param：MVP 生成 `Option<String>` + setter（不生成 enum），闭环校验只比字段名能过。
- $ref 分支可直接删除（不会触发）。
