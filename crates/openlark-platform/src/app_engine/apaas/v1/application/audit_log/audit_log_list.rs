//! 查询审计日志列表
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/application-audit_log/audit_log_list>
//! docPath: <https://open.feishu.cn/document/apaas-v1/application-audit_log/audit_log_list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 查询审计日志列表 Builder
#[derive(Debug, Clone)]
pub struct AuditLogListRequestBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 开始时间
    start_time: Option<i64>,
    /// 结束时间
    end_time: Option<i64>,
    /// 页码
    page: Option<u32>,
    /// 每页数量
    page_size: Option<u32>,
}

impl AuditLogListRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, namespace: impl Into<String>) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            start_time: None,
            end_time: None,
            page: None,
            page_size: None,
        }
    }

    /// 设置开始时间
    pub fn start_time(mut self, start_time: i64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// 设置结束时间
    pub fn end_time(mut self, end_time: i64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /// 设置页码
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// 设置每页数量
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<AuditLogListResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<AuditLogListResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/audit_log/audit_log_list",
            self.namespace
        );

        let mut req: ApiRequest<AuditLogListResponse> = ApiRequest::get(&url);
        if let Some(start_time) = self.start_time {
            req = req.query("start_time", start_time.to_string());
        }
        if let Some(end_time) = self.end_time {
            req = req.query("end_time", end_time.to_string());
        }
        if let Some(page) = self.page {
            req = req.query("page", page.to_string());
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("查询审计日志列表", "响应数据为空")
        })
    }
}

/// 审计日志信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuditLogInfo {
    /// 日志 ID
    #[serde(rename = "log_id")]
    log_id: String,
    /// 操作类型
    #[serde(rename = "operation_type")]
    operation_type: String,
    /// 操作人
    #[serde(rename = "operator")]
    operator: String,
    /// 操作时间
    #[serde(rename = "operation_time")]
    operation_time: i64,
}

/// 查询审计日志列表响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuditLogListResponse {
    /// 审计日志列表
    #[serde(rename = "items")]
    pub items: Vec<AuditLogInfo>,
    /// 是否有更多
    #[serde(rename = "has_more")]
    pub has_more: bool,
    /// 页码
    #[serde(rename = "page")]
    pub page: u32,
    /// 每页数量
    #[serde(rename = "page_size")]
    pub page_size: u32,
}

impl ApiResponseTrait for AuditLogListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to AuditLogListRequestBuilder, will be removed in v1.0 (#271)")]
pub type AuditLogListBuilder = AuditLogListRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../applications/{ns}/audit_log/audit_log_list → 强类型 AuditLogListResponse。
    #[tokio::test]
    async fn test_list_audit_log_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/apaas/v1/applications/ns_test/audit_log/audit_log_list",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "log_id": "log_001",
                            "operation_type": "CREATE",
                            "operator": "u_001",
                            "operation_time": 1717000000
                        }
                    ],
                    "has_more": false,
                    "page": 1,
                    "page_size": 20
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

        let resp = AuditLogListRequestBuilder::new(config, "ns_test")
            .start_time(1716000000)
            .end_time(1717000000)
            .page(1)
            .page_size(20)
            .execute()
            .await
            .expect("查询审计日志列表应成功");
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.items[0].log_id, "log_001");
        assert!(!resp.has_more);
        assert_eq!(resp.page, 1);
        assert_eq!(resp.page_size, 20);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/applications/ns_test/audit_log/audit_log_list"
        );
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("start_time=1716000000"));
        assert!(query.contains("end_time=1717000000"));
        assert!(query.contains("page=1"));
        assert!(query.contains("page_size=20"));
    }
}
