//! 批量查询国家/地区
//!
//! 文档: <https://open.feishu.cn/document/mdm-v1/mdm-v3/country_region/get>
//! docPath: <https://open.feishu.cn/document/mdm-v1/mdm-v3/country_region/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 批量查询国家/地区 Builder
#[derive(Debug, Clone)]
pub struct CountryRegionBatchGetRequestBuilder {
    config: Config,
    mdm_codes: Vec<String>,
}

impl CountryRegionBatchGetRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            mdm_codes: Vec::new(),
        }
    }

    /// 添加主数据编码
    pub fn mdm_code(mut self, mdm_code: impl Into<String>) -> Self {
        self.mdm_codes.push(mdm_code.into());
        self
    }

    /// 添加多个主数据编码
    pub fn mdm_codes(mut self, mdm_codes: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.mdm_codes.extend(mdm_codes.into_iter().map(Into::into));
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CountryRegionBatchGetResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CountryRegionBatchGetResponse> {
        let url = "/open-apis/mdm/v3/batch_country_region".to_string();

        // 添加查询参数
        validate_required_list!(self.mdm_codes, 50, "mdm_codes 不能为空");

        let req: ApiRequest<CountryRegionBatchGetResponse> = ApiRequest::get(&url);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 批量查询国家/地区响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CountryRegionBatchGetResponse {
    /// 国家/地区列表
    pub items: Vec<CountryRegion>,
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

impl ApiResponseTrait for CountryRegionBatchGetResponse {}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(
    note = "renamed to CountryRegionBatchGetRequestBuilder, will be removed in v1.0 (#271)"
)]
pub type CountryRegionBatchGetBuilder = CountryRegionBatchGetRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../mdm/v3/batch_country_region → 强类型 CountryRegionBatchGetResponse。
    #[tokio::test]
    async fn test_batch_get_country_region_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/mdm/v3/batch_country_region"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "mdm_code": "CN",
                            "name": "中国",
                            "i18n_name": { "zh_cn": "中国", "en_us": "China" },
                            "phone_code": "86"
                        }
                    ]
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

        let resp = CountryRegionBatchGetRequestBuilder::new(config)
            .mdm_code("CN")
            .execute()
            .await
            .expect("批量查询国家/地区应成功");
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.items[0].mdm_code, "CN");
        assert_eq!(resp.items[0].name, "中国");
        assert_eq!(resp.items[0].phone_code.as_deref(), Some("86"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/mdm/v3/batch_country_region"
        );
    }
}
