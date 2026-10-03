//! CardKit 卡片实体响应模型

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 转换卡片 ID 响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConvertCardIdResponse {
    /// 转换后的卡片 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
}

impl ApiResponseTrait for ConvertCardIdResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
