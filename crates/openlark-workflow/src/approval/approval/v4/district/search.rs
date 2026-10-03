//! 搜索地理库信息
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/district/search>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 搜索区划的请求体。
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct SearchDistrictsBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 区划 ID 列表。
    pub district_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 关键词。
    pub keyword: Option<String>,
}

/// 搜索区划的响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchDistrictsResponse {
    #[serde(default)]
    /// 列表项。
    pub items: Vec<super::list::DistrictItem>,
}

impl ApiResponseTrait for SearchDistrictsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索区划的请求。
#[derive(Debug, Clone)]
pub struct SearchDistrictsRequest {
    config: Arc<Config>,
    locale: Option<String>,
    body: SearchDistrictsBody,
}

impl SearchDistrictsRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            locale: None,
            body: SearchDistrictsBody::default(),
        }
    }
    /// 设置语言。
    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
        self
    }
    /// 设置区划 ID 列表。
    pub fn district_ids(mut self, district_ids: Vec<String>) -> Self {
        self.body.district_ids = Some(district_ids);
        self
    }
    /// 设置关键词。
    pub fn keyword(mut self, keyword: impl Into<String>) -> Self {
        self.body.keyword = Some(keyword.into());
        self
    }
    /// 执行搜索区划请求。
    pub async fn execute(self) -> SDKResult<SearchDistrictsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }
    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SearchDistrictsResponse> {
        let mut request =
            ApiRequest::<SearchDistrictsResponse>::post("/open-apis/approval/v4/districts/search");
        if let Some(locale) = self.locale {
            request = request.query("locale", locale);
        }
        request = request.body(serde_json::to_value(&self.body)?);
        Transport::request_typed(request, &self.config, Some(option), "搜索地理库信息").await
    }
}
