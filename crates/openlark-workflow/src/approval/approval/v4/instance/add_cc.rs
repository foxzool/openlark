//! 抄送审批实例（用户级，v4）
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/add_cc>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_utils::{missing_response_data_error, request_serialization_error};

/// 抄送审批实例请求体（用户级，v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct AddCcInstanceBodyV4 {
    /// 审批实例 Code
    pub instance_code: String,
    /// 被抄送人用户 ID 列表（最多 20 个）
    pub cc_user_ids: Vec<String>,
    /// 抄送留言（不超过 500 字）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// 抄送审批实例响应（用户级，v4）
#[derive(Debug, Clone, Deserialize, Default)]
pub struct AddCcInstanceResponseV4 {}

/// 抄送审批实例请求（用户级，v4）
#[derive(Debug, Clone)]
pub struct AddCcInstanceRequestV4 {
    config: Arc<Config>,
    body: AddCcInstanceBodyV4,
    user_id_type: Option<String>,
}

impl AddCcInstanceRequestV4 {
    /// 创建请求实例
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: AddCcInstanceBodyV4::default(),
            user_id_type: None,
        }
    }

    /// 设置审批实例 Code
    pub fn instance_code(mut self, instance_code: impl Into<String>) -> Self {
        self.body.instance_code = instance_code.into();
        self
    }

    /// 添加被抄送人用户 ID
    pub fn add_cc_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.body.cc_user_ids.push(user_id.into());
        self
    }

    /// 设置被抄送人用户 ID 列表
    pub fn cc_user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.body.cc_user_ids = user_ids;
        self
    }

    /// 设置抄送留言
    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.body.comment = Some(comment.into());
        self
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<AddCcInstanceResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<AddCcInstanceResponseV4> {
        validate_required!(self.body.instance_code.trim(), "审批实例 Code 不能为空");
        validate_required_list!(
            self.body.cc_user_ids,
            20,
            "被抄送人用户 ID 列表不能为空且不能超过 20 个"
        );

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceAddCc;
        let mut request = ApiRequest::<AddCcInstanceResponseV4>::post(api_endpoint.to_url());

        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        let body_json = serde_json::to_value(&self.body)
            .map_err(|e| request_serialization_error("抄送审批实例（用户级）", e))?;

        request = request.body(body_json);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            missing_response_data_error(
                "抄送审批实例（用户级）",
                response.raw_response.request_id.clone(),
            )
        })
    }
}

impl ApiResponseTrait for AddCcInstanceResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_instance_add_cc_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceAddCc;
        assert_eq!(endpoint.to_url(), "/open-apis/approval/v4/instances/add_cc");
    }

    #[test]
    fn test_add_cc_instance_request_builder() {
        let config = Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );
        let request = AddCcInstanceRequestV4::new(config)
            .instance_code("instance_code")
            .add_cc_user_id("ou_aaa")
            .add_cc_user_id("ou_bbb")
            .comment("请查阅")
            .user_id_type("open_id");

        assert_eq!(request.body.instance_code, "instance_code");
        assert_eq!(request.body.cc_user_ids, vec!["ou_aaa", "ou_bbb"]);
        assert_eq!(request.body.comment.as_deref(), Some("请查阅"));
        assert_eq!(request.user_id_type.as_deref(), Some("open_id"));
    }

    #[test]
    fn test_add_cc_instance_body_contract() {
        let body = AddCcInstanceBodyV4 {
            instance_code: "instance_code".to_string(),
            cc_user_ids: vec!["ou_aaa".to_string()],
            comment: Some("请查阅".to_string()),
        };
        let value = serde_json::to_value(&body).expect("add_cc body should serialize");
        assert_eq!(
            value,
            json!({
                "instance_code": "instance_code",
                "cc_user_ids": ["ou_aaa"],
                "comment": "请查阅"
            })
        );
    }
}
