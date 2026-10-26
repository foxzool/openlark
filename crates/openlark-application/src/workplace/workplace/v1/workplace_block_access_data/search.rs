//! 工作台小组件访问数据
//!
//! 文档: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/workplace-v1/workplace_block_access_data/search>
//! docPath: <https://open.feishu.cn/document/workplace-v1/workplace_access_data/search-2>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    error::CoreError,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 工作台小组件访问数据查询 Builder
#[derive(Debug, Clone)]
pub struct AccessDataSearchBlockRequestBuilder {
    config: Config,
    /// 查询参数 `from_date`（必填，数据检索开始时间，格式 yyyy-MM-dd）。
    from_date: Option<String>,
    /// 查询参数 `to_date`（必填，数据检索结束时间，格式 yyyy-MM-dd）。
    to_date: Option<String>,
    /// 查询参数 `page_size`（必填，分页大小，取值范围 1-200，默认 20）。
    page_size: Option<i32>,
    /// 查询参数 `page_token`（可选，分页标记，首次请求不填）。
    page_token: Option<String>,
    /// 查询参数 `block_id`（可选，小组件 id，不填返回全部小组件数据）。
    block_id: Option<String>,
}

impl AccessDataSearchBlockRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            from_date: None,
            to_date: None,
            page_size: None,
            page_token: None,
            block_id: None,
        }
    }

    /// 设置数据检索开始时间（必填，格式 yyyy-MM-dd）
    pub fn from_date(mut self, from_date: impl Into<String>) -> Self {
        self.from_date = Some(from_date.into());
        self
    }

    /// 设置数据检索结束时间（必填，格式 yyyy-MM-dd）
    pub fn to_date(mut self, to_date: impl Into<String>) -> Self {
        self.to_date = Some(to_date.into());
        self
    }

    /// 设置分页大小（必填，取值范围 1-200，默认 20）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（可选，首次请求不填）
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置小组件 id（可选，不填返回全部小组件数据）
    pub fn block_id(mut self, block_id: impl Into<String>) -> Self {
        self.block_id = Some(block_id.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<AccessDataSearchBlockResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<AccessDataSearchBlockResponse> {
        // from_date / to_date 官方必填（格式 yyyy-MM-dd），空白字符串视为未填写。
        let from_date = self
            .from_date
            .ok_or_else(|| CoreError::validation_msg("from_date 不能为空"))?;
        validate_required!(from_date, "from_date 不能为空");
        let to_date = self
            .to_date
            .ok_or_else(|| CoreError::validation_msg("to_date 不能为空"))?;
        validate_required!(to_date, "to_date 不能为空");
        // page_size 官方必填，取值范围 [1, 200]（超范围服务端报 1629006）。
        let page_size = self
            .page_size
            .ok_or_else(|| CoreError::validation_msg("page_size 不能为空"))?;
        if !(1..=200).contains(&page_size) {
            return Err(CoreError::validation_msg("page_size 取值范围为 1-200"));
        }

        let req: ApiRequest<AccessDataSearchBlockResponse> =
            ApiRequest::post("/open-apis/workplace/v1/workplace_block_access_data/search")
                .query("from_date", from_date)
                .query("to_date", to_date)
                .query("page_size", page_size.to_string())
                .query_opt("page_token", self.page_token)
                .query_opt("block_id", self.block_id);

        Transport::request_typed(
            req,
            &self.config,
            Some(option),
            "获取定制工作台小组件访问数据",
        )
        .await
    }
}

/// 工作台小组件访问数据查询响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccessDataSearchBlockResponse {
    /// 小组件访问数据列表
    #[serde(default)]
    pub items: Vec<BlockAccessData>,
    /// 是否还有更多项
    #[serde(default)]
    pub has_more: bool,
    /// 分页标记，has_more 为 true 时返回
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 小组件访问数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BlockAccessData {
    /// 时间，精确到天，格式 yyyy-MM-dd
    pub date: String,
    /// 小组件 id（BlockID）
    pub block_id: String,
    /// 小组件访问数据
    pub access_data: AccessData,
}

/// 访问数据（访问次数 / 访问用户数）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccessData {
    /// 访问次数
    pub pv: i64,
    /// 访问用户数（去重）
    pub uv: i64,
}

impl ApiResponseTrait for AccessDataSearchBlockResponse {}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(
    note = "renamed to AccessDataSearchBlockRequestBuilder, will be removed in v1.0 (#271)"
)]
pub type AccessDataSearchBlockBuilder = AccessDataSearchBlockRequestBuilder;

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：POST .../workplace_block_access_data/search?from_date=...&to_date=...&page_size=...
    /// → 强类型 AccessDataSearchBlockResponse 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_search_workplace_block_access_data_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/workplace/v1/workplace_block_access_data/search",
            ))
            .and(query_param("from_date", "2026-01-01"))
            .and(query_param("to_date", "2026-01-02"))
            .and(query_param("page_size", "20"))
            .and(query_param("page_token", "ddowkdkl9w2d"))
            .and(query_param("block_id", "283438293839422334"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [ {
                        "date": "2026-01-01",
                        "block_id": "283438293839422334",
                        "access_data": { "pv": 100, "uv": 30 }
                    } ],
                    "has_more": true,
                    "page_token": "ddowkdkl9w2d"
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = AccessDataSearchBlockRequestBuilder::new(config)
            .from_date("2026-01-01")
            .to_date("2026-01-02")
            .page_size(20)
            .page_token("ddowkdkl9w2d")
            .block_id("283438293839422334")
            .execute()
            .await
            .expect("工作台小组件访问数据查询应成功");
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.items[0].date, "2026-01-01");
        assert_eq!(resp.items[0].block_id, "283438293839422334");
        assert_eq!(resp.items[0].access_data.pv, 100);
        assert_eq!(resp.items[0].access_data.uv, 30);
        assert!(resp.has_more);
        assert_eq!(resp.page_token.as_deref(), Some("ddowkdkl9w2d"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/workplace/v1/workplace_block_access_data/search"
        );
    }

    /// 缺少必填查询参数 from_date 时应在校验层快速失败，不发起请求。
    #[tokio::test]
    async fn test_search_workplace_block_access_data_missing_from_date_fails() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let err = AccessDataSearchBlockRequestBuilder::new(config)
            .to_date("2026-01-02")
            .page_size(20)
            .execute()
            .await
            .expect_err("缺少 from_date 应返回校验错误");
        assert!(
            err.to_string().contains("from_date"),
            "错误信息应点名 from_date，实际: {err}"
        );
    }
}
