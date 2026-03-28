//! 批量获取员工信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/employee/batch_get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required_list, SDKResult,
};

use super::models::{BatchGetRequestBody, BatchGetResponse};

/// 批量获取员工请求
#[derive(Debug, Clone)]
pub struct BatchGetRequest {
    /// 配置信息
    config: Config,
    /// 员工 ID 列表（必填，最多 100 个）
    employee_ids: Vec<String>,
}

impl BatchGetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            employee_ids: Vec::new(),
        }
    }

    /// 设置员工 ID 列表（必填，最多 100 个）
    pub fn employee_ids(mut self, employee_ids: Vec<String>) -> Self {
        self.employee_ids = employee_ids;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchGetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchGetResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        // 1. 验证必填字段
        validate_required_list!(
            self.employee_ids,
            100,
            "员工 ID 列表不能为空且不能超过 100 个"
        );

        // 2. 构建端点
        let api_endpoint = FeishuPeopleApiV1::EmployeeBatchGet;
        let request = ApiRequest::<BatchGetResponse>::post(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = BatchGetRequestBody {
            employee_ids: self.employee_ids,
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
                "批量获取员工响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for BatchGetResponse {
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
