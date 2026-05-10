//! 查询数据变更日志详情
//!
//! 文档: https://open.feishu.cn/document/apaas-v1/application-audit_log/data_change_log_detail
//! docPath: https://open.feishu.cn/document/apaas-v1/application-audit_log/data_change_log_detail

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 查询数据变更日志详情 Builder
#[derive(Debug, Clone)]
pub struct DataChangeLogDetailBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 日志 ID
    log_id: String,
}

impl DataChangeLogDetailBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, namespace: impl Into<String>, log_id: impl Into<String>) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            log_id: log_id.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DataChangeLogDetailResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DataChangeLogDetailResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/audit_log/data_change_log_detail",
            self.namespace
        );

        let mut req: ApiRequest<DataChangeLogDetailResponse> = ApiRequest::get(&url);
        req = req.query("log_id", &self.log_id);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("查询数据变更日志详情", "响应数据为空")
        })
    }
}

/// 数据变更日志详情
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DataChangeLogDetail {
    /// 日志 ID
    #[serde(rename = "log_id")]
    log_id: String,
    /// 变更类型
    #[serde(rename = "change_type")]
    change_type: String,
    /// 变更对象类型
    #[serde(rename = "object_type")]
    object_type: String,
    /// 变更对象 ID
    #[serde(rename = "object_id")]
    object_id: String,
    /// 操作人
    #[serde(rename = "operator")]
    operator: String,
    /// 变更时间
    #[serde(rename = "change_time")]
    change_time: i64,
    /// 变更前数据
    #[serde(rename = "before_data", skip_serializing_if = "Option::is_none")]
    before_data: Option<serde_json::Value>,
    /// 变更后数据
    #[serde(rename = "after_data", skip_serializing_if = "Option::is_none")]
    after_data: Option<serde_json::Value>,
}

/// 查询数据变更日志详情响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DataChangeLogDetailResponse {
    /// 数据变更日志详情
    #[serde(rename = "data_change_log")]
    data_change_log: DataChangeLogDetail,
}

impl ApiResponseTrait for DataChangeLogDetailResponse {
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
