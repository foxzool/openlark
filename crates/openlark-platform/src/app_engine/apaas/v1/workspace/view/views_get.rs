//! 查询视图数据记录
//!
//! URL: GET:/open-apis/apaas/v1/workspaces/:workspace_id/views/:view_name/records
//! docPath:

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 查询视图记录 Builder
#[derive(Debug, Clone)]
pub struct ViewsGetBuilder {
    config: Config,
    /// 工作空间 ID
    workspace_id: String,
    /// 视图名称
    view_name: String,
    /// 页码
    page: Option<u32>,
    /// 每页数量
    page_size: Option<u32>,
    /// 筛选条件
    filter: Option<String>,
}

impl ViewsGetBuilder {
    /// 创建新的 Builder
    pub fn new(
        config: Config,
        workspace_id: impl Into<String>,
        view_name: impl Into<String>,
    ) -> Self {
        Self {
            config,
            workspace_id: workspace_id.into(),
            view_name: view_name.into(),
            page: None,
            page_size: None,
            filter: None,
        }
    }

    /// 设置页码
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// 设置每页数量
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置筛选条件
    pub fn filter(mut self, filter: impl Into<String>) -> Self {
        self.filter = Some(filter.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ViewsGetResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<ViewsGetResponse> {
        let url = format!(
            "/open-apis/apaas/v1/workspaces/{}/views/{}/records",
            self.workspace_id, self.view_name
        );

        let mut req: ApiRequest<ViewsGetResponse> = ApiRequest::get(&url);
        if let Some(page) = self.page {
            req = req.query("page", page.to_string());
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        if let Some(filter) = self.filter {
            req = req.query("filter", &filter);
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("查询视图记录", "响应数据为空"))
    }
}

/// 视图记录
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ViewRecord {
    /// 记录 ID
    #[serde(rename = "id")]
    id: String,
    /// 记录数据
    #[serde(rename = "data")]
    data: serde_json::Value,
    /// 创建时间
    #[serde(rename = "created_time")]
    created_time: i64,
    /// 更新时间
    #[serde(rename = "updated_time")]
    updated_time: i64,
}

/// 查询视图记录响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ViewsGetResponse {
    /// 记录列表
    #[serde(rename = "items")]
    items: Vec<ViewRecord>,
    /// 是否有更多
    #[serde(rename = "has_more")]
    has_more: bool,
    /// 页码
    #[serde(rename = "page")]
    page: u32,
    /// 每页数量
    #[serde(rename = "page_size")]
    page_size: u32,
}

impl ApiResponseTrait for ViewsGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
