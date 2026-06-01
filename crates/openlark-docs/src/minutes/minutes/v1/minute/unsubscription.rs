//! 取消订阅妙记变更事件
//!
//! docPath:

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use crate::common::{
    api_endpoints::MinutesApiV1,
    api_utils::{extract_response_data, serialize_params},
};

/// 取消订阅妙记变更事件请求。
pub struct UnsubscribeMinuteRequest {
    config: Config,
}

impl UnsubscribeMinuteRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        if body.is_null() {
            return Err(openlark_core::error::validation_error(
                "body",
                "请求体不能为空",
            ));
        }

        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(MinutesApiV1::Unsubscription.to_url())
                .body(serialize_params(&body, "取消订阅妙记变更事件")?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "取消订阅妙记变更事件")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn minute_unsubscription_issue_194_rejects_null_body() {
        let err = UnsubscribeMinuteRequest::new(Config::default())
            .execute(serde_json::Value::Null)
            .await
            .expect_err("空请求体必须失败");

        assert!(err.to_string().contains("请求体不能为空"));
    }
}
