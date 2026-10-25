//! 查询单个用户授权
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/authorization/get_by_param>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 查询单个用户授权请求
#[derive(Debug, Clone)]
pub struct GetByParamRequest {
    /// 配置信息
    config: Config,
    body: Value,
}

impl GetByParamRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: Value::Object(serde_json::Map::new()),
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: Value) -> Self {
        self.body = body;
        self
    }

    /// 向请求体添加字段。
    pub fn field(mut self, key: impl Into<String>, value: Value) -> Self {
        if !self.body.is_object() {
            self.body = Value::Object(serde_json::Map::new());
        }
        if let Some(body) = self.body.as_object_mut() {
            body.insert(key.into(), value);
        }
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetByParamResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetByParamResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        let api_endpoint = FeishuPeopleApiV1::AuthorizationGetByParam;
        let request = ApiRequest::<GetByParamResponse>::get(api_endpoint.to_url()).body(self.body);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询单个用户授权响应数据为空",
        )
        .await
    }
}

/// 查询单个用户授权响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetByParamResponse {
    /// 角色授权信息（官方键名为 role_authorization）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_authorization: Option<super::query::RoleAuthorization>,
}

impl ApiResponseTrait for GetByParamResponse {
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

    /// 端到端：GET /open-apis/corehr/v1/authorizations/get_by_param
    #[tokio::test]
    async fn test_get_by_param_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v1/authorizations/get_by_param"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "role_authorization": {
                        "employment_id": "6967639606963471902",
                        "permission_detail_list": [
                            {
                                "role": {
                                    "id": "7034393015968122400",
                                    "code": "department_manager",
                                    "name": { "zh_cn": "部门经理", "en_us": "department manager" },
                                    "description": { "zh_cn": "描述", "en_us": "description" },
                                    "org_truncation": [
                                        { "org_key": "department", "type": 0, "depth": 0 }
                                    ]
                                },
                                "assigned_organization_list": [
                                    [
                                        {
                                            "org_key": "department",
                                            "org_name": { "zh_cn": "部门", "en_us": "department" },
                                            "org_id_list": ["6967639606963471902"]
                                        }
                                    ]
                                ],
                                "update_time": "1720584039"
                            }
                        ]
                    }
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

        let resp = GetByParamRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let authorization = resp
            .role_authorization
            .as_ref()
            .expect("role_authorization 应解码");
        assert_eq!(
            authorization.employment_id.as_deref(),
            Some("6967639606963471902")
        );
        let details = authorization
            .permission_detail_list
            .as_ref()
            .expect("permission_detail_list 应解码");
        assert_eq!(details.len(), 1);
        let role = details[0].role.as_ref().expect("role 应解码");
        assert_eq!(role.id.as_deref(), Some("7034393015968122400"));
        let truncation = role.org_truncation.as_ref().expect("org_truncation 应解码");
        assert_eq!(truncation[0].depth, Some(0));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
