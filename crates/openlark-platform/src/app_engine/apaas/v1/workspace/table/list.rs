//! 获取工作空间下的数据表列表
//!
//! URL: GET:/open-apis/apaas/v1/workspaces/:workspace_id/tables
//! docPath: 

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 获取数据表列表 Builder
#[derive(Debug, Clone)]
pub struct TableListBuilder {
    config: Config,
    /// 工作空间 ID
    workspace_id: String,
}

impl TableListBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, workspace_id: impl Into<String>) -> Self {
        Self {
            config,
            workspace_id: workspace_id.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TableListResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<TableListResponse> {
        let url = format!(
            "/open-apis/apaas/v1/workspaces/{}/tables",
            self.workspace_id
        );

        let req: ApiRequest<TableListResponse> = ApiRequest::get(&url);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 数据表信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TableInfo {
    /// 数据表名称
    #[serde(rename = "table_name")]
    table_name: String,
    /// 数据表描述
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// 创建时间
    #[serde(rename = "created_time")]
    created_time: i64,
    /// 更新时间
    #[serde(rename = "updated_time")]
    updated_time: i64,
}

/// 数据表列表响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TableListResponse {
    /// 数据表列表
    #[serde(rename = "items")]
    items: Vec<TableInfo>,
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

impl ApiResponseTrait for TableListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = TableListBuilder::new(config.clone(), "test".to_string());
        let _ = request;
    }
}
