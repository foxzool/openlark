//! 通用 HTTP 管道 helper（canonical copy）。
//!
//! 业务 crate 的 `common::api_utils` 应 re-export 此处定义，避免各 crate 各派生一份
//! 导致 locality 失守（#330）。错误统一经 `map_context` 附加 `operation` / `resource`
//! （caller 传入的 context）/ `request_id`（响应携带时）结构化诊断上下文。

use crate::SDKResult;
use crate::error::validation_error;

/// 标准化参数序列化。
///
/// 序列化失败时返回 `validation_error`，附加 `operation=serialize_params` + `resource=<context>`。
pub fn serialize_params<T: serde::Serialize>(
    params: &T,
    context: &str,
) -> SDKResult<serde_json::Value> {
    serde_json::to_value(params).map_err(|e| {
        validation_error("request.params", format!("无法序列化请求参数: {e}")).map_context(|ctx| {
            ctx.set_operation("serialize_params")
                .add_context("resource", context);
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_params_success() {
        let params = vec!["a", "b"];
        let v = serialize_params(&params, "测试").expect("序列化应成功");
        assert_eq!(v, serde_json::json!(["a", "b"]));
    }
}
