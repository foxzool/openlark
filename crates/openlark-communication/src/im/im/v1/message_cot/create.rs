//! 创建 COT
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/message_cot/create>

use openlark_core::{SDKResult, api::ApiRequest, config::Config, http::Transport};

use crate::{common::api_utils::serialize_params, endpoints::IM_V1_MESSAGE_COT};

/// 创建 COT 请求
///
/// 用于创建一条 COT（消息协作）。
///
/// 请求体与响应均以 `serde_json::Value` 透传（openlark-api 核心契约 2 的
/// 无 schema 范式）：message_cot 为飞书新近发布资源，官方文档当前为 SPA
/// 动态渲染、字段定义无法静态抓取，故 SDK 不臆测字段。待文档稳定后可
/// 收敛为 typed Body/Response（参考同域 `message`/`thread` 资源）。
///
/// # 字段说明
///
/// - `config`: 配置信息
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_communication::im::v1::message_cot::CreateMessageCotRequest;
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let body = serde_json::json!({ /* 字段以飞书文档为准 */ });
/// let response = CreateMessageCotRequest::new(config).execute(body).await?;
/// ```
pub struct CreateMessageCotRequest {
    config: Config,
}

impl CreateMessageCotRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/im-v1/message_cot/create>
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
        // url: POST:/open-apis/im/v1/message_cot
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(IM_V1_MESSAGE_COT).body(serialize_params(&body, "创建 COT")?);

        Transport::request_typed(req, &self.config, Some(option), "创建 COT").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    /// 端到端：POST /open-apis/im/v1/message_cot，断言 path + body 透传 + typed data。
    #[tokio::test]
    async fn test_create_message_cot_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/im/v1/message_cot"))
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

        let resp = CreateMessageCotRequest::new(config)
            .execute(json!({ "message_id": "om_test" }))
            .await
            .expect("请求应成功");
        assert_eq!(resp["cot_id"], "cot_test001");

        let received = server.received_requests().await.expect("应收到请求");
        assert_eq!(received.len(), 1);
        let body_str = std::str::from_utf8(&received[0].body).expect("body utf8");
        assert!(
            body_str.contains("om_test"),
            "请求体应包含 message_id: {body_str}"
        );
    }
}
