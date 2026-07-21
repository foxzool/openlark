//! 更新公司
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/company/patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};

use super::models::{PatchRequestBody, PatchResponse};

/// 更新公司请求
#[derive(Debug, Clone)]
pub struct PatchRequest {
    /// 配置信息
    config: Config,
    /// 公司 ID（必填）
    company_id: String,
    /// 公司名称
    name: Option<String>,
    /// 公司编码
    code: Option<String>,
    /// 公司描述
    description: Option<String>,
    /// 公司状态
    /// - 0: 停用
    /// - 1: 启用
    status: Option<i32>,
}

impl PatchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            company_id: String::new(),
            name: None,
            code: None,
            description: None,
            status: None,
        }
    }

    /// 设置公司 ID（必填）
    pub fn company_id(mut self, company_id: String) -> Self {
        self.company_id = company_id;
        self
    }

    /// 设置公司名称
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
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

    /// 设置公司状态
    /// - 0: 停用
    /// - 1: 启用
    pub fn status(mut self, status: i32) -> Self {
        self.status = Some(status);
        self
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
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        // 1. 验证必填字段
        validate_required!(self.company_id.trim(), "公司 ID 不能为空");

        // 验证至少有一个更新字段
        if self.name.is_none()
            && self.code.is_none()
            && self.description.is_none()
            && self.status.is_none()
        {
            return Err(openlark_core::error::validation_error(
                "更新字段不能为空",
                "至少需要提供一个更新字段（name、code、description 或 status）",
            ));
        }

        // 2. 构建端点
        let api_endpoint = FeishuPeopleApiV1::CompanyPatch(self.company_id.clone());
        let request = ApiRequest::<PatchResponse>::patch(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = PatchRequestBody {
            company_id: self.company_id,
            name: self.name,
            code: self.code,
            description: self.description,
            status: self.status,
        };
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);

        // 4. 发送请求
        Transport::request_typed(request, &self.config, Some(option), "更新公司响应数据为空").await
    }
}

impl ApiResponseTrait for PatchResponse {
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

    /// 端到端：PATCH /open-apis/corehr/v1/companies/test001
    #[tokio::test]
    async fn test_patch_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/corehr/v1/companies/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "result": false }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        PatchRequest::new(config)
            .company_id("test001".to_string())
            .name("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
