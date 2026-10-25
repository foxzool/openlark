//! 获取申请表模板列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/portal_apply_schema/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::common::shared_models::I18nText;

/// 申请表模板选项（官方 `common_schema_option`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PortalApplySchemaOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 选项 ID。
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 选项名称。
    pub name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 选项描述。
    pub description: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 是否启用：1 已启用 / 2 已停用。
    pub active_status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 申请表模板配置（官方 `common_schema_config`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PortalApplySchemaConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 选项信息（字段类型为单选/多选时返回）。
    pub options: Option<Vec<PortalApplySchemaOption>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 申请表模板字段信息（官方 `common_schema_setting`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PortalApplySchemaSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 字段类型：1 单行文本 / 2 多行文本 / 3 单选 / 4 多选 / 5 日期 / 6 月份选择 / 7 年份选择 / 8 时间段 / 9 数字 / 10 默认字段 / 11 模块 / 13 附件。
    pub object_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 配置信息。
    pub config: Option<PortalApplySchemaConfig>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 申请表模板字段（官方 `common_schema_child`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PortalApplySchemaChild {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 字段 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 字段名称。
    pub name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 字段描述。
    pub description: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 字段信息。
    pub setting: Option<PortalApplySchemaSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 所属模块 ID。
    pub parent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 是否是自定义字段。
    pub is_customized: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 是否必填。
    pub is_required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 是否可见。
    pub is_visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 是否启用：1 已启用 / 2 已停用。
    pub active_status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 申请表模板模块（官方 `common_schema`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PortalApplySchemaObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 模块 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 模块名称。
    pub name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 模块描述。
    pub description: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 模块信息。
    pub setting: Option<PortalApplySchemaSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 是否是自定义模块。
    pub is_customized: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 是否必填。
    pub is_required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 是否可见。
    pub is_visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 是否启用：1 已启用 / 2 已停用。
    pub active_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 字段列表。
    pub children_list: Option<Vec<PortalApplySchemaChild>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 申请表模板（官方 `registration_schema`，官网申请表模板列表元素）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PortalApplySchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 申请表模板 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 申请表模板名称。
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 申请表模板适用场景：5 面试登记表 / 6 入职登记表 / 14 信息更新登记表。
    pub scenarios: Option<Vec<i32>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 模块列表。
    pub objects: Option<Vec<PortalApplySchemaObject>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 获取申请表模板列表请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 配置信息
    config: Config,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        use crate::common::api_endpoints::HireApiV1;

        let api_endpoint = HireApiV1::PortalApplySchemaList;
        let request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取申请表模板列表响应数据为空",
        )
        .await
    }
}

/// 获取申请表模板列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListResponse {
    #[serde(default)]
    /// 结果项列表。
    pub items: Vec<PortalApplySchema>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 下一页分页标记。
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 是否还有更多结果。
    pub has_more: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for ListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/hire/v1/portal_apply_schemas
    #[tokio::test]
    async fn test_list_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/portal_apply_schemas"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "ok",
                "data": {
                    "items": [
                        {
                            "id": "7044739584859326764",
                            "name": "默认官网申请表",
                            "scenarios": [14],
                            "objects": [
                                {
                                    "id": "6949805467799537964",
                                    "name": { "zh_cn": "基本信息", "en_us": "Basic Information" },
                                    "description": {
                                        "zh_cn": "候选人的基本信息",
                                        "en_us": "The basic information of the candidate"
                                    },
                                    "setting": {
                                        "object_type": 1,
                                        "config": {
                                            "options": [
                                                {
                                                    "key": "Option1",
                                                    "name": { "zh_cn": "选项一", "en_us": "Option1" },
                                                    "description": { "zh_cn": "第一个选项", "en_us": "First Option" },
                                                    "active_status": 1
                                                }
                                            ]
                                        }
                                    },
                                    "is_customized": true,
                                    "is_required": false,
                                    "is_visible": true,
                                    "active_status": 1,
                                    "children_list": [
                                        {
                                            "id": "6949805467799537964",
                                            "name": { "zh_cn": "自定义字段", "en_us": "Custom Field" },
                                            "description": {
                                                "zh_cn": "第一个自定义字段",
                                                "en_us": "First Custom Field"
                                            },
                                            "setting": {
                                                "object_type": 1,
                                                "config": {
                                                    "options": [
                                                        {
                                                            "key": "Option1",
                                                            "name": { "zh_cn": "选项一", "en_us": "Option1" },
                                                            "description": { "zh_cn": "第一个选项", "en_us": "First Option" },
                                                            "active_status": 1
                                                        }
                                                    ]
                                                }
                                            },
                                            "parent_id": "6949805467799537964",
                                            "is_customized": true,
                                            "is_required": false,
                                            "is_visible": true,
                                            "active_status": 1
                                        }
                                    ]
                                }
                            ]
                        }
                    ],
                    "page_token": "6930815272790114324",
                    "has_more": true
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = ListRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");
        let item = resp.items.first().expect("应返回 items 列表");
        assert_eq!(item.id.as_deref(), Some("7044739584859326764"));
        assert_eq!(item.scenarios.as_deref(), Some(&[14][..]));
        let object = item
            .objects
            .as_ref()
            .and_then(|l| l.first())
            .expect("应返回模块列表");
        assert_eq!(
            object.name.as_ref().and_then(|n| n.zh_cn.as_deref()),
            Some("基本信息")
        );
        let child = object
            .children_list
            .as_ref()
            .and_then(|l| l.first())
            .expect("应返回字段列表");
        assert_eq!(child.parent_id.as_deref(), Some("6949805467799537964"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
