//! 批量查询人员自定义组织变更记录
//!
//! docPath: <https://open.feishu.cn/document/corehr-v1/employee/employee-custom_org/employment_custom_org_record>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use std::sync::Arc;

/// 批量查询人员自定义组织变更记录请求。
#[derive(Debug, Clone)]
pub struct EmployeeCustomOrgEmploymentCustomOrgRecordRequest {
    config: Arc<Config>,
    /// 用户 ID 列表（必填）
    user_ids: Vec<String>,
    /// 用户 ID 类型（必填：open_id/union_id/user_id/people_corehr_id）
    user_id_type: String,
    /// 组织类型编码列表（必填）
    object_api_names: Vec<String>,
}

impl EmployeeCustomOrgEmploymentCustomOrgRecordRequest {
    /// 创建请求。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            user_ids: Vec::new(),
            user_id_type: String::new(),
            object_api_names: Vec::new(),
        }
    }

    /// 设置用户 ID 列表（必填）。
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = user_ids;
        self
    }

    /// 设置用户 ID 类型（必填：open_id/union_id/user_id/people_corehr_id）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = user_id_type.into();
        self
    }

    /// 设置组织类型编码列表（必填）。
    pub fn object_api_names(mut self, object_api_names: Vec<String>) -> Self {
        self.object_api_names = object_api_names;
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.user_ids, "user_ids 不能为空");
        validate_required!(self.user_id_type, "user_id_type 不能为空");
        validate_required!(self.object_api_names, "object_api_names 不能为空");

        let req: ApiRequest<serde_json::Value> =
            ApiRequest::get("/open-apis/corehr/v2/custom_org/employment_custom_org_record")
                .query_multi("user_ids", self.user_ids)
                .query("user_id_type", self.user_id_type)
                .query_multi("object_api_names", self.object_api_names);

        Transport::request_typed(
            req,
            &self.config,
            Some(option),
            "批量查询人员自定义组织变更记录",
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/corehr/v2/custom_org/employment_custom_org_record
    #[tokio::test]
    async fn test_employment_custom_org_record_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/corehr/v2/custom_org/employment_custom_org_record",
            ))
            .and(query_param("user_ids", "7356072152375772716"))
            .and(query_param("user_id_type", "people_corehr_id"))
            .and(query_param("object_api_names", "custom_org_01"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "custom_org_list": [
                        {
                            "custom_org_list": [
                                {
                                    "custom_org_name": { "zh_cn": "王冰", "en_us": "Bob" },
                                    "custom_org_id": "7293641346149138452",
                                    "rate": "42.98"
                                }
                            ],
                            "effective_time": "2024-06-13 00:00:00",
                            "start_reason": "自动打标",
                            "job_data_custom_org_id": "7260357352426782739",
                            "version_id": "7260357352426782749",
                            "object_api_name": "custom_org_03",
                            "user_id": "7352797725202581036"
                        }
                    ]
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

        let resp = EmployeeCustomOrgEmploymentCustomOrgRecordRequest::new(Arc::new(config))
            .user_ids(vec!["7356072152375772716".to_string()])
            .user_id_type("people_corehr_id")
            .object_api_names(vec!["custom_org_01".to_string()])
            .execute()
            .await
            .expect("请求应成功");

        assert!(resp.get("custom_org_list").is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
