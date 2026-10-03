//! CardKit 卡片组件响应模型

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 新增组件响应（官方 `data` 为空对象）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateCardElementResponse {}

impl ApiResponseTrait for CreateCardElementResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新组件响应（官方 `data` 为空对象）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCardElementResponse {}

impl ApiResponseTrait for UpdateCardElementResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 补丁组件响应（官方 `data` 为空对象）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchCardElementResponse {}

impl ApiResponseTrait for PatchCardElementResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 流式更新文本响应（官方 `data` 为空对象）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCardElementContentResponse {}

impl ApiResponseTrait for UpdateCardElementContentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除组件响应（官方 `data` 为空对象）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteCardElementResponse {}

impl ApiResponseTrait for DeleteCardElementResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
