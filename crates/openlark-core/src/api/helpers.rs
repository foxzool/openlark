//! 通用 HTTP 管道 helper（canonical copy）。
//!
//! 业务 crate 的 `common::api_utils` 应 re-export 此处定义，避免各 crate 各派生一份
//! 导致 locality 失守（#330）。错误统一经 `map_context` 附加 `operation` / `resource`
//! （caller 传入的 context）/ `request_id`（响应携带时）结构化诊断上下文。

use crate::SDKResult;
use crate::api::Response;
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

/// 标准化响应数据提取。
///
/// 响应 `data` 为空时返回 `validation_error`，附加 `operation=extract_response_data` +
/// `resource=<context>` + 响应携带的 `request_id`。
pub fn extract_response_data<T>(response: Response<T>, context: &str) -> SDKResult<T> {
    let request_id = response.raw_response.request_id.clone();
    response.data.ok_or_else(|| {
        validation_error("response.data", "服务器没有返回有效的数据").map_context(|ctx| {
            ctx.set_operation("extract_response_data")
                .add_context("resource", context);
            if let Some(req_id) = request_id.as_ref().filter(|r| !r.trim().is_empty()) {
                ctx.set_request_id(req_id);
            }
        })
    })
}

/// 无 data 接口：仅用 `code==0` 判断成功。
///
/// 用于响应不含 `data` 字段的 API。失败时附加 `operation=ensure_success` + `resource=<context>`。
pub fn ensure_success(response: Response<serde_json::Value>, context: &str) -> SDKResult<()> {
    if response.raw_response.is_success() {
        Ok(())
    } else {
        Err(
            validation_error("response.code", response.raw_response.msg.to_string()).map_context(
                |ctx| {
                    ctx.set_operation("ensure_success")
                        .add_context("resource", context);
                },
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::RawResponse;

    #[test]
    fn serialize_params_success() {
        let params = vec!["a", "b"];
        let v = serialize_params(&params, "测试").expect("序列化应成功");
        assert_eq!(v, serde_json::json!(["a", "b"]));
    }

    #[test]
    fn extract_response_data_some() {
        let response: Response<String> = Response {
            data: Some("x".to_string()),
            raw_response: RawResponse::success(),
        };
        assert_eq!(extract_response_data(response, "测试").unwrap(), "x");
    }

    #[test]
    fn extract_response_data_empty_is_error() {
        let response: Response<String> = Response {
            data: None,
            raw_response: RawResponse::success(),
        };
        assert!(extract_response_data(response, "测试").is_err());
    }
}
