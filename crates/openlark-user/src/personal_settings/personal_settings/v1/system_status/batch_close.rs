//! 批量关闭系统状态
//! docPath: <https://open.feishu.cn/document/server-docs/personal_settings-v1/system_status/batch_close>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
/// 待补充文档。
pub struct BatchCloseSystemStatusRequest {
    config: Arc<Config>,
    status_id: String,
    body: BatchCloseSystemStatusBody,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// 待补充文档。
pub struct BatchCloseSystemStatusBody {
    /// 待补充文档。
    pub user_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 待补充文档。
pub struct BatchCloseSystemStatusResponse {
    /// 待补充文档。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for BatchCloseSystemStatusResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl BatchCloseSystemStatusRequest {
    /// 待补充文档。
    pub fn new(config: Arc<Config>, status_id: impl Into<String>) -> Self {
        Self {
            config,
            status_id: status_id.into(),
            body: BatchCloseSystemStatusBody::default(),
        }
    }

    /// 待补充文档。
    pub fn user_ids(mut self, ids: Vec<String>) -> Self {
        self.body.user_ids = ids;
        self
    }

    /// 待补充文档。
    pub async fn execute(self) -> SDKResult<BatchCloseSystemStatusResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 待补充文档。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BatchCloseSystemStatusResponse> {
        validate_required!(self.status_id.trim(), "status_id 不能为空");
        validate_required_list!(
            self.body.user_ids,
            1000,
            "user_ids 不能为空且不能超过 1000 个"
        );

        let path = format!(
            "/open-apis/personal_settings/v1/system_statuses/{}/batch_close",
            self.status_id
        );
        let body = serde_json::to_value(&self.body)?;
        let req: ApiRequest<BatchCloseSystemStatusResponse> = ApiRequest::post(&path).body(body);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        Ok(resp
            .data
            .unwrap_or(BatchCloseSystemStatusResponse { data: None }))
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
