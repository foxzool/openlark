//! 获取应用信息
//! docPath: <https://open.feishu.cn/document/server-docs/application-v6/application/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::CoreError,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取应用信息的请求。
#[derive(Debug, Clone)]
pub struct GetApplicationRequest {
    config: Arc<Config>,
    app_id: String,
    /// 查询参数 `lang`（必填，应用信息语言，可选 zh_cn/en_us/ja_jp）。
    lang: Option<String>,
    /// 查询参数 `user_id_type`（可选，用户 ID 类型，默认 open_id）。
    user_id_type: Option<String>,
}

/// 获取应用信息的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetApplicationResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetApplicationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetApplicationRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
            lang: None,
            user_id_type: None,
        }
    }

    /// 设置应用信息语言（必填，可选 zh_cn/en_us/ja_jp）。
    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.lang = Some(lang.into());
        self
    }

    /// 设置用户 ID 类型（可选，默认 open_id）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行获取应用信息请求。
    pub async fn execute(self) -> SDKResult<GetApplicationResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetApplicationResponse> {
        // lang 官方必填（zh_cn/en_us/ja_jp），空白字符串视为未填写。
        let lang = self
            .lang
            .ok_or_else(|| CoreError::validation_msg("lang 不能为空"))?;
        validate_required!(lang, "lang 不能为空");

        let path = format!("/open-apis/application/v6/applications/{}", self.app_id);
        let req: ApiRequest<GetApplicationResponse> = ApiRequest::get(&path)
            .query("lang", lang)
            .query_opt("user_id_type", self.user_id_type);

        Transport::request_typed(req, &self.config, Some(option), "获取应用信息").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../applications/{app_id}?lang=zh_cn → 强类型 GetApplicationResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_get_application_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/application/v6/applications/cli_test_app"))
            .and(query_param("lang", "zh_cn"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "app_id": "cli_test_app", "app_name": "测试应用" } }
            })))
            .mount(&server)
            .await;

        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let resp = GetApplicationRequest::new(config, "cli_test_app")
            .lang("zh_cn")
            .execute()
            .await
            .expect("获取应用信息应成功");
        assert_eq!(resp.data.unwrap()["app_id"], "cli_test_app");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/application/v6/applications/cli_test_app"
        );
    }
}
