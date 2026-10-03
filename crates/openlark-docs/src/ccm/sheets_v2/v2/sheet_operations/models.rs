/// CCM Sheet V2 表格操作模型
use serde::{Deserialize, Serialize};

/// 删除范围请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRangeParams {
    /// 范围参数，如 "Sheet1!A1:C10"
    #[serde(rename = "range")]
    pub range: String,
    /// 删除维度
    #[serde(rename = "dimension")]
    pub dimension: String,
}

/// 删除范围响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRangeResponse {
    /// 删除结果
    pub data: Option<DeleteRangeResult>,
}

/// 删除范围结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRangeResult {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 删除的范围
    #[serde(rename = "deleted_range")]
    pub deleted_range: String,
}

/// 插入行列请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertDimensionParams {
    /// 插入范围
    #[serde(rename = "range")]
    pub range: String,
    /// 插入维度
    #[serde(rename = "dimension")]
    pub dimension: String,
    /// 是否包含样式
    #[serde(rename = "inherit_style")]
    pub inherit_style: Option<bool>,
}

/// 插入行列响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertDimensionResponse {
    /// 插入结果
    pub data: Option<InsertDimensionResult>,
}

/// 插入行列结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertDimensionResult {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 插入的范围
    #[serde(rename = "inserted_range")]
    pub inserted_range: String,
    /// 插入的位置
    #[serde(rename = "inserted_dimension")]
    pub inserted_dimension: String,
}

/// 移动维度请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveDimensionParams {
    /// 源范围
    #[serde(rename = "source_range")]
    pub source_range: String,
    /// 目标范围
    #[serde(rename = "destination_index")]
    pub destination_index: String,
    /// 移动维度
    #[serde(rename = "dimension")]
    pub dimension: String,
}

/// 移动维度响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveDimensionResponse {
    /// 移动结果
    pub data: Option<MoveDimensionResult>,
}

/// 移动维度结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveDimensionResult {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 移动的范围
    #[serde(rename = "moved_range")]
    pub moved_range: String,
}

/// 替换范围请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplaceRangeParams {
    /// 替换范围
    #[serde(rename = "range")]
    pub range: String,
    /// 替换规则
    #[serde(rename = "replace_rule")]
    pub replace_rule: String,
    /// 替换选项
    #[serde(rename = "replace_option")]
    pub replace_option: Option<String>,
}

/// 替换范围响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplaceRangeResponse {
    /// 替换结果
    pub data: Option<ReplaceRangeResult>,
}

/// 替换范围结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplaceRangeResult {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 替换的范围
    #[serde(rename = "replaced_range")]
    pub replaced_range: String,
    /// 替换的数量
    #[serde(rename = "replaced_count")]
    pub replaced_count: i32,
}

/// 查找请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindReplaceParams {
    /// 查找范围
    #[serde(rename = "range")]
    pub range: String,
    /// 查找内容
    #[serde(rename = "find")]
    pub find: String,
    /// 替换内容
    #[serde(rename = "replace")]
    pub replace: String,
    /// 查找选项
    #[serde(rename = "find_option")]
    pub find_option: Option<String>,
}

/// 查找替换响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindReplaceResponse {
    /// 查找替换结果
    pub data: Option<FindReplaceResult>,
}

/// 查找替换结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindReplaceResult {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 查找的范围
    #[serde(rename = "find_range")]
    pub find_range: String,
    /// 查找的数量
    #[serde(rename = "find_count")]
    pub find_count: i32,
    /// 替换的数量
    #[serde(rename = "replace_count")]
    pub replace_count: i32,
}

/// 合并单元格请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeCellsParams {
    /// 合并范围
    #[serde(rename = "range")]
    pub range: String,
    /// 合并类型
    #[serde(rename = "merge_type")]
    pub merge_type: String,
}

/// 合并单元格响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeCellsResponse {
    /// 合并结果
    pub data: Option<MergeCellsResult>,
}

/// 合并单元格结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeCellsResult {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 合并的范围
    #[serde(rename = "merge_range")]
    pub merge_range: String,
    /// 合并的类型
    #[serde(rename = "merge_type")]
    pub merge_type: String,
}

/// 取消合并单元格请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnmergeCellsParams {
    /// 取消合并范围
    #[serde(rename = "range")]
    pub range: String,
}

/// 取消合并单元格响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnmergeCellsResponse {
    /// 取消合并结果
    pub data: Option<UnmergeCellsResult>,
}

/// 取消合并单元格结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnmergeCellsResult {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 取消合并的范围
    #[serde(rename = "unmerge_range")]
    pub unmerge_range: String,
}
