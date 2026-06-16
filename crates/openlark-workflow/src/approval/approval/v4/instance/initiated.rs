//! 查询用户的已发起审批列表（用户级，v4）
//!
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/initiated

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::common::api_utils::missing_response_data_error;

/// 审批摘要项（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct InstanceSummaryV4 {
    /// 摘要键
    pub key: String,
    /// 摘要值
    pub value: String,
}

/// 已发起审批实例列表项（用户级，v4）
#[derive(Debug, Clone, Deserialize)]
pub struct InitiatedInstanceItemV4 {
    /// 实例状态
    pub instance_status: String,
    /// 审批定义 Code
    pub definition_code: String,
    /// 发起人用户 ID
    pub initiator: String,
    /// 发起人姓名
    pub initiator_name: String,
    /// 审批实例 Code
    pub instance_code: String,
    /// 审批定义分组 ID
    #[serde(default)]
    pub definition_group_id: Option<String>,
    /// 审批定义分组名称
    #[serde(default)]
    pub definition_group_name: Option<String>,
    /// 审批定义名称
    pub definition_name: String,
    /// 审批摘要列表
    #[serde(default)]
    pub summaries: Vec<InstanceSummaryV4>,
}

/// 查询用户的已发起审批列表响应（用户级，v4）
#[derive(Debug, Clone, Deserialize)]
pub struct InitiatedInstanceResponseV4 {
    /// 实例列表
    #[serde(default)]
    pub instances: Vec<InitiatedInstanceItemV4>,
    /// 是否有更多数据
    #[serde(default)]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(default)]
    pub page_token: Option<String>,
}

/// 查询用户的已发起审批列表请求（用户级，v4）
#[derive(Debug, Clone)]
pub struct InitiatedInstanceRequestV4 {
    config: Arc<Config>,
    definition_code: Option<String>,
    user_id_type: Option<String>,
    locale: Option<String>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl InitiatedInstanceRequestV4 {
    /// 创建请求实例
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            definition_code: None,
            user_id_type: None,
            locale: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置审批定义 Code（按审批类型过滤）
    pub fn definition_code(mut self, definition_code: impl Into<String>) -> Self {
        self.definition_code = Some(definition_code.into());
        self
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置语言
    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<InitiatedInstanceResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<InitiatedInstanceResponseV4> {
        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceInitiated;
        let mut request = ApiRequest::<InitiatedInstanceResponseV4>::get(api_endpoint.to_url());

        if let Some(definition_code) = self.definition_code {
            request = request.query("definition_code", definition_code);
        }
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }
        if let Some(locale) = self.locale {
            request = request.query("locale", locale);
        }
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            missing_response_data_error(
                "查询已发起审批列表（用户级）",
                response.raw_response.request_id.clone(),
            )
        })
    }
}

impl ApiResponseTrait for InitiatedInstanceResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_instance_initiated_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceInitiated;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/approval/v4/instances/initiated"
        );
    }

    #[test]
    fn test_initiated_instance_request_builder() {
        let config = Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );
        let request = InitiatedInstanceRequestV4::new(config)
            .definition_code("definition_code")
            .user_id_type("open_id")
            .locale("zh-CN")
            .page_size(50)
            .page_token("token_1");

        assert_eq!(request.definition_code.as_deref(), Some("definition_code"));
        assert_eq!(request.user_id_type.as_deref(), Some("open_id"));
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token.as_deref(), Some("token_1"));
    }

    #[test]
    fn test_initiated_instance_response_deserialize() {
        let json = serde_json::json!({
            "instances": [{
                "instance_status": "Running",
                "definition_code": "ABC",
                "initiator": "ou_xxx",
                "initiator_name": "张三",
                "instance_code": "INS1",
                "definition_group_id": "g1",
                "definition_group_name": "考勤",
                "definition_name": "加班",
                "summaries": [{"key": "k", "value": "v"}]
            }],
            "has_more": false,
            "page_token": ""
        });
        let resp: InitiatedInstanceResponseV4 =
            serde_json::from_value(json).expect("initiated response should deserialize");
        assert_eq!(resp.instances.len(), 1);
        assert_eq!(resp.instances[0].definition_name, "加班");
        assert_eq!(resp.instances[0].summaries.len(), 1);
        assert_eq!(resp.has_more, Some(false));
    }
}
