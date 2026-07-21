//! 删除外部内推奖励
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/external_referral_reward/delete>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::hire::hire::common_models::ExternalReferralRewardResult;

/// 删除外部内推奖励请求
#[derive(Debug, Clone)]
pub struct DeleteRequest {
    /// 配置信息
    config: Config,
    external_referral_reward_id: String,
}

impl DeleteRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            external_referral_reward_id: String::new(),
        }
    }

    /// 提供 `external_referral_reward_id` 能力。
    pub fn external_referral_reward_id(
        mut self,
        external_referral_reward_id: impl Into<String>,
    ) -> Self {
        self.external_referral_reward_id = external_referral_reward_id.into();
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
            self.external_referral_reward_id.trim(),
            "external_referral_reward_id 不能为空"
        );

        let request = ApiRequest::<DeleteResponse>::delete(format!(
            "/open-apis/hire/v1/external_referral_rewards/{}",
            self.external_referral_reward_id
        ));
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "删除外部内推奖励响应数据为空",
        )
        .await
    }
}

/// 删除外部内推奖励响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DeleteResponse {
    #[serde(flatten)]
    /// `reward` 字段。
    pub reward: ExternalReferralRewardResult,
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

    /// 端到端：DELETE /open-apis/hire/v1/external_referral_rewards/test001
    #[tokio::test]
    async fn test_delete_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/hire/v1/external_referral_rewards/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "reward": {  } }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        DeleteRequest::new(config)
            .external_referral_reward_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
