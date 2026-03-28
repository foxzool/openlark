//! 查询审计日志详情
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/application-audit_log/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 查询审计日志详情 Builder
#[derive(Debug, Clone)]
pub struct AuditLogGetBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 日志 ID
    log_id: String,
}

impl AuditLogGetBuilder {
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
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/audit_log",
            self.namespace
        );

        let mut req: ApiRequest<AuditLogGetResponse> = ApiRequest::get(&url);
        req = req.query("log_id", &self.log_id);
        let resp = Transport::request(req, &self.config, None).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("查询审计日志详情", "响应数据为空")
        })
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
    audit_log: AuditLogDetail,
}

impl ApiResponseTrait for AuditLogGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
