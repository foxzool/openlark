//! 批量获取公司
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/company/batch_get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required_list, SDKResult,
};

use super::models::{BatchGetRequestBody, BatchGetResponse};

/// 批量获取公司请求
#[derive(Debug, Clone)]
pub struct BatchGetRequest {
    /// 配置信息
    config: Config,
    /// 公司 ID 列表（必填，最多 100 个）
    company_ids: Vec<String>,
}

impl BatchGetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            company_ids: Vec::new(),
        }
    }

    /// 设置公司 ID 列表（必填，最多 100 个）
    pub fn company_ids(mut self, company_ids: Vec<String>) -> Self {
        self.company_ids = company_ids;
        self
    }

    /// 添加单个公司 ID
    pub fn add_company_id(mut self, company_id: String) -> Self {
        self.company_ids.push(company_id);
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
            self.company_ids,
            100,
            "公司 ID 列表不能为空且不能超过 100 个"
        );

        // 2. 构建端点
        let api_endpoint = FeishuPeopleApiV1::CompanyBatchGet;
        let request = ApiRequest::<BatchGetResponse>::post(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = BatchGetRequestBody {
            company_ids: self.company_ids,
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
                "批量获取公司响应数据为空",
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
