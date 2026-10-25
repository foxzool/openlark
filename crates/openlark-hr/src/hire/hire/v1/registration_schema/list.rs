//! 获取信息登记表列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/registration_schema/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::CommonSchema;

/// 信息登记表（官方 `registration_schema`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RegistrationSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 信息登记表 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 信息登记表名称。
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 登记表适用场景：5 面试登记表 / 6 入职登记表 / 14 信息更新登记表。
    pub scenarios: Option<Vec<i32>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 模块列表（官方 `common_schema[]`）。
    pub objects: Option<Vec<CommonSchema>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 获取信息登记表列表请求
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

        let api_endpoint = HireApiV1::RegistrationSchemaList;
        let request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取信息登记表列表响应数据为空",
        )
        .await
    }
}

/// 获取信息登记表列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListResponse {
    #[serde(default)]
    /// 结果项列表（官方 `registration_schema[]`）。
    pub items: Vec<RegistrationSchema>,
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

    /// 端到端：GET /open-apis/hire/v1/registration_schemas
    #[tokio::test]
    async fn test_list_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/registration_schemas"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "SUCCESS",
                "data": {
                    "items": [
                        {
                            "id": "6930815272790114324",
                            "name": "默认登记表",
                            "scenarios": [5],
                            "objects": [
                                {
                                    "id": "6930815272790114324",
                                    "name": { "zh_cn": "基本信息", "en_us": "Basic Info" },
                                    "object_type": 11,
                                    "is_customized": false,
                                    "is_required": true,
                                    "is_visible": true,
                                    "active_status": 1
                                }
                            ]
                        }
                    ],
                    "has_more": false
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
        assert_eq!(item.id.as_deref(), Some("6930815272790114324"));
        assert_eq!(item.name.as_deref(), Some("默认登记表"));
        assert_eq!(item.scenarios.as_deref(), Some(&[5][..]));
        let objects = item.objects.as_ref().expect("objects 应被类型化解析");
        assert_eq!(objects.len(), 1);
        assert_eq!(objects[0].id.as_deref(), Some("6930815272790114324"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
