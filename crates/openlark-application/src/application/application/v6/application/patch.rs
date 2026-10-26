//! 更新应用分组信息
//! docPath: <https://open.feishu.cn/document/server-docs/application-v6/application/patch>

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

/// 应用回调配置（`callback_info`）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchApplicationCallbackInfo {
    /// 回调类型：webhook-发送至开发者服务器 / websocket-长连接接收回调。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_type: Option<String>,
    /// 回调地址。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_url: Option<String>,
    /// 订阅的回调列表（长度范围 0～99999）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed_callbacks: Option<Vec<String>>,
}

/// 更新应用分组信息的请求体（全部字段可选）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchApplicationBody {
    /// 应用分类的国际化描述（长度范围 1～3）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_categories: Option<Vec<String>>,
    /// 是否允许刷新 user_access_token。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_refresh_token: Option<bool>,
    /// 应用回调配置。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_info: Option<PatchApplicationCallbackInfo>,
}

/// 更新应用分组信息的请求。
#[derive(Debug, Clone)]
pub struct PatchApplicationRequest {
    config: Arc<Config>,
    app_id: String,
    /// 查询参数 `lang`（必填，返回语言，可选 zh_cn/en_us/ja_jp）。
    lang: Option<String>,
    body: PatchApplicationBody,
}

/// 更新应用分组信息的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchApplicationResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for PatchApplicationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl PatchApplicationRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
            lang: None,
            body: PatchApplicationBody::default(),
        }
    }

    /// 设置返回语言（必填，可选 zh_cn/en_us/ja_jp；应用分类取值需与该语言对应）。
    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.lang = Some(lang.into());
        self
    }

    /// 设置应用分类的国际化描述（可选，长度范围 1～3）。
    pub fn common_categories(mut self, common_categories: Vec<String>) -> Self {
        self.body.common_categories = Some(common_categories);
        self
    }

    /// 设置是否允许刷新 user_access_token（可选）。
    pub fn allow_refresh_token(mut self, allow_refresh_token: bool) -> Self {
        self.body.allow_refresh_token = Some(allow_refresh_token);
        self
    }

    /// 设置应用回调配置（可选）。
    pub fn callback_info(mut self, callback_info: PatchApplicationCallbackInfo) -> Self {
        self.body.callback_info = Some(callback_info);
        self
    }

    /// 执行更新应用分组信息请求。
    pub async fn execute(self) -> SDKResult<PatchApplicationResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<PatchApplicationResponse> {
        // lang 官方必填（zh_cn/en_us/ja_jp），空白字符串视为未填写。
        let lang = self
            .lang
            .ok_or_else(|| CoreError::validation_msg("lang 不能为空"))?;
        validate_required!(lang, "lang 不能为空");

        let body = serde_json::to_value(&self.body)?;
        let path = format!("/open-apis/application/v6/applications/{}", self.app_id);
        let req: ApiRequest<PatchApplicationResponse> =
            ApiRequest::patch(&path).query("lang", lang).body(body);

        Transport::request_typed(req, &self.config, Some(option), "更新应用分组信息").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：PATCH .../applications/{app_id}?lang=zh_cn + 官方 body → 强类型 PatchApplicationResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_patch_application_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/application/v6/applications/cli_test_app"))
            .and(query_param("lang", "zh_cn"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "app_id": "cli_test_app" } }
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

        let resp = PatchApplicationRequest::new(config, "cli_test_app")
            .lang("zh_cn")
            .common_categories(vec!["分析工具".to_string()])
            .allow_refresh_token(false)
            .execute()
            .await
            .expect("更新应用分组信息应成功");
        assert_eq!(resp.data.unwrap()["app_id"], "cli_test_app");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/application/v6/applications/cli_test_app"
        );
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["common_categories"][0], "分析工具");
        assert_eq!(sent["allow_refresh_token"], false);
    }
}
