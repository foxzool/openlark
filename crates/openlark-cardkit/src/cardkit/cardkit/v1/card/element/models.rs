//! CardKit 卡片组件响应模型

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 新增组件响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateCardElementResponse {
    /// 卡片 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    /// 组件 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
}

impl ApiResponseTrait for CreateCardElementResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新组件响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCardElementResponse {
    /// 卡片 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    /// 组件 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
}

impl ApiResponseTrait for UpdateCardElementResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 补丁组件响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchCardElementResponse {
    /// 卡片 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    /// 组件 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
}

impl ApiResponseTrait for PatchCardElementResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 流式更新文本响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCardElementContentResponse {
    /// 卡片 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    /// 组件 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
}

impl ApiResponseTrait for UpdateCardElementContentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除组件响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteCardElementResponse {
    /// 卡片 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    /// 组件 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
}

impl ApiResponseTrait for DeleteCardElementResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
