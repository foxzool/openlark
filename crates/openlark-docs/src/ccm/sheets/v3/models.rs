/// Sheets v3 API 共享模型定义
///
/// 包含所有 Sheets v3 API 共享的数据结构、枚举和类型定义
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// ============================================================================
// 基础类型定义
// ============================================================================

/// 电子表格令牌类型
pub type SpreadsheetToken = String;

/// 工作表ID类型
pub type SheetId = String;

/// 筛选视图ID类型
pub type FilterViewId = String;

/// 浮动图片ID类型
pub type FloatImageId = String;

/// 浮动图片令牌类型
pub type FloatImageToken = String;

/// 范围类型
pub type Range = String;

/// 单元格引用类型
pub type CellReference = String;

// ============================================================================
// 工作表相关模型
// ============================================================================

/// 工作表属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetProperty {
    /// 工作表标题
    pub title: String,
    /// 工作表索引
    pub index: i32,
    /// 工作表ID
    pub sheet_id: Option<String>,
    /// 工作表颜色
    pub sheet_color: Option<String>,
    /// 工作表是否隐藏
    pub hidden: Option<bool>,
    /// 网格属性
    pub grid_properties: Option<GridProperties>,
}

/// 网格属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridProperties {
    /// 冻结行数
    pub frozen_row_count: Option<i32>,
    /// 冻结列数
    pub frozen_column_count: Option<i32>,
    /// 是否隐藏网格线
    pub hide_gridlines: Option<bool>,
}

/// 工作表信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetInfo {
    /// 工作表ID
    pub sheet_id: String,
    /// 工作表标题
    pub title: String,
    /// 工作表索引
    pub index: i32,
    /// 工作表颜色
    pub sheet_color: Option<String>,
    /// 工作表是否隐藏
    pub hidden: Option<bool>,
    /// 网格属性
    pub grid_properties: Option<GridProperties>,
}

// ============================================================================
// 电子表格相关模型
// ============================================================================

/// 电子表格基础信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpreadsheetInfo {
    /// 电子表格令牌
    pub spreadsheet_token: String,
    /// 电子表格标题
    pub title: String,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: Option<String>,
    /// 时区
    pub time_zone: Option<String>,
    /// 语言
    pub locale: Option<String>,
    /// 工作表列表
    pub sheets: Option<Vec<SheetInfo>>,
}

/// 时区配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeZone {
    /// 时区标识符，如 "Asia/Shanghai"
    pub time_zone: String,
}

/// 语言配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Locale {
    /// 语言代码，如 "zh_CN"
    pub locale: String,
}

/// 创建电子表格请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSpreadsheetRequest {
    /// 电子表格标题
    pub title: String,
    /// 工作表初始配置列表
    pub sheets: Option<Vec<SheetProperty>>,
    /// 时区设置
    pub time_zone: Option<TimeZone>,
    /// 语言设置
    pub locale: Option<Locale>,
    /// 文件夹路径（可选）
    pub folder_path: Option<String>,
    /// 自定义属性
    pub properties: Option<HashMap<String, Value>>,
}

impl Default for CreateSpreadsheetRequest {
    fn default() -> Self {
        Self {
            title: "未命名电子表格".to_string(),
            sheets: None,
            time_zone: None,
            locale: None,
            folder_path: None,
            properties: None,
        }
    }
}

/// 更新电子表格属性请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSpreadsheetRequest {
    /// 电子表格标题
    pub title: Option<String>,
    /// 时区设置
    pub time_zone: Option<TimeZone>,
    /// 语言设置
    pub locale: Option<Locale>,
    /// 自定义属性
    pub properties: Option<HashMap<String, Value>>,
}

// ============================================================================
// 筛选相关模型
// ============================================================================

/// 筛选条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterCondition {
    /// 列标识符 (A, B, C...)
    pub column_id: String,
    /// 筛选操作符
    pub operator: String,
    /// 筛选值
    pub value: Option<serde_json::Value>,
    /// 是否忽略大小写
    pub ignore_case: Option<bool>,
}

/// 创建筛选请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFilterRequest {
    /// 筛选范围
    pub range: Range,
    /// 筛选条件列表
    pub conditions: Option<Vec<FilterCondition>>,
}

/// 更新筛选请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFilterRequest {
    /// 筛选条件列表
    pub conditions: Option<Vec<FilterCondition>>,
}

/// 筛选信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterInfo {
    /// 筛选ID
    pub filter_id: String,
    /// 筛选范围
    pub range: Range,
    /// 筛选条件列表
    pub conditions: Option<Vec<FilterCondition>>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
}

// ============================================================================
// 筛选视图相关模型
// ============================================================================

/// 筛选视图基础信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterViewInfo {
    /// 筛选视图ID
    pub filter_view_id: String,
    /// 筛选视图名称
    pub name: String,
    /// 筛选范围
    pub range: Range,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
}

