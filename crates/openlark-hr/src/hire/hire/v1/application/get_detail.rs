//! 获取投递详情
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/application/get_detail>

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

use crate::hire::hire::common_models::{
    ApplicationJobInfo, ApplicationOfferInfo, ApplicationTalentInfo,
};

/// 获取投递详情请求
#[derive(Debug, Clone)]
pub struct GetDetailRequest {
    /// 配置信息
    config: Config,
    application_id: String,
}

impl GetDetailRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            application_id: String::new(),
        }
    }

    /// 设置 `application_id`。
    pub fn application_id(mut self, application_id: String) -> Self {
        self.application_id = application_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetDetailResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetDetailResponse> {
        validate_required!(self.application_id.trim(), "投递 ID 不能为空");

        let request = ApiRequest::<GetDetailResponse>::get(format!(
            "/open-apis/hire/v1/applications/{}/get_detail",
            self.application_id
        ));
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取投递详情响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 获取投递详情响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetDetailResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_id` 字段。
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_status` 字段。
    pub application_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `stage_id` 字段。
    pub stage_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `stage_name` 字段。
    pub stage_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_info` 字段。
    pub job_info: Option<ApplicationJobInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `talent_info` 字段。
    pub talent_info: Option<ApplicationTalentInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `offer_info` 字段。
    pub offer_info: Option<ApplicationOfferInfo>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for GetDetailResponse {
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

    /// 端到端：GET /open-apis/hire/v1/applications/test001/get_detail
    #[tokio::test]
    async fn test_get_detail_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/applications/test001/get_detail"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {  }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        GetDetailRequest::new(config)
            .application_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
