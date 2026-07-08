/// CCM Sheet V2 表格基础模型
use serde::{Deserialize, Serialize};

/// 获取表格信息请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSpreadsheetParams {
    /// 是否包含工作表信息
    #[serde(rename = "include_sheet")]
    pub include_sheet: Option<bool>,
}

/// 获取表格信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSpreadsheetResponse {
    /// 表格信息
    pub data: Option<SpreadsheetInfo>,
}

/// 表格信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpreadsheetInfo {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 表格标题
    #[serde(rename = "title")]
    pub title: String,
    /// 创建者信息
    #[serde(rename = "create_time")]
    pub create_time: i64,
    /// 编辑者信息
    #[serde(rename = "editor")]
    pub editor: Option<UserInfo>,
    /// 工作表列表
    #[serde(rename = "sheets")]
    pub sheets: Option<Vec<SpreadsheetSheetInfo>>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// 用户ID类型
    #[serde(rename = "user_id_type")]
    pub user_id_type: String,
    /// 用户名称
    #[serde(rename = "name")]
    pub name: Option<String>,
}

/// 工作表信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpreadsheetSheetInfo {
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
}

/// 创建表格请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSpreadsheetParams {
    /// 表格标题
    #[serde(rename = "title")]
    pub title: String,
    /// 文件夹token（可选）
    #[serde(rename = "folder_token")]
    pub folder_token: Option<String>,
}

/// 创建表格响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSpreadsheetResponse {
    /// 表格创建结果
    pub data: Option<CreateSpreadsheetResult>,
}

/// 创建表格结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSpreadsheetResult {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 表格标题
    #[serde(rename = "title")]
    pub title: String,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub create_time: i64,
}

/// 更新表格请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSpreadsheetParams {
    /// 表格标题
    #[serde(rename = "title")]
    pub title: Option<String>,
}

/// 更新表格响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSpreadsheetResponse {
    /// 表格更新结果
    pub data: Option<UpdateSpreadsheetResult>,
}

/// 更新表格结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSpreadsheetResult {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 表格标题
    #[serde(rename = "title")]
    pub title: String,
    /// 更新时间
    #[serde(rename = "update_time")]
    pub update_time: i64,
}
