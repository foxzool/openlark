//! 删除外部投递
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/external_application/delete>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::ExternalApplication;

/// 删除外部投递请求
#[derive(Debug, Clone)]
pub struct DeleteRequest {
    /// 配置信息
    config: Config,
    external_application_id: String,
}

impl DeleteRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            external_application_id: String::new(),
        }
    }

    /// 设置 `external_application_id`。
    pub fn external_application_id(mut self, external_application_id: impl Into<String>) -> Self {
        self.external_application_id = external_application_id.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DeleteResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteResponse> {
        validate_required!(
            self.external_application_id.trim(),
            "external_application_id 不能为空"
        );

        let request = ApiRequest::<DeleteResponse>::delete(format!(
            "/open-apis/hire/v1/external_applications/{}",
            self.external_application_id
        ));
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "删除外部投递响应数据为空",
        )
        .await
    }
}

/// 删除外部投递响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DeleteResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `external_application_id` 字段。
    pub external_application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `result` 字段。
    pub result: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `success` 字段。
    pub success: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 外部投递信息（官方 `external_application` 容器）。
    pub external_application: Option<ExternalApplication>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for DeleteResponse {
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

    /// 端到端：DELETE /open-apis/hire/v1/external_applications/test001
    #[tokio::test]
    async fn test_delete_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/hire/v1/external_applications/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "ok",
                "data": {
                    "external_application": {
                        "id": "6989202908470446380",
                        "job_recruitment_type": 1,
                        "job_title": "高级Java",
                        "resume_source": "lagou",
                        "stage": "简历初筛",
                        "talent_id": "6960663240925956459",
                        "termination_reason": "不合适",
                        "delivery_type": 1,
                        "modify_time": 1618500278645i64,
                        "termination_type": "HR 主动终止"
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

        let resp = DeleteRequest::new(config)
            .external_application_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");
        let ext_app = resp
            .external_application
            .as_ref()
            .expect("external_application 应被类型化解析");
        assert_eq!(ext_app.id.as_deref(), Some("6989202908470446380"));
        assert_eq!(ext_app.job_title.as_deref(), Some("高级Java"));
        assert_eq!(ext_app.resume_source.as_deref(), Some("lagou"));
        assert_eq!(ext_app.stage.as_deref(), Some("简历初筛"));
        assert_eq!(ext_app.termination_reason.as_deref(), Some("不合适"));
        assert_eq!(ext_app.delivery_type, Some(1));
        assert_eq!(ext_app.modify_time, Some(1618500278645));
        assert_eq!(ext_app.termination_type.as_deref(), Some("HR 主动终止"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
