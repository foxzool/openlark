//! 完成 COT
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/message_cot/complete>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};

use crate::{common::api_utils::serialize_params, endpoints::IM_V1_MESSAGE_COT};

/// 完成 COT 请求
///
/// 用于完成指定的 COT（消息协作）。
///
/// 请求体与响应均以 `serde_json::Value` 透传（openlark-api 核心契约 2 的
/// 无 schema 范式）：message_cot 官方文档当前为 SPA 动态渲染、字段定义
/// 无法静态抓取，故 SDK 不臆测字段。待文档稳定后可收敛为 typed
/// Body/Response（参考同域 `message`/`thread` 资源）。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `cot_id`: 待完成的 COT ID，必填（路径参数）
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_communication::im::v1::message_cot::CompleteMessageCotRequest;
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let body = serde_json::json!({ /* 字段以飞书文档为准 */ });
/// let response = CompleteMessageCotRequest::new(config)
///     .cot_id("cot_xxx")
///     .execute(body)
///     .await?;
/// ```
pub struct CompleteMessageCotRequest {
    config: Config,
    cot_id: String,
}

impl CompleteMessageCotRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            cot_id: String::new(),
        }
    }

    /// 待完成的 COT ID（路径参数，必填）
    pub fn cot_id(mut self, cot_id: impl Into<String>) -> Self {
        self.cot_id = cot_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/im-v1/message_cot/complete>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.cot_id, "cot_id 不能为空");

        // url: POST:/open-apis/im/v1/message_cot/complete/:cot_id
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(format!("{}/complete/{}", IM_V1_MESSAGE_COT, self.cot_id))
                .body(serialize_params(&body, "完成 COT")?);

        Transport::request_typed(req, &self.config, Some(option), "完成 COT").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    /// 端到端：POST /open-apis/im/v1/message_cot/complete/:cot_id，断言 path + body 透传。
    #[tokio::test]
    async fn test_complete_message_cot_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/im/v1/message_cot/complete/cot_test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "cot_id": "cot_test001" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = CompleteMessageCotRequest::new(config)
            .cot_id("cot_test001")
            .execute(json!({ "status": "completed" }))
            .await
            .expect("请求应成功");
        assert_eq!(resp["cot_id"], "cot_test001");

        let received = server.received_requests().await.expect("应收到请求");
        assert_eq!(received.len(), 1);
        let body_str = std::str::from_utf8(&received[0].body).expect("body utf8");
        assert!(
            body_str.contains("completed"),
            "请求体应包含 status 字段: {body_str}"
        );
    }

    /// 缺 cot_id 时 validate_required! 应在发请求前返回错误。
    #[tokio::test]
    async fn test_complete_message_cot_missing_cot_id_is_error() {
        let config = Config::default();
        let request = CompleteMessageCotRequest::new(config);
        let result = request.execute(json!({})).await;
        assert!(result.is_err(), "缺 cot_id 应返回错误");
    }
}
