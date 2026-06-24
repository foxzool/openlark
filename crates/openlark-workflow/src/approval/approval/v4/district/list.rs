//! 查询地理库信息
//!
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/district/list

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// 待补充文档。
pub struct DistrictBaseInfo {
    /// 待补充文档。
    pub id: String,
    /// 待补充文档。
    pub name: String,
    /// 待补充文档。
    pub level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// 待补充文档。
pub struct DistrictItem {
    /// 待补充文档。
    pub id: String,
    /// 待补充文档。
    pub name: String,
    /// 待补充文档。
    pub level: String,
    #[serde(default)]
    /// 待补充文档。
    pub has_sub_district: bool,
    #[serde(default)]
    /// 待补充文档。
    pub parent_districts: Vec<DistrictBaseInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// 待补充文档。
pub struct ListDistrictsResponse {
    /// 待补充文档。
    pub version: String,
    #[serde(default)]
    /// 待补充文档。
    pub has_more: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 待补充文档。
    pub page_token: Option<String>,
    #[serde(default)]
    /// 待补充文档。
    pub items: Vec<DistrictItem>,
}

impl ApiResponseTrait for ListDistrictsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone)]
/// 待补充文档。
pub struct ListDistrictsRequest {
    config: Arc<Config>,
    page_size: Option<i32>,
    page_token: Option<String>,
    root_district_id: Option<String>,
    list_type: Option<String>,
    locale: Option<String>,
}

impl ListDistrictsRequest {
    /// 待补充文档。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            root_district_id: None,
            list_type: None,
            locale: None,
        }
    }
    /// 待补充文档。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }
    /// 待补充文档。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }
    /// 待补充文档。
    pub fn root_district_id(mut self, root_district_id: impl Into<String>) -> Self {
        self.root_district_id = Some(root_district_id.into());
        self
    }
    /// 待补充文档。
    pub fn list_type(mut self, list_type: impl Into<String>) -> Self {
        self.list_type = Some(list_type.into());
        self
    }
    /// 待补充文档。
    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
        self
    }
    /// 待补充文档。
    pub async fn execute(self) -> SDKResult<ListDistrictsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }
    /// 待补充文档。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListDistrictsResponse> {
        let mut request =
            ApiRequest::<ListDistrictsResponse>::get("/open-apis/approval/v4/districts");
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(root_district_id) = self.root_district_id {
            request = request.query("root_district_id", root_district_id);
        }
        if let Some(list_type) = self.list_type {
            request = request.query("list_type", list_type);
        }
        if let Some(locale) = self.locale {
            request = request.query("locale", locale);
        }
        let response = Transport::request(request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("查询地理库信息", "响应数据为空"))
    }
}
