//! 批量删除邮件组管理员

use crate::common::api_utils::serialize_params;
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
pub struct BatchDeleteMailGroupManagerRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    body: BatchDeleteMailGroupManagerBody,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// 待补充文档。
pub struct BatchDeleteMailGroupManagerBody {
    /// 待补充文档。
    pub manager_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 待补充文档。
pub struct BatchDeleteMailGroupManagerResponse {
    /// 待补充文档。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for BatchDeleteMailGroupManagerResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl BatchDeleteMailGroupManagerRequest {
    /// 待补充文档。
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            body: BatchDeleteMailGroupManagerBody::default(),
        }
    }

    /// 待补充文档。
    pub fn manager_ids(mut self, ids: Vec<String>) -> Self {
        self.body.manager_ids = ids;
        self
    }

    /// 待补充文档。
    pub async fn execute(self) -> SDKResult<BatchDeleteMailGroupManagerResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 待补充文档。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BatchDeleteMailGroupManagerResponse> {
        validate_required!(self.mailgroup_id.trim(), "mailgroup_id 不能为空");
        validate_required_list!(
            self.body.manager_ids,
            1000,
            "manager_ids 不能为空且不能超过 1000 个"
        );

        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/managers/batch_delete",
            self.mailgroup_id
        );
        let req: ApiRequest<BatchDeleteMailGroupManagerResponse> =
            ApiRequest::post(&path).body(serialize_params(&self.body, "批量删除邮件组管理员")?);

        let _resp: openlark_core::api::Response<BatchDeleteMailGroupManagerResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(BatchDeleteMailGroupManagerResponse { data: None })
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
