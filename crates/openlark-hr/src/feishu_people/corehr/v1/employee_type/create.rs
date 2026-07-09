//! 创建人员类型
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/employee_type/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};

use super::models::{CreateRequestBody, CreateResponse};

/// 创建人员类型请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 配置信息
    config: Config,
    /// 人员类型名称（必填）
    name: String,
    /// 人员类型描述
    description: Option<String>,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            name: String::new(),
            description: None,
        }
    }

    /// 设置人员类型名称（必填）
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    /// 设置人员类型描述
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
        validate_required!(self.name.trim(), "人员类型名称不能为空");

        // 2. 构建端点
        let api_endpoint = FeishuPeopleApiV1::EmployeeTypeCreate;
        let request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = CreateRequestBody {
            name: self.name,
            description: self.description,
        };
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);

        // 4. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 5. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "创建人员类型响应数据为空",
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
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v1/employee_types
    #[tokio::test]
    async fn test_create_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v1/employee_types"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "employee_type_id": "" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        CreateRequest::new(config)
            .name("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
