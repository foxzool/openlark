//! 通用工具与辅助函数

pub mod api_utils;
pub mod chain;
pub mod validation;

// 重导出验证函数，方便外部使用
pub use validation::{
    validate_card_id, validate_element_id, validate_id_list, validate_id_type, validate_sequence,
    validate_uuid,
};
