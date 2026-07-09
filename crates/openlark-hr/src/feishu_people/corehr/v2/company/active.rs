//! 启用/停用公司
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/company/active>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};

use serde::{Deserialize, Serialize};

/// 启用/停用公司请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActiveRequest {
    /// 是否启用（true=启用，false=停用）
    pub active: bool,
}

impl ActiveRequest {
    /// 创建请求
    pub fn new(active: bool) -> Self {
        Self { active }
    }
}

/// 启用/停用公司响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActiveResponse {
    /// 公司 ID
    pub data: Option<ActiveResponseData>,
}

/// `ActiveResponseData`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActiveResponseData {
    /// 公司 ID
    pub id: String,
}

impl ApiResponseTrait for ActiveResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 启用/停用公司请求构建器
#[derive(Debug, Clone)]
pub struct ActiveRequestBuilder {
    config: Config,
    active: bool,
}

impl ActiveRequestBuilder {
    /// 创建请求构建器
    pub fn new(config: Config, active: bool) -> Self {
        Self { config, active }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ActiveResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ActiveResponse> {
        let request = ApiRequest::<ActiveResponse>::post("/open-apis/corehr/v2/companies/active");

        // 序列化请求体
        let request_body = ActiveRequest::new(self.active);
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);

        // 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "启用/停用公司响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/companies/active
    #[tokio::test]
    async fn test_active_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/companies/active"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {  }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        ActiveRequestBuilder::new(config, true)
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
