//! Bitable 创建多维表格API
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use super::AppService;
use super::models::{App, CreateAppRequest as CreateAppRequestBody};

/// 创建多维表格请求。
pub struct CreateAppRequest {
    /// 应用名称
    name: String,
    /// 文件夹token
    folder_token: Option<String>,
    /// 时区
    time_zone: Option<String>,
    /// 配置信息
    config: Config,
}

/// 创建多维表格响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateAppResponse {
    /// 应用信息
    pub app: App,
}

impl ApiResponseTrait for CreateAppResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateAppRequest {
    /// 创建新的多维表格创建请求。
    /// 创建新增多维表格请求
    pub fn new(config: Config) -> Self {
        Self {
            name: String::new(),
            folder_token: None,
            time_zone: None,
            config,
        }
    }

    /// 设置应用名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// 设置文件夹token
    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.folder_token = Some(folder_token.into());
        self
    }

    /// 设置时区
    pub fn time_zone(mut self, time_zone: impl Into<String>) -> Self {
        self.time_zone = Some(time_zone.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<CreateAppResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateAppResponse> {
        // 验证必填字段
        validate_required!(self.name, "应用名称不能为空");

        // 🚀 使用新的enum+builder系统生成API端点
        // 替代传统的字符串拼接方式，提供类型安全和IDE自动补全
        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::AppCreate;

        // 构建请求体 - 符合官方文档格式
        let request_body = CreateAppRequestBody {
            name: self.name.clone(),
            folder_token: self.folder_token.clone(),
            time_zone: self.time_zone.clone(),
            app_settings: None,
        };

        // #439: method 来自 catalog；叶子不再重复声明稳定请求语义
        let api_request: ApiRequest<CreateAppResponse> =
            api_endpoint.to_request::<CreateAppResponse>().body(
                openlark_core::api::RequestData::Binary(serde_json::to_vec(&request_body)?),
            );

        // 发送请求
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl AppService {
    /// 创建新增多维表格请求 builder。
    pub fn create_builder(&self, name: impl Into<String>) -> CreateAppRequest {
        CreateAppRequest::new(self.config.clone()).name(name)
    }

    /// 创建新增多维表格请求（带完整参数）
    pub fn create_app(
        &self,
        name: impl Into<String>,
        folder_token: Option<impl Into<String>>,
        time_zone: Option<impl Into<String>>,
    ) -> CreateAppRequest {
        let mut request = CreateAppRequest::new(self.config.clone()).name(name);

        if let Some(folder_token) = folder_token {
            request = request.folder_token(folder_token);
        }

        if let Some(time_zone) = time_zone {
            request = request.time_zone(time_zone);
        }

        request
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/bitable/v1/apps → CreateAppResponse。
    /// 完整断言 method、path、auth（来自 catalog #439）和响应。
    #[tokio::test]
    async fn test_create_app_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/bitable/v1/apps"))
            .and(header("Authorization", "Bearer test-tenant-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "app": { "app_token": "app001", "name": "测试应用" } }
            })))
            .mount(&server)
            .await;
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();
        let option = openlark_core::req_option::RequestOption::builder()
            .tenant_access_token("test-tenant-token")
            .build();
        let resp = CreateAppRequest::new(config)
            .name("测试应用")
            .execute_with_options(option)
            .await
            .expect("创建多维表格应成功");
        assert_eq!(resp.app.app_token, "app001");
        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].method, "POST");
        assert_eq!(received[0].url.path(), "/open-apis/bitable/v1/apps");
        let body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为 JSON");
        assert_eq!(body["name"], "测试应用");
        assert_eq!(
            received[0]
                .headers
                .get("authorization")
                .and_then(|h| h.to_str().ok()),
            Some("Bearer test-tenant-token")
        );
    }
}
