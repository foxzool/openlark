/// Sheets 电子表格 v3 数据模型
///
/// 注意：这些结构体对应「电子表格」相关接口响应体中的 data 字段结构。
use serde::{Deserialize, Serialize};

// ============================================================================
// spreadsheet
// ============================================================================

/// 创建电子表格请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateSpreadsheetParams {
    /// 表格标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 文件夹 token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
}

/// 创建电子表格响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSpreadsheetResponse {
    /// 新建的电子表格信息
    pub spreadsheet: CreatedSpreadsheet,
}

/// 创建电子表格返回的表格信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatedSpreadsheet {
    /// 电子表格标题
    pub title: String,
    /// 所属文件夹 token（创建时指定）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
    /// 电子表格访问 URL
    pub url: String,
    /// 电子表格唯一 token
    pub spreadsheet_token: String,
}

/// 修改电子表格属性请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateSpreadsheetParams {
    /// 新的电子表格标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// 修改电子表格属性响应体 data（data 为 `{}`）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateSpreadsheetResponse {}

/// 获取电子表格信息响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSpreadsheetResponse {
    /// 电子表格信息
    pub spreadsheet: SpreadsheetInfo,
}

/// 电子表格信息（获取电子表格信息接口返回）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpreadsheetInfo {
    /// 电子表格标题
    pub title: String,
    /// 表格拥有者（创建者）用户 ID
    pub owner_id: String,
    /// 表格 token（等价 spreadsheet_token）
    pub token: String,
    /// 电子表格访问 URL
    pub url: String,
}

// ============================================================================
// spreadsheet.sheet
// ============================================================================

/// 获取工作表列表响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuerySheetResponse {
    /// 工作表列表
    pub sheets: Vec<Sheet>,
}

/// 查询工作表响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSheetResponse {
    /// 工作表信息
    pub sheet: Sheet,
}

/// 工作表属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sheet {
    /// 工作表 ID
    pub sheet_id: String,
    /// 工作表标题
    pub title: String,
    /// 工作表在工作簿中的位置索引（从 0 开始）
    pub index: i32,
    /// 工作表是否隐藏
    pub hidden: bool,
    /// 工作表网格属性
    pub grid_properties: GridProperties,
    /// 资源类型（sheet / grid / chart 等）
    pub resource_type: String,
    /// 合并单元格范围列表（无合并时为 None）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merges: Option<Vec<MergeRange>>,
}

/// 网格属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridProperties {
    /// 冻结的行数
    pub frozen_row_count: i32,
    /// 冻结的列数
    pub frozen_column_count: i32,
    /// 总行数
    pub row_count: i32,
    /// 总列数
    pub column_count: i32,
}

/// 合并单元格范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeRange {
    /// 合并区域起始行索引（含，从 0 开始）
    pub start_row_index: i32,
    /// 合并区域结束行索引（不含）
    pub end_row_index: i32,
    /// 合并区域起始列索引（含，从 0 开始）
    pub start_column_index: i32,
    /// 合并区域结束列索引（不含）
    pub end_column_index: i32,
}

/// 移动行列请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveDimensionParams {
    /// 移动源位置信息
    pub source: DimensionSource,
    /// 移动的目标位置行或者列
    pub destination_index: i32,
}

/// 移动源位置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionSource {
    /// ROWS 或 COLUMNS
    pub major_dimension: String,
    /// 移动起始索引（含，从 0 开始）
    pub start_index: i32,
    /// 移动结束索引（不含）
    pub end_index: i32,
}

/// 移动行列响应体 data（data 为 `{}`）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MoveDimensionResponse {}

// ============================================================================
// spreadsheet.sheet.find / replace
// ============================================================================

/// 查找条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindCondition {
    /// 查找范围（A1 或 R1C1 格式，如 `Sheet1!A1:C10`）
    pub range: String,
    /// 是否区分大小写
    pub match_case: bool,
    /// 是否整单元格匹配
    pub match_entire_cell: bool,
    /// 是否按正则表达式查找
    pub search_by_regex: bool,
    /// 是否在公式中查找
    pub include_formulas: bool,
}

/// 查找单元格请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindParams {
    /// 查找条件
    pub find_condition: FindCondition,
    /// 要查找的内容
    pub find: String,
}

/// 查找单元格响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindResponse {
    /// 查找结果
    pub find_result: FindResult,
}

/// 替换单元格请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindReplaceParams {
    /// 查找条件
    pub find_condition: FindCondition,
    /// 要查找的内容
    pub find: String,
    /// 替换为的内容
    pub replacement: String,
}

/// 替换单元格响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindReplaceResponse {
    /// 替换结果
    pub replace_result: FindResult,
}

/// 查找/替换结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindResult {
    /// 命中查找条件的单元格列表
    pub matched_cells: Vec<String>,
    /// 命中查找条件的公式单元格列表
    pub matched_formula_cells: Vec<String>,
    /// 命中结果的行数
    pub rows_count: i32,
}
