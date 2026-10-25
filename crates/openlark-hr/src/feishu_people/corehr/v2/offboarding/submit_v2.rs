//! 操作员工离职
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/offboarding/submit_v2>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 操作员工离职请求
#[derive(Debug, Clone)]
pub struct SubmitV2Request {
    /// 配置信息
    config: Config,
    request_body: Option<Value>,
}

impl SubmitV2Request {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request_body: None,
        }
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SubmitV2Response> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SubmitV2Response> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        let api_endpoint = FeishuPeopleApiV2::OffboardingSubmitV2;
        let mut request = ApiRequest::<SubmitV2Response>::post(api_endpoint.to_url());

        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "操作员工离职响应数据为空",
        )
        .await
    }
}

/// 操作员工离职响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubmitV2Response {
    /// 离职记录 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_id: Option<String>,
    /// 雇员 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 离职原因
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_reason_unique_identifier: Option<String>,
    /// 离职日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_date: Option<String>,
    /// 离职原因说明
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_reason_explanation: Option<String>,
    /// 是否加入离职屏蔽名单
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_block_list: Option<bool>,
    /// 屏蔽原因
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_reason: Option<String>,
    /// 屏蔽原因说明
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_reason_explanation: Option<String>,
    /// 创建时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// 离职是否保留飞书账号
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retain_account: Option<bool>,
}

impl ApiResponseTrait for SubmitV2Response {
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

    /// 端到端：POST /open-apis/corehr/v2/offboardings/submit_v2
    #[tokio::test]
    async fn test_submit_v2_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/offboardings/submit_v2"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "offboarding_id": "7095671727698478604",
                    "employment_id": "6982509313466189342",
                    "offboarding_reason_unique_identifier": "reason_for_offboarding_option8",
                    "offboarding_date": "2022-05-18",
                    "offboarding_reason_explanation": "离职原因说明",
                    "add_block_list": false,
                    "block_reason": "红线",
                    "block_reason_explanation": "xx 年 xx 月 xx 日因 xx 原因红线",
                    "created_time": "2022-05-09 17:50:17",
                    "retain_account": false
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

        let resp = SubmitV2Request::new(config)
            .execute()
            .await
            .expect("请求应成功");

        assert_eq!(resp.offboarding_id, Some("7095671727698478604".to_string()));
        assert_eq!(
            resp.offboarding_reason_unique_identifier,
            Some("reason_for_offboarding_option8".to_string())
        );
        assert_eq!(resp.add_block_list, Some(false));
        assert_eq!(resp.block_reason, Some("红线".to_string()));
        assert_eq!(resp.retain_account, Some(false));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
