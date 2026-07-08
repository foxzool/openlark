/// CCM Sheet V2 筛选模型
use serde::{Deserialize, Serialize};

/// 创建筛选请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFilterParams {
    /// 筛选范围
    #[serde(rename = "range")]
    pub range: String,
    /// 筛选条件
    #[serde(rename = "filter_spec")]
    pub filter_spec: FilterSpec,
}

/// 筛选规格
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterSpec {
    /// 筛选条件列表
    #[serde(rename = "filter_specs")]
    pub filter_specs: Vec<FilterCondition>,
}

/// 筛选条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterCondition {
    /// 列索引
    #[serde(rename = "column_index")]
    pub column_index: i32,
    /// 筛选操作符
    #[serde(rename = "operator")]
    pub operator: String,
    /// 比较值
    #[serde(rename = "compare_values")]
    pub compare_values: Vec<serde_json::Value>,
}

/// 创建筛选响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFilterResponse {
    /// 筛选结果
    pub data: Option<FilterResult>,
}

/// 筛选结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterResult {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 筛选ID
    #[serde(rename = "filter_id")]
    pub filter_id: String,
    /// 筛选范围
    #[serde(rename = "range")]
    pub range: String,
}

/// 获取筛选请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFilterParams {
    /// 筛选范围
    #[serde(rename = "range")]
    pub range: String,
}

/// 获取筛选响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFilterResponse {
    /// 筛选信息
    pub data: Option<FilterInfo>,
}

/// 筛选信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterInfo {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 筛选ID
    #[serde(rename = "filter_id")]
    pub filter_id: String,
    /// 筛选范围
    #[serde(rename = "range")]
    pub range: String,
    /// 筛选规格
    #[serde(rename = "filter_spec")]
    pub filter_spec: FilterSpec,
}

/// 更新筛选请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFilterParams {
    /// 筛选ID
    #[serde(rename = "filter_id")]
    pub filter_id: String,
    /// 筛选规格
    #[serde(rename = "filter_spec")]
    pub filter_spec: FilterSpec,
}

/// 更新筛选响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFilterResponse {
    /// 更新结果
    pub data: Option<FilterResult>,
}

/// 删除筛选请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFilterParams {
    /// 筛选ID
    #[serde(rename = "filter_id")]
    pub filter_id: String,
}

/// 删除筛选响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFilterResponse {
    /// 删除结果
    pub data: Option<DeleteFilterResult>,
}

/// 删除筛选结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFilterResult {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 筛选ID
    #[serde(rename = "filter_id")]
    pub filter_id: String,
}
