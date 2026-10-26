//! 查询单个人员自定义组织变更记录
//!
//! docPath: <https://open.feishu.cn/document/corehr-v1/employee/employee-custom_org/querybyid>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use std::sync::Arc;

/// 查询单个人员自定义组织变更记录请求。
#[derive(Debug, Clone)]
pub struct EmployeeCustomOrgQuerybyidRequest {
    config: Arc<Config>,
    /// 自定义组织 ID（必填）
    job_data_custom_org_id: String,
    /// 版本 ID 列表（为空时查询全部的版本记录）
    version_id: Option<Vec<String>>,
    /// 自定义组织类型编码（必填）
    object_api_name: String,
    /// 用户 ID 类型（必填：open_id/union_id/user_id/people_corehr_id）
    user_id_type: String,
}

impl EmployeeCustomOrgQuerybyidRequest {
    /// 创建请求。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            job_data_custom_org_id: String::new(),
            version_id: None,
            object_api_name: String::new(),
            user_id_type: String::new(),
        }
    }

    /// 设置自定义组织 ID（必填）。
    pub fn job_data_custom_org_id(mut self, job_data_custom_org_id: impl Into<String>) -> Self {
        self.job_data_custom_org_id = job_data_custom_org_id.into();
        self
    }

    /// 设置版本 ID 列表（为空时查询全部的版本记录）。
    pub fn version_id(mut self, version_id: Vec<String>) -> Self {
        self.version_id = Some(version_id);
        self
    }

    /// 设置自定义组织类型编码（必填）。
    pub fn object_api_name(mut self, object_api_name: impl Into<String>) -> Self {
        self.object_api_name = object_api_name.into();
        self
    }

    /// 设置用户 ID 类型（必填：open_id/union_id/user_id/people_corehr_id）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = user_id_type.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(
            self.job_data_custom_org_id,
            "job_data_custom_org_id 不能为空"
        );
        validate_required!(self.object_api_name, "object_api_name 不能为空");
        validate_required!(self.user_id_type, "user_id_type 不能为空");

        let req: ApiRequest<serde_json::Value> =
            ApiRequest::get("/open-apis/corehr/v2/custom_org/querybyid")
                .query("job_data_custom_org_id", self.job_data_custom_org_id)
                .query_multi("version_id", self.version_id.unwrap_or_default())
                .query("object_api_name", self.object_api_name)
                .query("user_id_type", self.user_id_type);

        Transport::request_typed(
            req,
            &self.config,
            Some(option),
            "查询单个人员自定义组织变更记录",
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

    /// 端到端：GET /open-apis/corehr/v2/custom_org/querybyid
    #[tokio::test]
    async fn test_querybyid_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v2/custom_org/querybyid"))
            .and(query_param("job_data_custom_org_id", "7293841029445207596"))
            .and(query_param("object_api_name", "custom_org_03"))
            .and(query_param("user_id_type", "people_corehr_id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "custom_org": [
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
                    ],
                    "user_id": "006"
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

        let resp = EmployeeCustomOrgQuerybyidRequest::new(Arc::new(config))
            .job_data_custom_org_id("7293841029445207596")
            .object_api_name("custom_org_03")
            .user_id_type("people_corehr_id")
            .execute()
            .await
            .expect("请求应成功");

        assert!(resp.get("custom_org").is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
