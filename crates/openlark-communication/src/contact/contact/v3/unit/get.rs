//! 获取单位信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/unit/get>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};

use crate::{contact::contact::v3::unit::models::GetUnitResponse, endpoints::CONTACT_V3_UNIT};

/// 获取单位信息请求
///
/// 用于查询单个单位的详情。
pub struct GetUnitRequest {
    config: Config,
    unit_id: String,
}

impl GetUnitRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            unit_id: String::new(),
        }
    }

    /// 单位 ID（路径参数）
    pub fn unit_id(mut self, unit_id: impl Into<String>) -> Self {
        self.unit_id = unit_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/unit/get>
    pub async fn execute(self) -> SDKResult<GetUnitResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetUnitResponse> {
        validate_required!(self.unit_id, "unit_id 不能为空");

        // url: GET:/open-apis/contact/v3/unit/:unit_id
        let req: ApiRequest<GetUnitResponse> =
            ApiRequest::get(format!("{}/{}", CONTACT_V3_UNIT, self.unit_id));

        Transport::request_typed(req, &self.config, Some(option), "获取单位信息").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/contact/v3/unit/test001
    #[tokio::test]
    async fn test_get_unit_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/contact/v3/unit/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "unit": { "unit_id": "test001" } }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        GetUnitRequest::new(config)
            .unit_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
