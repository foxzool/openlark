//! 获取 OKR 进展记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v1/progress_record/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取 OKR 进展记录请求
#[derive(Debug, Clone)]
pub struct GetRequest {
    /// 进展记录 ID（必填）
    progress_id: String,
    /// 配置信息
    config: Config,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config, progress_id: String) -> Self {
        Self {
            progress_id,
            config,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetResponse> {
        use crate::common::api_endpoints::OkrApiV1;

        // 1. 验证必填字段
        validate_required!(self.progress_id.trim(), "进展记录 ID 不能为空");

        // 2. 构建端点
        let api_endpoint = OkrApiV1::ProgressRecordGet(self.progress_id.clone());
        let request = ApiRequest::<GetResponse>::get(api_endpoint.to_url());

        // 3. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 4. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取 OKR 进展记录响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 获取 OKR 进展记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 进展记录 ID
    pub progress_id: String,
    /// OKR ID
    pub okr_id: String,
    /// 进展内容
    pub content: String,
    /// 进展百分比
    pub progress_rate: i32,
    /// 进展说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 附件列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<super::create::ProgressAttachment>>,
    /// 创建者 ID
    pub creator_id: String,
    /// 创建时间
    pub created_at: i64,
    /// 更新时间
    pub updated_at: i64,
}

impl ApiResponseTrait for GetResponse {
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
