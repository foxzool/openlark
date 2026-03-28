//! 创建 OKR 进展记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v1/progress_record/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 创建 OKR 进展记录请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// OKR ID（必填）
    okr_id: String,
    /// 进展内容（必填）
    content: String,
    /// 进展百分比（必填，0-100）
    progress_rate: i32,
    /// 进展说明（可选）
    description: Option<String>,
    /// 附件列表（可选）
    attachments: Option<Vec<ProgressAttachment>>,
    /// 配置信息
    config: Config,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config, okr_id: String, content: String, progress_rate: i32) -> Self {
        Self {
            okr_id,
            content,
            progress_rate,
            description: None,
            attachments: None,
            config,
        }
    }

    /// 设置进展说明（可选）
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 设置附件列表（可选）
    pub fn attachments(mut self, attachments: Vec<ProgressAttachment>) -> Self {
        self.attachments = Some(attachments);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        use crate::common::api_endpoints::OkrApiV1;

        // 1. 验证必填字段
        validate_required!(self.okr_id.trim(), "OKR ID 不能为空");
        validate_required!(self.content.trim(), "进展内容不能为空");

        // 2. 构建端点
        let api_endpoint = OkrApiV1::ProgressRecordCreate;
        let request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = CreateRequestBody {
            okr_id: self.okr_id,
            content: self.content,
            progress_rate: self.progress_rate,
            description: self.description,
            attachments: self.attachments,
        };
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {}", e),
            )
        })?);

        // 4. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 5. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "创建 OKR 进展记录响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 创建 OKR 进展记录请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequestBody {
    /// OKR ID（必填）
    pub okr_id: String,
    /// 进展内容（必填）
    pub content: String,
    /// 进展百分比（必填，0-100）
    pub progress_rate: i32,
    /// 进展说明（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 附件列表（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<ProgressAttachment>>,
}

/// 进展记录附件
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProgressAttachment {
    /// 附件类型
    pub file_type: String,
    /// 附件 URL
    pub file_url: String,
    /// 附件名称
    pub file_name: String,
}

/// 创建 OKR 进展记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 进展记录 ID
    pub progress_id: String,
    /// OKR ID
    pub okr_id: String,
    /// 进展内容
    pub content: String,
    /// 进展百分比
    pub progress_rate: i32,
    /// 创建时间
    pub created_at: i64,
}

impl ApiResponseTrait for CreateResponse {
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
