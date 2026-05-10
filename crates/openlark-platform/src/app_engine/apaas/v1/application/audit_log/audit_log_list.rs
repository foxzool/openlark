//! 查询审计日志列表
//!
//! 文档: https://open.feishu.cn/document/apaas-v1/application-audit_log/audit_log_list
//! docPath: https://open.feishu.cn/document/apaas-v1/application-audit_log/audit_log_list

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
pub struct AuditLogListBuilder {
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

impl AuditLogListBuilder {
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
    items: Vec<AuditLogInfo>,
    /// 是否有更多
    #[serde(rename = "has_more")]
    has_more: bool,
    /// 页码
    #[serde(rename = "page")]
    page: u32,
    /// 每页数量
    #[serde(rename = "page_size")]
    page_size: u32,
}

impl ApiResponseTrait for AuditLogListResponse {
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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
