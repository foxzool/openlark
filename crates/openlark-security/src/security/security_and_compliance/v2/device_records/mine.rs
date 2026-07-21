//! 获取我的设备认证信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/security_and_compliance-v2/device_record-mine>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType, http::Transport,
    req_option::RequestOption,
};

/// 获取我的设备认证信息请求
///
/// 支持分页及状态过滤。
#[derive(Debug)]
pub struct GetMyDeviceRecordsRequest {
    /// 配置信息。
    config: Config,
    /// 页面大小（可选）。
    page_size: Option<i32>,
    /// 分页标记（可选）。
    page_token: Option<String>,
    /// 状态过滤（可选）。
    status: Option<String>,
}

impl GetMyDeviceRecordsRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            status: None,
        }
    }

    /// 设置页面大小。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置状态过滤（如 `approved`/`pending`/`rejected`）。
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// 执行请求，返回响应 `data` 字段内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::get("/open-apis/security_and_compliance/v2/device_records/mine")
                .query_opt("page_size", self.page_size.map(|v| v.to_string()))
                .query_opt("page_token", self.page_token.as_ref())
                .query_opt("status", self.status.as_ref())
                .with_supported_access_token_types(vec![AccessTokenType::App]);

        Transport::request_typed(req, &self.config, Some(option), "获取我的设备认证信息").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET .../device_records/mine + query 拼装 + 响应解析。
    #[tokio::test]
    async fn test_get_my_device_records_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/security_and_compliance/v2/device_records/mine",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [{ "device_record_id": "dr_001" }],
                    "has_more": false
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

        let data = GetMyDeviceRecordsRequest::new(config)
            .page_size(10)
            .status("approved")
            .execute()
            .await
            .expect("获取我的设备认证信息应成功");
        assert_eq!(data["items"].as_array().unwrap().len(), 1);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("page_size=10"));
        assert!(query.contains("status=approved"));
    }
}
