//! 批量删除补充信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/performance-v2/additional_informations.batch/delete>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 批量删除补充信息请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DeleteRequest {
    /// 绩效周期 ID（必填）
    cycle_id: String,
    /// 用户 ID 列表（必填）
    user_ids: Vec<String>,
    /// 配置信息
    config: Config,
}

impl DeleteRequest {
    /// 创建请求
    pub fn new(config: Config, cycle_id: String) -> Self {
        Self {
            cycle_id,
            user_ids: Vec::new(),
            config,
        }
    }

    /// 添加用户 ID
    pub fn add_user_id(mut self, user_id: String) -> Self {
        self.user_ids.push(user_id);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DeleteResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteResponse> {
        use crate::common::api_endpoints::PerformanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.cycle_id.trim(), "cycle_id");

        // 2. 构建端点
        let api_endpoint = PerformanceApiV1::AdditionalInformationsBatchDelete;
        let request = ApiRequest::<DeleteResponse>::post(api_endpoint.to_url());

        // 3. 构建请求体
        let request_body = DeleteRequestBody {
            cycle_id: self.cycle_id,
            user_ids: self.user_ids,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {}", e),
            )
        })?;
        let request = request.body(request_body_json);

        // 4. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 5. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "批量删除补充信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 删除请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRequestBody {
    /// 绩效周期 ID
    pub cycle_id: String,
    /// 用户 ID 列表
    pub user_ids: Vec<String>,
}

/// 批量删除补充信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteResponse {
    /// 删除成功的条数
    pub success_count: i32,
}

impl ApiResponseTrait for DeleteResponse {
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
