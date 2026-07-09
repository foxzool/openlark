//! 删除地点
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/location/delete>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};

use super::models::DeleteResponse;

/// 删除地点请求
#[derive(Debug, Clone)]
pub struct DeleteRequest {
    /// 配置信息
    config: Config,
    /// 地点 ID（必填）
    location_id: String,
}

impl DeleteRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            location_id: String::new(),
        }
    }

    /// 设置地点 ID（必填）
    pub fn location_id(mut self, location_id: String) -> Self {
        self.location_id = location_id;
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
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        // 1. 验证必填字段
        validate_required!(self.location_id.trim(), "地点 ID 不能为空");

        // 2. 构建端点
        let api_endpoint = FeishuPeopleApiV1::LocationDelete(self.location_id.clone());
        let request = ApiRequest::<DeleteResponse>::delete(api_endpoint.to_url());

        // 3. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 4. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "删除地点响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for DeleteResponse {
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

    /// 端到端：DELETE /open-apis/corehr/v1/locations/test001
    #[tokio::test]
    async fn test_delete_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/corehr/v1/locations/test001"))
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

        DeleteRequest::new(config)
            .location_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
