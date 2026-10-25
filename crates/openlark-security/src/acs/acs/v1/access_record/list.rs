//! 获取门禁记录列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/acs-v1/access_record/list>
//!
//! 文档核对：查询参数为 `from`（必填）、`to`（必填，时间跨度不能超过 30 天）、
//! `device_id`（可选）、`user_id_type`（可选，默认 open_id）、
//! `page_size`（可选，默认 100，最大 500）、`page_token`（可选）。

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType, http::Transport,
    req_option::RequestOption,
};

/// 获取门禁记录列表请求
///
/// `from`/`to` 为必填查询参数（单位秒，时间跨度不能超过 30 天，超限由服务端
/// 错误码 1490017 兜底）。
#[derive(Debug)]
pub struct ListAccessRecordsRequest {
    /// 配置信息。
    config: Config,
    /// 记录开始时间，单位秒（查询参数，必填）。
    from: i64,
    /// 记录结束时间，单位秒（查询参数，必填）。
    to: i64,
    /// 门禁设备 ID（查询参数，可选）。
    device_id: Option<String>,
    /// 用户 ID 类型（查询参数，可选，open_id/union_id/user_id，默认 open_id）。
    user_id_type: Option<String>,
    /// 页面大小（可选，默认 100，最大 500）。
    page_size: Option<i32>,
    /// 分页标记（可选）。
    page_token: Option<String>,
}

impl ListAccessRecordsRequest {
    /// 创建新的请求构建器。
    ///
    /// `from`/`to` 为必填的记录起止时间（单位秒）。
    pub fn new(config: Config, from: i64, to: i64) -> Self {
        Self {
            config,
            from,
            to,
            device_id: None,
            user_id_type: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置门禁设备 ID。
    pub fn device_id(mut self, device_id: impl Into<String>) -> Self {
        self.device_id = Some(device_id.into());
        self
    }

    /// 设置用户 ID 类型（open_id/union_id/user_id）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
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

    /// 执行请求，返回响应 `data` 字段内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::get("/open-apis/acs/v1/access_records")
                .query("from", self.from.to_string())
                .query("to", self.to.to_string())
                .query_opt("device_id", self.device_id.as_ref())
                .query_opt("user_id_type", self.user_id_type.as_ref())
                .query_opt("page_size", self.page_size.map(|v| v.to_string()))
                .query_opt("page_token", self.page_token.as_ref())
                .with_supported_access_token_types(vec![AccessTokenType::Tenant]);

        Transport::request_typed(req, &self.config, Some(option), "获取门禁记录列表").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET .../access_records + query 拼装（from/to 必填）+ 响应解析。
    #[tokio::test]
    async fn test_list_access_records_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/acs/v1/access_records"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [{ "access_record_id": "rec_001" }],
                    "page_token": "next_page",
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

        let data = ListAccessRecordsRequest::new(config, 1624520521, 1624606921)
            .device_id("dev_001")
            .user_id_type("open_id")
            .page_size(50)
            .page_token("curr_page")
            .execute()
            .await
            .expect("获取门禁记录列表应成功");

        assert_eq!(data["items"].as_array().unwrap().len(), 1);
        assert_eq!(data["has_more"], false);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("from=1624520521"));
        assert!(query.contains("to=1624606921"));
        assert!(query.contains("device_id=dev_001"));
        assert!(query.contains("user_id_type=open_id"));
        assert!(query.contains("page_size=50"));
        assert!(query.contains("page_token=curr_page"));
    }
}
