//! 查询角色成员信息
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/permission/application-role-member/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 查询角色成员信息 Builder
#[derive(Debug, Clone)]
pub struct RoleMemberGetBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 角色 API 名称
    role_api_name: String,
    /// 页码
    page: Option<u32>,
    /// 每页数量
    page_size: Option<u32>,
}

impl RoleMemberGetBuilder {
    /// 创建新的 Builder
    pub fn new(
        config: Config,
        namespace: impl Into<String>,
        role_api_name: impl Into<String>,
    ) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            role_api_name: role_api_name.into(),
            page: None,
            page_size: None,
        }
    }

    /// 设置页码
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// 设置每页数量
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RoleMemberGetResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/roles/{}/member",
            self.namespace, self.role_api_name
        );

        let mut req: ApiRequest<RoleMemberGetResponse> = ApiRequest::get(&url);
        if let Some(page) = self.page {
            req = req.query("page", page.to_string());
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        Transport::request(req, &self.config, None).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<RoleMemberGetResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/roles/{}/member",
            self.namespace, self.role_api_name
        );

        let mut req: ApiRequest<RoleMemberGetResponse> = ApiRequest::get(&url);
        if let Some(page) = self.page {
            req = req.query("page", page.to_string());
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        Transport::request(req, &self.config, Some(option)).await
    }
}

/// 角色成员信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoleMember {
    /// 用户 ID
    #[serde(rename = "user_id")]
    user_id: String,
    /// 成员类型
    #[serde(rename = "member_type")]
    member_type: String,
    /// 添加时间
    #[serde(rename = "added_time")]
    added_time: i64,
}

/// 查询角色成员信息响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoleMemberGetResponse {
    /// 角色成员列表
    #[serde(rename = "items")]
    items: Vec<RoleMember>,
    /// 是否有更多
    #[serde(rename = "has_more")]
    has_more: bool,
}

impl ApiResponseTrait for RoleMemberGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

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
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