/// 创建筛选视图请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFilterViewRequest {
    /// 筛选视图ID (可选，不填自动生成)
    pub filter_view_id: Option<String>,
    /// 筛选视图名称 (可选，不填自动生成)
    pub name: Option<String>,
    /// 筛选范围 (必填)
    pub range: Range,
}

/// 更新筛选视图请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFilterViewRequest {
    /// 筛选视图名称
    pub name: Option<String>,
    /// 筛选范围
    pub range: Option<Range>,
}

/// 筛选视图条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterViewCondition {
    /// 筛选视图ID
    pub filter_view_id: String,
    /// 列标识符
    pub column_id: String,
    /// 筛选操作符
    pub operator: String,
    /// 筛选值
    pub value: Option<serde_json::Value>,
    /// 是否忽略大小写
    pub ignore_case: Option<bool>,
}

// ============================================================================
// 浮动图片相关模型
// ============================================================================

/// 创建浮动图片请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFloatImageRequest {
    /// 浮动图片令牌 (必填)
    pub float_image_token: FloatImageToken,
    /// 图片范围 (必填，只支持单个单元格)
    pub range: Range,
    /// 浮动图片ID (可选，不填自动生成)
    pub float_image_id: Option<String>,
    /// 图片展示宽度 (可选)
    pub width: Option<i32>,
    /// 图片展示高度 (可选)
    pub height: Option<i32>,
    /// 水平偏移量 (可选，默认为0)
    pub offset_x: Option<i32>,
    /// 垂直偏移量 (可选，默认为0)
    pub offset_y: Option<i32>,
}

/// 更新浮动图片请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFloatImageRequest {
    /// 图片范围
    pub range: Option<Range>,
    /// 图片展示宽度
    pub width: Option<i32>,
    /// 图片展示高度
    pub height: Option<i32>,
    /// 水平偏移量
    pub offset_x: Option<i32>,
    /// 垂直偏移量
    pub offset_y: Option<i32>,
}

/// 浮动图片信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloatImageInfo {
    /// 浮动图片ID
    pub float_image_id: String,
    /// 浮动图片令牌
    pub float_image_token: FloatImageToken,
    /// 图片范围
    pub range: Range,
    /// 图片展示宽度
    pub width: i32,
    /// 图片展示高度
    pub height: i32,
    /// 水平偏移量
    pub offset_x: i32,
    /// 垂直偏移量
    pub offset_y: i32,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
}

// ============================================================================
// 数据操作相关模型
// ============================================================================

/// 查找单元格请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindCellsRequest {
    /// 查找范围
    pub range: Range,
    /// 查找内容
    pub find: String,
    /// 是否匹配大小写
    pub match_case: Option<bool>,
    /// 是否匹配整个单元格
    pub match_entire_cell: Option<bool>,
    /// 是否使用正则表达式
    pub use_regular_expressions: Option<bool>,
}

/// 查找结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindResult {
    /// 匹配的单元格位置
    pub cell_position: Vec<CellPosition>,
    /// 匹配数量
    pub total_count: i32,
}

/// 单元格位置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellPosition {
    /// 工作表ID
    pub sheet_id: String,
    /// 行号
    pub row_index: i32,
    /// 列号
    pub column_index: i32,
    /// 单元格值
    pub value: Option<serde_json::Value>,
}

/// 替换单元格请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplaceCellsRequest {
    /// 替换范围
    pub range: Range,
    /// 查找内容
    pub find: String,
    /// 替换内容
    pub replacement: String,
    /// 是否匹配大小写
    pub match_case: Option<bool>,
    /// 是否匹配整个单元格
    pub match_entire_cell: Option<bool>,
    /// 是否使用正则表达式
    pub use_regular_expressions: Option<bool>,
}

/// 替换结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplaceResult {
    /// 成功替换的单元格位置
    pub replaced_cells: Vec<CellPosition>,
    /// 替换数量
    pub replaced_count: i32,
}

/// 移动行列请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveDimensionRequest {
    /// 移动源范围
    pub source_range: Range,
    /// 移动目标位置
    pub destination_index: i32,
    /// 移动维度 ("ROWS" 或 "COLUMNS")
    pub dimension: String,
}

// ============================================================================
// 响应模型
// ============================================================================

/// API响应基础结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetsResponse<T> {
    /// 响应数据
    pub data: Option<T>,
    /// 错误信息
    pub error: Option<serde_json::Value>,
    /// 请求ID
    pub request_id: Option<String>,
}

/// 分页响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagedResponse<T> {
    /// 数据列表
    pub items: Option<Vec<T>>,
    /// 是否还有更多数据
    pub has_more: Option<bool>,
    /// 页面令牌
    pub page_token: Option<String>,
    /// 总数
    pub total: Option<i32>,
}
