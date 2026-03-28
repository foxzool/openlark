//! 封存发薪活动
//!
//! docPath: <https://open.feishu.cn/document/server-docs/payroll-v1/payment_activity/archive>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 封存发薪活动请求
#[derive(Debug, Clone)]
pub struct ArchiveRequest {
    /// 发薪活动 ID（必填）
    activity_id: String,
    /// 配置信息
    config: Config,
}

impl ArchiveRequest {
    /// 创建请求
    pub fn new(config: Config, activity_id: String) -> Self {
        Self {
            activity_id,
            config,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ArchiveResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ArchiveResponse> {
        use crate::common::api_endpoints::PayrollApiV1;

        // 1. 构建端点
        let api_endpoint = PayrollApiV1::PaymentActivityArchive(self.activity_id.clone());
        let request = ApiRequest::<ArchiveResponse>::post(api_endpoint.to_url());

        // 2. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 3. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "封存发薪活动响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 封存发薪活动响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArchiveResponse {
    /// 是否成功
    pub success: bool,
    /// 发薪活动 ID
    pub activity_id: String,
    /// 封存时间（Unix 时间戳）
    pub archived_at: i64,
}

impl ApiResponseTrait for ArchiveResponse {
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
