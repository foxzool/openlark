/// CCM Sheet V2 工作表模型
use serde::{Deserialize, Serialize};

/// 添加工作表请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddSheetParams {
    /// 工作表标题
    #[serde(rename = "title")]
    pub title: String,
    /// 插入位置（工作表ID）
    #[serde(rename = "index")]
    pub index: Option<String>,
}

/// 添加工作表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddSheetResponse {
    /// 工作表添加结果
    pub data: Option<AddSheetResult>,
}

/// 添加工作表结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddSheetResult {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
    /// 工作表标题
    #[serde(rename = "title")]
    pub title: String,
    /// 工作表索引
    #[serde(rename = "index")]
    pub index: i32,
}

/// 获取工作表信息请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSheetParams {
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
}

/// 获取工作表信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSheetResponse {
    /// 工作表信息
    pub data: Option<SheetDetailInfo>,
}

/// 更新工作表请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSheetParams {
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
    /// 工作表标题
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// 工作表索引
    #[serde(rename = "index")]
    pub index: Option<i32>,
}

/// 更新工作表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSheetResponse {
    /// 工作表更新结果
    pub data: Option<UpdateSheetResult>,
}

/// 更新工作表结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSheetResult {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
    /// 工作表标题
    #[serde(rename = "title")]
    pub title: String,
    /// 工作表索引
    #[serde(rename = "index")]
    pub index: i32,
}

/// 删除工作表请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSheetParams {
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
}

/// 删除工作表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSheetResponse {
    /// 工作表删除结果
    pub data: Option<DeleteSheetResult>,
}

/// 工作表详细信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetDetailInfo {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
    /// 工作表标题
    #[serde(rename = "title")]
    pub title: String,
    /// 工作表类型
    #[serde(rename = "sheet_type")]
    pub sheet_type: String,
    /// 行数
    #[serde(rename = "row_count")]
    pub row_count: i32,
    /// 列数
    #[serde(rename = "column_count")]
    pub column_count: i32,
    /// 工作表索引
    #[serde(rename = "index")]
    pub index: i32,
}

/// 删除工作表结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSheetResult {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
}
