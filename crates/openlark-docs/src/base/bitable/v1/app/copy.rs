//! Bitable 复制多维表格API
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/copy>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use super::AppService;
use super::models::{App, CopyAppRequest as CopyAppRequestBody};

/// 复制多维表格请求。
pub struct CopyAppRequest {
    /// 应用token
    app_token: String,
    /// 新应用名称
    name: Option<String>,
    /// 目标文件夹token
    folder_token: Option<String>,
    /// 是否复制内容（true: 不复制内容，false: 复制内容）
    without_content: Option<bool>,
    /// 时区
    time_zone: Option<String>,
    /// 配置信息
    config: Config,
}

/// 复制多维表格响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CopyAppResponse {
    /// 应用信息
    pub app: App,
}

impl ApiResponseTrait for CopyAppResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CopyAppRequest {
    /// 创建新的多维表格复制请求。
    /// 创建复制多维表格请求
    pub fn new(config: Config) -> Self {
        Self {
            app_token: String::new(),
            name: None,
            folder_token: None,
            without_content: None,
            time_zone: None,
            config,
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    /// 设置新应用名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置目标文件夹token
    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.folder_token = Some(folder_token.into());
        self
    }

    /// 设置是否复制内容（true: 不复制内容，false: 复制内容）
    pub fn without_content(mut self, without_content: bool) -> Self {
        self.without_content = Some(without_content);
        self
    }

    /// 设置时区
    pub fn time_zone(mut self, time_zone: impl Into<String>) -> Self {
        self.time_zone = Some(time_zone.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<CopyAppResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<CopyAppResponse> {
        // 验证必填字段
        validate_required!(self.app_token, "应用令牌不能为空");

        // 🚀 使用新的enum+builder系统生成API端点
        // 替代传统的字符串拼接方式，提供类型安全和IDE自动补全
        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::AppCopy(self.app_token.clone());

        // 构建请求体
        let request_body = CopyAppRequestBody {
            name: self.name.clone(),
            folder_token: self.folder_token.clone(),
            without_content: self.without_content,
            time_zone: self.time_zone.clone(),
        };

        // #439: method 来自 catalog
        let api_request: ApiRequest<CopyAppResponse> =
            api_endpoint.to_request::<CopyAppResponse>().body(
                openlark_core::api::RequestData::Binary(serde_json::to_vec(&request_body)?),
            );

        // 发送请求
        Transport::request_typed(
            api_request,
            &self.config,
            Some(option),
            "Bitable 复制多维表格API",
        )
        .await
    }
}

impl AppService {
    /// 创建复制多维表格请求 builder。
    pub fn copy_builder(&self, app_token: impl Into<String>) -> CopyAppRequest {
        CopyAppRequest::new(self.config.clone()).app_token(app_token)
    }

    /// 创建复制多维表格请求（带完整参数）
    pub fn copy_app(
        &self,
        app_token: impl Into<String>,
        name: Option<impl Into<String>>,
        folder_token: Option<impl Into<String>>,
        without_content: Option<bool>,
        time_zone: Option<impl Into<String>>,
    ) -> CopyAppRequest {
        let mut request = CopyAppRequest::new(self.config.clone()).app_token(app_token);

        if let Some(name) = name {
            request = request.name(name);
        }

        if let Some(folder_token) = folder_token {
            request = request.folder_token(folder_token);
        }

        if let Some(without_content) = without_content {
            request = request.without_content(without_content);
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
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST .../apps/{app_token}/copy → CopyAppResponse。
    #[tokio::test]
    async fn test_copy_app_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/bitable/v1/apps/app001/copy"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "app": { "app_token": "app001", "name": "副本" } }
            })))
            .mount(&server).await;
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();
        CopyAppRequest::new(config)
            .app_token("app001")
            .name("副本")
            .execute()
            .await
            .expect("复制多维表格应成功");
        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/bitable/v1/apps/app001/copy"
        );
    }
}
