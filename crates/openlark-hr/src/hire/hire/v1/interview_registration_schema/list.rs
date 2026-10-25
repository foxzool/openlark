//! 获取面试登记表列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/interview_registration_schema/list>

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

/// `ListRequestBody`。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_token: Option<String>,
}

/// 获取面试登记表列表请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 配置信息
    config: Config,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置分页大小。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
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

        if let Some(size) = self.page_size
            && !(1..=100).contains(&size)
        {
            return Err(openlark_core::error::validation_error(
                "分页大小超出范围",
                "page_size 必须在 1-100 之间",
            ));
        }

        let api_endpoint = HireApiV1::InterviewRegistrationSchemaList;
        let request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());
        let request_body = ListRequestBody {
            page_size: self.page_size,
            page_token: self.page_token,
        };
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取面试登记表列表响应数据为空",
        )
        .await
    }
}

/// 登记表选项（官方 `common_schema_option`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RegistrationSchemaOption {
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

/// 登记表配置信息（官方 `common_schema_config`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RegistrationSchemaConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 选项信息，仅在类型为单选、多选时有值。
    pub options: Option<Vec<RegistrationSchemaOption>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 登记表模块/字段信息（官方 `common_schema_setting`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RegistrationSchemaSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 字段类型（模块信息中固定返回 11）。
    pub object_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 配置信息。
    pub config: Option<RegistrationSchemaConfig>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 登记表模块（官方 `common_schema`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RegistrationSchemaObject {
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
    pub setting: Option<RegistrationSchemaSetting>,
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
    pub children_list: Option<Vec<RegistrationSchemaChild>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 登记表字段（官方 `common_schema_child`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RegistrationSchemaChild {
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
    pub setting: Option<RegistrationSchemaSetting>,
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

/// 面试登记表（官方 `interview_registration_schema`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewRegistrationSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试登记表 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试登记表名称。
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 是否用作全局面试登记表。
    pub is_used_as_interview: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 模块列表。
    pub object_list: Option<Vec<RegistrationSchemaObject>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 获取面试登记表列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListResponse {
    #[serde(default)]
    /// 面试登记表列表。
    pub items: Vec<InterviewRegistrationSchema>,
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

    /// 端到端：GET /open-apis/hire/v1/interview_registration_schemas
    #[tokio::test]
    async fn test_list_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/interview_registration_schemas"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "ok", "data": {
                    "items": [
                        {
                            "id": "7044739584859326764",
                            "name": "默认面试登记表",
                            "is_used_as_interview": true,
                            "object_list": [
                                {
                                    "id": "6949805467799537964",
                                    "name": { "zh_cn": "登记表模块", "en_us": "interview module" },
                                    "description": { "zh_cn": "用于面试登记的模块", "en_us": "module used for interview" },
                                    "setting": {
                                        "object_type": 11,
                                        "config": {
                                            "options": [
                                                {
                                                    "key": "test",
                                                    "name": { "zh_cn": "模块选项", "en_us": "module option" },
                                                    "description": { "zh_cn": "用于模块的选项", "en_us": "option for module" },
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
                                            "name": { "zh_cn": "登记表字段", "en_us": "interview registration field" },
                                            "description": { "zh_cn": "用于登记表的字段", "en_us": "field for interview registration" },
                                            "setting": {
                                                "object_type": 1,
                                                "config": {
                                                    "options": [
                                                        {
                                                            "key": "1",
                                                            "name": { "zh_cn": "选项A", "en_us": "option A" },
                                                            "description": { "zh_cn": "字段选项", "en_us": "option for field" },
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
                    "has_more": true,
                    "page_token": "eyJvZmZzZXQiOjEwLCJ0aW1lc3RhbXAiOjE2Mjc1NTUyMjM2NzIsImlkIjpudWxsfQ=="
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
        assert_eq!(resp.items.len(), 1);
        let item = &resp.items[0];
        assert_eq!(item.id.as_deref(), Some("7044739584859326764"));
        assert_eq!(item.name.as_deref(), Some("默认面试登记表"));
        assert_eq!(item.is_used_as_interview, Some(true));
        let object = item
            .object_list
            .as_ref()
            .expect("object_list 应存在")
            .first()
            .unwrap();
        assert_eq!(object.setting.as_ref().unwrap().object_type, Some(11));
        let child = object.children_list.as_ref().unwrap().first().unwrap();
        assert_eq!(child.parent_id.as_deref(), Some("6949805467799537964"));
        assert_eq!(
            child
                .setting
                .as_ref()
                .unwrap()
                .config
                .as_ref()
                .unwrap()
                .options
                .as_ref()
                .unwrap()
                .len(),
            1
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
