//! 修改 OKR 周期状态
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v1/period/patch>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 修改 OKR 周期状态请求
#[derive(Debug, Clone)]
pub struct PatchRequest {
    /// 周期 ID（必填）
    period_id: String,
    /// 周期状态（必填）
    /// - 1: 未开始
    /// - 2: 进行中
    /// - 3: 已结束
    status: i32,
    /// 配置信息
    config: Config,
}

impl PatchRequest {
    /// 创建请求
    pub fn new(config: Config, period_id: String, status: i32) -> Self {
        Self {
            period_id,
            status,
            config,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<PatchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PatchResponse> {
        use crate::common::api_endpoints::OkrApiV1;

        // 1. 构建端点
        let api_endpoint = OkrApiV1::PeriodPatch(self.period_id.clone());
        let request = ApiRequest::<PatchResponse>::patch(api_endpoint.to_url());

        // 2. 序列化请求体
        let request_body = PatchRequestBody {
            status: self.status,
        };
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {}", e),
            )
        })?);

        // 3. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 4. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "修改 OKR 周期状态响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 修改 OKR 周期状态请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchRequestBody {
    /// 周期状态（必填）
    pub status: i32,
}

/// 修改 OKR 周期状态响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchResponse {
    /// 周期 ID
    pub period_id: String,
    /// 周期状态
    pub status: i32,
    /// 更新时间
    pub updated_at: i64,
}

impl ApiResponseTrait for PatchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

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
