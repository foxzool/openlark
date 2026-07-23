/// API通用工具函数（re-export core canonical + 本域 error 构造器）。
///
/// `serialize_params` 已下沉到
/// `openlark_core::api`（#330），本模块 re-export canonical copy；保留 workflow 域专用的
/// 标准 error 构造器（`missing_response_data_error` / `request_serialization_error`），
/// 多处审批任务叶子直接复用其错误形状。
use openlark_core::error;

// canonical HTTP 管道 helper（#330 下沉到 core）
pub use openlark_core::api::serialize_params;

const ERROR_COMPONENT: &str = "openlark-workflow";

fn attach_standard_error_context(
    err: openlark_core::error::CoreError,
    operation: &str,
    resource: &str,
    request_id: Option<String>,
) -> openlark_core::error::CoreError {
    err.with_operation(operation, ERROR_COMPONENT)
        .map_context(|ctx| {
            ctx.add_context("resource", resource);
            if let Some(request_id) = request_id.filter(|value| !value.trim().is_empty()) {
                ctx.set_request_id(request_id);
            }
        })
}

/// 创建“响应 data 为空”的标准错误。
pub fn missing_response_data_error(
    resource: &str,
    request_id: Option<String>,
) -> openlark_core::error::CoreError {
    attach_standard_error_context(
        error::validation_error("response.data", "服务器没有返回有效的数据"),
        "extract_response_data",
        resource,
        request_id,
    )
}

/// 创建“请求参数序列化失败”的标准错误。
pub fn request_serialization_error(
    resource: &str,
    source: impl std::fmt::Display,
) -> openlark_core::error::CoreError {
    attach_standard_error_context(
        error::validation_error("request.params", format!("无法序列化请求参数: {source}")),
        "serialize_params",
        resource,
        None,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::error::ErrorTrait;

    #[test]
    fn missing_response_data_error_attaches_context() {
        let err = missing_response_data_error("审批任务查询", Some("req-wf-456".to_string()));
        assert_eq!(err.context().operation(), Some("extract_response_data"));
        assert_eq!(err.context().component(), Some(ERROR_COMPONENT));
        assert_eq!(err.context().get_context("resource"), Some("审批任务查询"));
        assert_eq!(err.context().request_id(), Some("req-wf-456"));
    }

    #[test]
    fn request_serialization_error_attaches_context() {
        let err = request_serialization_error("退回审批任务", "boom");
        assert_eq!(err.context().operation(), Some("serialize_params"));
        assert_eq!(err.context().get_context("resource"), Some("退回审批任务"));
    }
}
