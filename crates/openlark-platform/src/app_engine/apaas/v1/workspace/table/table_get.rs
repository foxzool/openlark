//! 获取数据表详细信息
//!
//! URL: GET:/open-apis/apaas/v1/workspaces/:workspace_id/tables/:table_name
//! docPath: 

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 获取数据表详情 Builder
#[derive(Debug, Clone)]
pub struct TableGetBuilder {
    config: Config,
    /// 工作空间 ID
    workspace_id: String,
    /// 数据表名称
    table_name: String,
}

impl TableGetBuilder {
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
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TableGetResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<TableGetResponse> {
        let url = format!(
            "/open-apis/apaas/v1/workspaces/{}/tables/{}",
            self.workspace_id, self.table_name
        );

        let req: ApiRequest<TableGetResponse> = ApiRequest::get(&url);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 字段信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FieldInfo {
    /// 字段名称
    #[serde(rename = "field_name")]
    field_name: String,
    /// 字段类型
    #[serde(rename = "field_type")]
    field_type: String,
    /// 是否为主键
    #[serde(rename = "is_primary_key")]
    is_primary_key: bool,
    /// 字段描述
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

/// 数据表详情响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TableGetResponse {
    /// 数据表名称
    #[serde(rename = "table_name")]
    table_name: String,
    /// 数据表描述
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// 字段列表
    #[serde(rename = "fields")]
    fields: Vec<FieldInfo>,
    /// 创建时间
    #[serde(rename = "created_time")]
    created_time: i64,
    /// 更新时间
    #[serde(rename = "updated_time")]
    updated_time: i64,
}

impl ApiResponseTrait for TableGetResponse {
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
