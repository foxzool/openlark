//! 更新地点
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/location/patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};

use super::models::{PatchRequestBody, PatchResponse};

/// 更新地点请求
#[derive(Debug, Clone)]
pub struct PatchRequest {
    /// 配置信息
    config: Config,
    /// 地点 ID（必填）
    location_id: String,
    /// 地点名称
    name: Option<String>,
    /// 地点类型
    /// - 1: 总部
    /// - 2: 分公司
    /// - 3: 办事处
    /// - 4: 其他
    location_type: Option<i32>,
    /// 详细地址
    address: Option<String>,
    /// 城市
    city: Option<String>,
    /// 国家
    country: Option<String>,
    /// 状态
    /// - 1: 启用
    /// - 2: 停用
    status: Option<i32>,
}

impl PatchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            location_id: String::new(),
            name: None,
            location_type: None,
            address: None,
            city: None,
            country: None,
            status: None,
        }
    }

    /// 设置地点 ID（必填）
    pub fn location_id(mut self, location_id: String) -> Self {
        self.location_id = location_id;
        self
    }

    /// 设置地点名称
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// 设置地点类型
    pub fn location_type(mut self, location_type: i32) -> Self {
        self.location_type = Some(location_type);
        self
    }

    /// 设置详细地址
    pub fn address(mut self, address: String) -> Self {
        self.address = Some(address);
        self
    }

    /// 设置城市
    pub fn city(mut self, city: String) -> Self {
        self.city = Some(city);
        self
    }

    /// 设置国家
    pub fn country(mut self, country: String) -> Self {
        self.country = Some(country);
        self
    }

    /// 设置状态
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
        validate_required!(self.location_id.trim(), "地点 ID 不能为空");

        // 2. 构建端点
        let api_endpoint = FeishuPeopleApiV1::LocationPatch(self.location_id.clone());
        let request = ApiRequest::<PatchResponse>::patch(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = PatchRequestBody {
            name: self.name,
            location_type: self.location_type,
            address: self.address,
            city: self.city,
            country: self.country,
            status: self.status,
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
                "更新地点响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
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

    /// 端到端：PATCH /open-apis/corehr/v1/locations/test001
    #[tokio::test]
    async fn test_patch_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
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

        PatchRequest::new(config)
            .location_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
