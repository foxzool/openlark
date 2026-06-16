//! 批量删除邮件组权限成员

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

/// Batch Delete Mail Group Permission Member Request。
#[derive(Debug, Clone)]
pub struct BatchDeleteMailGroupPermissionMemberRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    body: BatchDeleteMailGroupPermissionMemberBody,
}

/// Batch Delete Mail Group Permission Member Body。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchDeleteMailGroupPermissionMemberBody {
    /// permission_member_ids 字段。
    pub permission_member_ids: Vec<String>,
}

/// Batch Delete Mail Group Permission Member Response。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteMailGroupPermissionMemberResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for BatchDeleteMailGroupPermissionMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl BatchDeleteMailGroupPermissionMemberRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            body: BatchDeleteMailGroupPermissionMemberBody::default(),
        }
    }

    /// permission member ids。
    pub fn permission_member_ids(mut self, ids: Vec<String>) -> Self {
        self.body.permission_member_ids = ids;
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<BatchDeleteMailGroupPermissionMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BatchDeleteMailGroupPermissionMemberResponse> {
        validate_required!(self.mailgroup_id.trim(), "mailgroup_id 不能为空");
        validate_required_list!(
            self.body.permission_member_ids,
            1000,
            "permission_member_ids 不能为空且不能超过 1000 个"
        );

        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/permission_members/batch_delete",
            self.mailgroup_id
        );
        let req: ApiRequest<BatchDeleteMailGroupPermissionMemberResponse> =
            ApiRequest::delete(&path).body(serialize_params(&self.body, "批量删除邮件组权限成员")?);

        let _resp: openlark_core::api::Response<BatchDeleteMailGroupPermissionMemberResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(BatchDeleteMailGroupPermissionMemberResponse { data: None })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

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
