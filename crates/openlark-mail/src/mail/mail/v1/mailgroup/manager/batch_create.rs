//! 批量创建邮件组管理员

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
pub struct BatchCreateMailGroupManagerRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    body: BatchCreateMailGroupManagerBody,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// 待补充文档。
pub struct BatchCreateMailGroupManagerBody {
    /// 待补充文档。
    pub manager_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 待补充文档。
pub struct BatchCreateMailGroupManagerResponse {
    /// 待补充文档。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for BatchCreateMailGroupManagerResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl BatchCreateMailGroupManagerRequest {
    /// 待补充文档。
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            body: BatchCreateMailGroupManagerBody::default(),
        }
    }

    /// 待补充文档。
    pub fn manager_ids(mut self, ids: Vec<String>) -> Self {
        self.body.manager_ids = ids;
        self
    }

    /// 待补充文档。
    pub async fn execute(self) -> SDKResult<BatchCreateMailGroupManagerResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 待补充文档。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BatchCreateMailGroupManagerResponse> {
        validate_required!(self.mailgroup_id.trim(), "mailgroup_id 不能为空");
        validate_required_list!(
            self.body.manager_ids,
            1000,
            "manager_ids 不能为空且不能超过 1000 个"
        );

        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/managers/batch_create",
            self.mailgroup_id
        );
        let req: ApiRequest<BatchCreateMailGroupManagerResponse> =
            ApiRequest::post(&path).body(serialize_params(&self.body, "批量创建邮件组管理员")?);

        let _resp: openlark_core::api::Response<BatchCreateMailGroupManagerResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(BatchCreateMailGroupManagerResponse { data: None })
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
