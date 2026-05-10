//! 分页查询国家/地区
//!
//! 文档: https://open.feishu.cn/document/mdm-v1/mdm-v3/country_region/list
//! docPath: https://open.feishu.cn/document/mdm-v1/mdm-v3/country_region/list

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 分页查询国家/地区 Builder
#[derive(Debug, Clone)]
pub struct CountryRegionListBuilder {
    config: Config,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl CountryRegionListBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
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
        let mut url = "/open-apis/mdm/v3/country_regions".to_string();

        // 添加查询参数
        let mut params = Vec::new();
        if let Some(page_size) = self.page_size {
            params.push(format!("page_size={}", page_size));
        }
        if let Some(ref page_token) = self.page_token {
            params.push(format!("page_token={}", page_token));
        }
        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let req: ApiRequest<CountryRegionListResponse> = ApiRequest::get(&url);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = CountryRegionListBuilder::new(config.clone())
            .page_size(1)
            .page_token("test".to_string());
        let _ = request;
    }
}
