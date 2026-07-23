//! 安全服务错误处理模块
//!
//! security 业务 crate 复用 core 的统一错误体系：leaf 经
//! `Transport::request_typed → Response::decode → CoreError` 路径产生错误，
//! 构造复用 core 通用构造器（`validation_error` 等），不另立领域错误类型。
//! （历史上的 `SecurityErrorBuilder` / `map_feishu_security_error` 为 0 消费者
//! 测试专属死代码，#500 删除。）

use openlark_core::error::CoreError;

/// 安全服务错误类型 - 完全基于 CoreError
pub type SecurityError = CoreError;

/// 安全服务结果类型
pub type SecurityResult<T> = Result<T, SecurityError>;
