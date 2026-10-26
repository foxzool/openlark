//! 分页查询国家/地区
//!
//! 文档: <https://open.feishu.cn/document/mdm-v1/mdm-v3/country_region/list>
//! docPath: <https://open.feishu.cn/document/mdm-v1/mdm-v3/country_region/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 分页查询国家/地区 Builder
#[derive(Debug, Clone)]
pub struct CountryRegionListRequestBuilder {
    config: Config,
    /// 希望返回的语言种类（zh-CN / en-US / ja-JP）
    languages: Vec<String>,
    /// 需要的查询字段集
    fields: Vec<String>,
    /// 查询页大小（1～1000）
    limit: Option<i32>,
    /// 查询起始位置（0～100000）
    offset: Option<i32>,
    /// 是否返回总数
    return_count: Option<bool>,
    /// 分页标记
    page_token: Option<String>,
}

impl CountryRegionListRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            languages: Vec::new(),
            fields: Vec::new(),
            limit: None,
            offset: None,
            return_count: None,
            page_token: None,
        }
    }

    /// 添加返回语言种类
    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.languages.push(language.into());
        self
    }

    /// 添加多个返回语言种类
    pub fn languages(mut self, languages: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.languages.extend(languages.into_iter().map(Into::into));
        self
    }

    /// 添加查询字段
    pub fn field(mut self, field: impl Into<String>) -> Self {
        self.fields.push(field.into());
        self
    }

    /// 添加多个查询字段
    pub fn fields(mut self, fields: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.fields.extend(fields.into_iter().map(Into::into));
        self
    }

    /// 设置查询页大小（1～1000）
    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// 设置查询起始位置（0～100000）
    pub fn offset(mut self, offset: i32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// 设置是否返回总数
    pub fn return_count(mut self, return_count: bool) -> Self {
        self.return_count = Some(return_count);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CountryRegionListResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CountryRegionListResponse> {
        // 官方必填查询参数：languages / fields
        validate_required_list!(self.languages, 100, "languages 不能为空且不能超过 100 个");
        validate_required_list!(self.fields, 100, "fields 不能为空且不能超过 100 个");

        let req: ApiRequest<CountryRegionListResponse> =
            ApiRequest::get("/open-apis/mdm/v3/country_regions")
                .query("languages", self.languages.join(","))
                .query("fields", self.fields.join(","))
                .query_opt("limit", self.limit.map(|limit| limit.to_string()))
                .query_opt("offset", self.offset.map(|offset| offset.to_string()))
                .query_opt("return_count", self.return_count.map(|rc| rc.to_string()))
                .query_opt("page_token", self.page_token);
        Transport::request_typed(req, &self.config, Some(option), "分页查询国家/地区").await
    }
}

/// 分页查询国家/地区响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CountryRegionListResponse {
    /// 国家/地区列表
    pub items: Vec<CountryRegion>,
    /// 分页标记
    #[serde(rename = "page_token")]
    pub page_token: Option<String>,
    /// 是否还有更多
    #[serde(rename = "has_more")]
    pub has_more: Option<bool>,
}

/// 国家/地区信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CountryRegion {
    /// 主数据编码
    #[serde(rename = "mdm_code")]
    pub mdm_code: String,
    /// 国家/地区名称
    pub name: String,
    /// 国际化名称
    #[serde(rename = "i18n_name")]
    pub i18n_name: Option<CountryRegionI18nName>,
    /// 电话区号
    #[serde(rename = "phone_code")]
    pub phone_code: Option<String>,
}

/// 国家/地区国际化名称
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CountryRegionI18nName {
    /// 中文名称
    #[serde(rename = "zh_cn")]
    pub zh_cn: Option<String>,
    /// 英文名称
    #[serde(rename = "en_us")]
    pub en_us: Option<String>,
    /// 日文名称
    #[serde(rename = "ja_jp")]
    pub ja_jp: Option<String>,
}

impl ApiResponseTrait for CountryRegionListResponse {}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to CountryRegionListRequestBuilder, will be removed in v1.0 (#271)")]
pub type CountryRegionListBuilder = CountryRegionListRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = CountryRegionListRequestBuilder::new(config.clone())
            .languages(vec!["zh-CN".to_string()])
            .fields(vec!["name".to_string()])
            .limit(10)
            .offset(0)
            .return_count(true)
            .page_token("test".to_string());
        let _ = request;
    }

    /// 端到端：GET .../mdm/v3/country_regions → 必填查询参数 languages/fields 按官方文档发送。
    #[tokio::test]
    async fn test_list_country_regions_sends_required_query_params() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/mdm/v3/country_regions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [],
                    "has_more": false
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

        let resp = CountryRegionListRequestBuilder::new(config)
            .languages(vec!["zh-CN".to_string(), "en-US".to_string()])
            .fields(vec!["name".to_string(), "mdm_code".to_string()])
            .limit(10)
            .return_count(true)
            .execute()
            .await
            .expect("分页查询国家/地区应成功");
        assert!(resp.items.is_empty());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let query = received[0].url.query().unwrap_or("");
        // 官方必填查询参数（数组按仓库惯例以逗号拼接，',' 编码为 %2C）
        assert!(query.contains("languages=zh-CN%2Cen-US"));
        assert!(query.contains("fields=name%2Cmdm_code"));
        assert!(query.contains("limit=10"));
        assert!(query.contains("return_count=true"));
    }
}
