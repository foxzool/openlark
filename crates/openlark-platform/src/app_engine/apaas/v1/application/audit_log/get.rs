//! 查询审计日志详情
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/application-audit_log/get>
//! docPath: <https://open.feishu.cn/document/apaas-v1/application-audit_log/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 查询审计日志详情 Builder
#[derive(Debug, Clone)]
pub struct AuditLogGetRequestBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 日志 ID
    log_id: String,
}

impl AuditLogGetRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, namespace: impl Into<String>, log_id: impl Into<String>) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            log_id: log_id.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<AuditLogGetResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<AuditLogGetResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/audit_log",
            self.namespace
        );

        let mut req: ApiRequest<AuditLogGetResponse> = ApiRequest::get(&url);
        req = req.query("log_id", &self.log_id);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("查询审计日志详情", "响应数据为空")
        })
    }
}

/// 审计日志详情
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuditLogDetail {
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
    /// 操作详情
    #[serde(rename = "details")]
    details: serde_json::Value,
}

/// 查询审计日志详情响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuditLogGetResponse {
    /// 审计日志详情
    #[serde(rename = "audit_log")]
    pub audit_log: AuditLogDetail,
}

impl ApiResponseTrait for AuditLogGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to AuditLogGetRequestBuilder, will be removed in v1.0 (#271)")]
pub type AuditLogGetBuilder = AuditLogGetRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../audit_log?log_id=... → AuditLogGetResponse（inner data.audit_log）。
    #[tokio::test]
    async fn test_get_audit_log_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/apaas/v1/applications/ns_test/audit_log"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "audit_log": {
                        "log_id": "log_001",
                        "operation_type": "CREATE",
                        "operator": "u_001",
                        "operation_time": 1717000000,
                        "details": {"action": "create_record"}
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

        let resp = AuditLogGetRequestBuilder::new(config, "ns_test", "log_001")
            .execute()
            .await
            .expect("查询审计日志详情应成功");
        assert_eq!(resp.audit_log.log_id, "log_001");
        assert_eq!(resp.audit_log.operation_type, "CREATE");
        assert_eq!(resp.audit_log.operator, "u_001");
        assert_eq!(resp.audit_log.details["action"], "create_record");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/applications/ns_test/audit_log"
        );
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("log_id=log_001"));
    }
}
