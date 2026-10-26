//! 查询单个用户授权
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/authorization/get_by_param>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 查询单个用户授权请求
#[derive(Debug, Clone)]
pub struct GetByParamRequest {
    /// 配置信息
    config: Config,
    /// 查询参数（官方无请求体，employment_id/user_id_type 均为查询参数）
    query: GetByParamQuery,
}

/// 查询单个用户授权查询参数
///
/// employment_id 必填；user_id_type 可选（默认 people_corehr_id）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetByParamQuery {
    /// 被授权用户的 ID，ID 类型与 user_id_type 的取值意义一致，默认为飞书人事中的 employment_id
    pub employment_id: String,
    /// 用户 ID 类型（open_id/union_id/user_id/people_corehr_id，默认 people_corehr_id）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl GetByParamRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query: GetByParamQuery {
                employment_id: String::new(),
                user_id_type: None,
            },
        }
    }

    /// 设置被授权用户的 ID
    pub fn employment_id(mut self, employment_id: impl Into<String>) -> Self {
        self.query.employment_id = employment_id.into();
        self
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.query.user_id_type = Some(user_id_type.into());
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

        validate_required!(self.query.employment_id, "employment_id 不能为空");

        let api_endpoint = FeishuPeopleApiV1::AuthorizationGetByParam;
        let GetByParamQuery {
            employment_id,
            user_id_type,
        } = self.query;
        let request = ApiRequest::<GetByParamResponse>::get(api_endpoint.to_url())
            .query("employment_id", employment_id)
            .query_opt("user_id_type", user_id_type);
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
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/corehr/v1/authorizations/get_by_param
    /// （官方无请求体，employment_id 为必填查询参数）
    #[tokio::test]
    async fn test_get_by_param_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v1/authorizations/get_by_param"))
            .and(query_param("employment_id", "6967639606963471902"))
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
            .employment_id("6967639606963471902")
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
        let query = received[0].url.query().unwrap_or_default();
        assert!(query.contains("employment_id=6967639606963471902"));
    }
}
