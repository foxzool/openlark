//! 创建公司
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/company/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};

use super::models::{CreateRequestBody, CreateResponse};

/// 创建公司请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 配置信息
    config: Config,
    /// 公司名称（必填）
    name: String,
    /// 公司编码
    code: Option<String>,
    /// 公司描述
    description: Option<String>,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            name: String::new(),
            code: None,
            description: None,
        }
    }

    /// 设置公司名称（必填）
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    /// 设置公司编码
    pub fn code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }

    /// 设置公司描述
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
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
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        // 1. 验证必填字段
        validate_required!(self.name.trim(), "公司名称不能为空");

        // 2. 构建端点
        let api_endpoint = FeishuPeopleApiV1::CompanyCreate;
        let request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = CreateRequestBody {
            name: self.name,
            code: self.code,
            description: self.description,
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
                "创建公司响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for CreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;
    use serde_json::json;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build()
    }

    #[test]
    fn test_company_create_request_builder() {
        let request = CreateRequest::new(create_test_config())
            .name("测试公司".to_string())
            .code("COMP-001".to_string())
            .description("用于单元测试".to_string());

        assert_eq!(request.name, "测试公司");
        assert_eq!(request.code.as_deref(), Some("COMP-001"));
        assert_eq!(request.description.as_deref(), Some("用于单元测试"));
    }

    #[test]
    fn test_company_create_request_body_serialize() {
        let request = CreateRequest::new(create_test_config())
            .name("测试公司".to_string())
            .code("COMP-002".to_string());

        let body = CreateRequestBody {
            name: request.name,
            code: request.code,
            description: request.description,
        };
        let value = serde_json::to_value(body).expect("序列化请求体失败");

        assert_eq!(value["name"], json!("测试公司"));
        assert_eq!(value["code"], json!("COMP-002"));
    }

    #[test]
    fn test_company_create_response_deserialize() {
        let value = json!({"company_id": "com_123"});
        let response: CreateResponse = serde_json::from_value(value).expect("反序列化响应失败");
        assert_eq!(response.company_id, "com_123");
    }

    #[test]
    fn test_company_create_validation() {
        let request = CreateRequest::new(create_test_config()).name("  ".to_string());
        let result: SDKResult<()> = (|| {
            validate_required!(request.name.trim(), "公司名称不能为空");
            Ok(())
        })();

        assert!(result.is_err());
    }
}
