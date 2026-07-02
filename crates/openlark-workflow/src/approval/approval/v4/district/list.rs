//! 查询地理库信息
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/district/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 区划基础信息。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DistrictBaseInfo {
    /// ID。
    pub id: String,
    /// 名称。
    pub name: String,
    /// 层级。
    pub level: String,
}

/// 区划项。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DistrictItem {
    /// ID。
    pub id: String,
    /// 名称。
    pub name: String,
    /// 层级。
    pub level: String,
    #[serde(default)]
    /// 是否含下级区划。
    pub has_sub_district: bool,
    #[serde(default)]
    /// 上级区划。
    pub parent_districts: Vec<DistrictBaseInfo>,
}

/// 查询区划列表的响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListDistrictsResponse {
    /// 版本。
    pub version: String,
    #[serde(default)]
    /// 是否有更多。
    pub has_more: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 分页标记。
    pub page_token: Option<String>,
    #[serde(default)]
    /// 列表项。
    pub items: Vec<DistrictItem>,
}

impl ApiResponseTrait for ListDistrictsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询区划列表的请求。
#[derive(Debug, Clone)]
pub struct ListDistrictsRequest {
    config: Arc<Config>,
    page_size: Option<i32>,
    page_token: Option<String>,
    root_district_id: Option<String>,
    list_type: Option<String>,
    locale: Option<String>,
}

impl ListDistrictsRequest {
    /// 创建请求实例。
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
    /// 设置分页大小。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }
    /// 设置分页标记。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }
    /// 设置根区划 ID。
    pub fn root_district_id(mut self, root_district_id: impl Into<String>) -> Self {
        self.root_district_id = Some(root_district_id.into());
        self
    }
    /// 设置列表类型。
    pub fn list_type(mut self, list_type: impl Into<String>) -> Self {
        self.list_type = Some(list_type.into());
        self
    }
    /// 设置语言。
    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
        self
    }
    /// 执行查询区划列表请求。
    pub async fn execute(self) -> SDKResult<ListDistrictsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }
    /// 带自定义请求选项执行。
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
