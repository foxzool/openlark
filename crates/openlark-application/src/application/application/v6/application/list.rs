//! 获取企业安装的应用
//! docPath: <https://open.feishu.cn/document/application-v6/admin/list>

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

/// 获取企业安装的应用的请求。
#[derive(Debug, Clone)]
pub struct ListApplicationsRequest {
    config: Arc<Config>,
    /// 查询参数 `page_size`（可选，分页大小，默认 50，最大 50）。
    page_size: Option<i32>,
    /// 查询参数 `page_token`（可选，分页标记，首次请求不填）。
    page_token: Option<String>,
    /// 查询参数 `user_id_type`（可选，用户 ID 类型，默认 open_id）。
    user_id_type: Option<String>,
    /// 查询参数 `lang`（必填，应用信息返回语言，可选 zh_cn/en_us/ja_jp）。
    lang: Option<String>,
    /// 查询参数 `status`（可选，应用状态过滤：0-停用 / 1-启用 / 2-未启用，不传代表全部）。
    status: Option<i32>,
    /// 查询参数 `payment_type`（可选，付费类型过滤：0-免费 / 1-付费，不传代表全部）。
    payment_type: Option<i32>,
    /// 查询参数 `owner_type`（可选，所有者类型过滤：0-飞书科技 / 1-飞书合作伙伴 / 2-企业内成员，不传代表全部）。
    owner_type: Option<i32>,
}

/// 获取企业安装的应用的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListApplicationsResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for ListApplicationsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListApplicationsRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            user_id_type: None,
            lang: None,
            status: None,
            payment_type: None,
            owner_type: None,
        }
    }

    /// 设置分页大小（可选，默认 50，最大 50）。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（可选，首次请求不填）。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置用户 ID 类型（可选，默认 open_id）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置应用信息返回语言（必填，可选 zh_cn/en_us/ja_jp）。
    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.lang = Some(lang.into());
        self
    }

    /// 设置应用状态过滤（可选：0-停用 / 1-启用 / 2-未启用，不传代表全部）。
    pub fn status(mut self, status: i32) -> Self {
        self.status = Some(status);
        self
    }

    /// 设置付费类型过滤（可选：0-免费 / 1-付费，不传代表全部）。
    pub fn payment_type(mut self, payment_type: i32) -> Self {
        self.payment_type = Some(payment_type);
        self
    }

    /// 设置所有者类型过滤（可选：0-飞书科技 / 1-飞书合作伙伴 / 2-企业内成员，不传代表全部）。
    pub fn owner_type(mut self, owner_type: i32) -> Self {
        self.owner_type = Some(owner_type);
        self
    }

    /// 执行获取企业安装的应用请求。
    pub async fn execute(self) -> SDKResult<ListApplicationsResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListApplicationsResponse> {
        // lang 官方必填（zh_cn/en_us/ja_jp），空白字符串视为未填写。
        let lang = self
            .lang
            .ok_or_else(|| CoreError::validation_msg("lang 不能为空"))?;
        validate_required!(lang, "lang 不能为空");

        let path = "/open-apis/application/v6/applications";
        let req: ApiRequest<ListApplicationsResponse> = ApiRequest::get(path)
            .query_opt("page_size", self.page_size.map(|v| v.to_string()))
            .query_opt("page_token", self.page_token)
            .query_opt("user_id_type", self.user_id_type)
            .query("lang", lang)
            .query_opt("status", self.status.map(|v| v.to_string()))
            .query_opt("payment_type", self.payment_type.map(|v| v.to_string()))
            .query_opt("owner_type", self.owner_type.map(|v| v.to_string()));

        Transport::request_typed(req, &self.config, Some(option), "获取企业安装的应用").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET /open-apis/application/v6/applications?lang=... → 强类型 ListApplicationsResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_list_applications_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/application/v6/applications"))
            .and(query_param("lang", "zh_cn"))
            .and(query_param("page_size", "20"))
            .and(query_param("status", "1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "items": [] } }
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

        let resp = ListApplicationsRequest::new(config)
            .lang("zh_cn")
            .page_size(20)
            .status(1)
            .execute()
            .await
            .expect("获取企业安装的应用应成功");
        assert!(resp.data.unwrap()["items"].is_array());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/application/v6/applications"
        );
        let query = received[0].url.query().unwrap_or_default();
        assert!(query.contains("lang=zh_cn"));
        assert!(query.contains("page_size=20"));
        assert!(query.contains("status=1"));
    }

    /// lang 官方必填：缺省报校验错误，不发起请求。
    #[tokio::test]
    async fn test_list_applications_requires_lang() {
        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .enable_token_cache(false)
                .build(),
        );

        let err = ListApplicationsRequest::new(config)
            .execute()
            .await
            .expect_err("缺少 lang 应失败");
        assert!(err.to_string().contains("lang 不能为空"));
    }
}
