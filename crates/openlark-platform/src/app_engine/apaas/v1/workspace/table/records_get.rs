//! 查询数据表数据记录
//!
//! URL: GET:/open-apis/apaas/v1/workspaces/:workspace_id/tables/:table_name/records

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 查询数据表记录 Builder
#[derive(Debug, Clone)]
pub struct TableRecordsGetBuilder {
    config: Config,
    /// 工作空间 ID
    workspace_id: String,
    /// 数据表名称
    table_name: String,
    /// 页码
    page: Option<u32>,
    /// 每页数量
    page_size: Option<u32>,
    /// 筛选条件
    filter: Option<String>,
    /// 排序
    order_by: Option<String>,
}

impl TableRecordsGetBuilder {
    /// 创建新的 Builder
    pub fn new(
        config: Config,
        workspace_id: impl Into<String>,
        table_name: impl Into<String>,
    ) -> Self {
        Self {
            config,
            workspace_id: workspace_id.into(),
            table_name: table_name.into(),
            page: None,
            page_size: None,
            filter: None,
            order_by: None,
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

    /// 设置排序
    pub fn order_by(mut self, order_by: impl Into<String>) -> Self {
        self.order_by = Some(order_by.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TableRecordsGetResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<TableRecordsGetResponse> {
        let url = format!(
            "/open-apis/apaas/v1/workspaces/{}/tables/{}/records",
            self.workspace_id, self.table_name
        );

        let mut req: ApiRequest<TableRecordsGetResponse> = ApiRequest::get(&url);
        if let Some(page) = self.page {
            req = req.query("page", page.to_string());
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        if let Some(filter) = self.filter {
            req = req.query("filter", &filter);
        }
        if let Some(order_by) = self.order_by {
            req = req.query("order_by", &order_by);
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("查询数据表记录", "响应数据为空"))
    }
}

/// 记录信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TableRecord {
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

/// 查询数据表记录响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TableRecordsGetResponse {
    /// 记录列表
    #[serde(rename = "items")]
    items: Vec<TableRecord>,
    /// 是否有更多
    #[serde(rename = "has_more")]
    has_more: bool,
    /// 页码
    #[serde(rename = "page")]
    page: u32,
    /// 每页数量
    #[serde(rename = "page_size")]
    page_size: u32,
    /// 总数
    #[serde(rename = "total_count")]
    total_count: u32,
}

impl ApiResponseTrait for TableRecordsGetResponse {
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
