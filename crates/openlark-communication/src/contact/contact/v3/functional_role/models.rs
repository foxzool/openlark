//! 角色相关模型（不算 API）

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 创建角色响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateFunctionalRoleResponse {
    /// 新创建的角色 ID。
    pub role_id: String,
}

impl ApiResponseTrait for CreateFunctionalRoleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
