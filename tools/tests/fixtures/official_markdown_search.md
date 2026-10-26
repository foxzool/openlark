# 批量获取数据源

批量获取创建的数据源信息。

## 请求

基本 | &nbsp;
---|---
HTTP URL | https://open.feishu.cn/open-apis/search/v2/data_sources
HTTP Method | GET
支持的应用类型 | Custom App、Store App
权限要求<br>**调用该 API 所需的权限。开启其中任意一项权限即可调用**<br>开启任一权限即可 | 查询、创建、修改和删除自定义搜索数据源、数据范式或数据项(search:data_source)<br>查询自定义搜索数据源、数据范式或数据项(search:data_source:readonly)

### 请求头

名称 | 类型 | 必填 | 描述
---|---|---|---
Authorization | string | 是 | `tenant_access_token`<br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use)

### 查询参数

名称 | 类型 | 必填 | 描述
---|---|---|---
view | int | 否 | 回包数据格式，0-全量数据；1-摘要数据。<br>**注**：摘要数据仅包含"id"，"name"，"state"。<br>**示例值**：0<br>**可选值有**：<br>- 0：全量数据<br>- 1：摘要数据
page_size | int | 否 | 分页大小<br>**示例值**：10<br>**数据校验规则**：<br>- 最大值：`50`
page_token | string | 否 | 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果<br>**示例值**："PxZFma9OIRhdBlT/dOYNiu2Ro8F2WAhcby7OhOijfljZ"

## 响应

### 响应体

名称 | 类型 | 描述
---|---|---
code | int | 错误码，非 0 表示失败
msg | string | 错误描述
data | \- | \-
has_more | boolean | 是否还有更多项
page_token | string | 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
items | data_source\[\] | 数据源中的数据记录
id | string | 数据源的唯一标识
name | string | data_source的展示名称
state | int | 数据源状态，0-已上线，1-未上线。如果未填，默认是未上线状态。<br>**可选值有**：<br>- 0：已上线<br>- 1：未上线
description | string | 对于数据源的描述
create_time | string | 创建时间，使用Unix时间戳，单位为“秒”
update_time | string | 更新时间，使用Unix时间戳，单位为“秒”
is_exceed_quota | boolean | 是否超限
icon_url | string | 数据源在 search tab 上的展示图标路径，建议使用png或jpeg格式，否则可能无法在客户端正常展示
template | string | 数据源采用的展示模版名称
searchable_fields | string\[\] | 【已废弃，如有定制需要请使用“数据范式”接口】描述哪些字段可以被搜索
i18n_name | i18n_meta | 数据源的国际化展示名称
zh_cn | string | 国际化字段：中文
en_us | string | 国际化字段：英文
ja_jp | string | 国际化字段：日文
i18n_description | i18n_meta | 数据源的国际化描述
zh_cn | string | 国际化字段：中文
en_us | string | 国际化字段：英文
ja_jp | string | 国际化字段：日文
schema_id | string | 数据源关联的 schema 标识
app_id | string | datasource对应的开放平台应用id

### 响应体示例
```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "has_more": true,
        "page_token": "NqjxxDD6oV574VOicGKaK/IETyZStu8897k/f4qY",
        "items": [
            {
                "id": "6953903108179099667",
                "name": "客服工单",
                "state": 0,
                "description": "搜索客服工单数据",
                "create_time": "1618829192",
                "update_time": "1618829192",
                "is_exceed_quota": true,
                "icon_url": "https://www.xxx.com/open.jpg",
                "template": "search_common_card",
                "searchable_fields": [
                    "field1"
                ],
                "i18n_name": {
                    "zh_cn": "任务",
                    "en_us": "TODO",
                    "ja_jp": "タスク"
                },
                "i18n_description": {
                    "zh_cn": "任务",
                    "en_us": "TODO",
                    "ja_jp": "タスク"
                },
                "schema_id": "custom_schema",
                "app_id": "cli_a1306bed4738d01b"
            }
        ]
    }
}
```

### 错误码

HTTP状态码 | 错误码 | 描述 | 排查建议
---|---|---|---
500 | 1270001 | 系统内部错误 | 联系系统开发人员协助定位
400 | 1270002 | 参数错误 | 根据错误信息和文档排查非法参数
400 | 1270005 | 该功能仅对旗舰版可用 | 请联系销售人员升级套餐以使用此高级功能
400 | 1271001 | 非法 page_token | page_token 无法解析，看是否填写正确
500 | 1271002 | 生成 page_token 失败 | 系统生成 page_token 失败。如果重试后仍然无法成功，请联系系统开发人员

