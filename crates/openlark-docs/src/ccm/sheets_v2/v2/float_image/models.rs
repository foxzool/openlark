/// CCM Sheet V2 浮图模型
use serde::{Deserialize, Serialize};

/// 创建浮图请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFloatImageParams {
    /// 图片token
    #[serde(rename = "image_token")]
    pub image_token: String,
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
    /// 浮图位置
    #[serde(rename = "float_image")]
    pub float_image: FloatImagePosition,
}

/// 浮图位置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloatImagePosition {
    /// 左边距
    #[serde(rename = "offset_x")]
    pub offset_x: i32,
    /// 上边距
    #[serde(rename = "offset_y")]
    pub offset_y: i32,
    /// 宽度
    #[serde(rename = "width")]
    pub width: i32,
    /// 高度
    #[serde(rename = "height")]
    pub height: i32,
    /// 缩放比例
    #[serde(rename = "scale")]
    pub scale: f64,
    /// Z轴层级
    #[serde(rename = "z_index")]
    pub z_index: i32,
}

/// 创建浮图响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFloatImageResponse {
    /// 浮图创建结果
    pub data: Option<FloatImageResult>,
}

/// 浮图操作结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloatImageResult {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 浮图ID
    #[serde(rename = "float_image_id")]
    pub float_image_id: String,
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
}

/// 获取浮图请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFloatImageParams {
    /// 浮图ID
    #[serde(rename = "float_image_id")]
    pub float_image_id: String,
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
}

/// 获取浮图响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFloatImageResponse {
    /// 浮图信息
    pub data: Option<FloatImageInfo>,
}

/// 浮图信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloatImageInfo {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 浮图ID
    #[serde(rename = "float_image_id")]
    pub float_image_id: String,
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
    /// 图片token
    #[serde(rename = "image_token")]
    pub image_token: String,
    /// 浮图位置
    #[serde(rename = "float_image")]
    pub float_image: FloatImagePosition,
}

/// 更新浮图请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFloatImageParams {
    /// 浮图ID
    #[serde(rename = "float_image_id")]
    pub float_image_id: String,
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
    /// 浮图位置
    #[serde(rename = "float_image")]
    pub float_image: FloatImagePosition,
}

/// 更新浮图响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFloatImageResponse {
    /// 浮图更新结果
    pub data: Option<FloatImageResult>,
}

/// 删除浮图请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFloatImageParams {
    /// 浮图ID
    #[serde(rename = "float_image_id")]
    pub float_image_id: String,
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
}

/// 删除浮图响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFloatImageResponse {
    /// 浮图删除结果
    pub data: Option<FloatImageResult>,
}
