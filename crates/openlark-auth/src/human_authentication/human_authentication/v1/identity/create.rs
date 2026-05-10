//! 录入身份信息
//!
//! 文档: https://open.feishu.cn/document/server-docs/human_authentication-v1/create
//! docPath: https://open.feishu.cn/document/server-docs/human_authentication-v1/create

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Deserializer, Serialize};

use crate::common::HumanAuthenticationApiV1;

/// 实名认证用户 ID 类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HumanAuthenticationUserIdType {
    /// 应用内用户标识。
    #[serde(rename = "open_id")]
    OpenId,
    /// 开发商维度用户标识。
    #[serde(rename = "union_id")]
    UnionId,
    /// 租户内用户标识。
    #[serde(rename = "user_id")]
    UserId,
}

impl HumanAuthenticationUserIdType {
    /// 获取查询参数值。
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OpenId => "open_id",
            Self::UnionId => "union_id",
            Self::UserId => "user_id",
        }
    }
}

/// 录入身份信息请求体。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
struct IdentityCreateRequestBody {
    identity_name: String,
    identity_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    mobile: Option<String>,
}

/// 录入身份信息响应。
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct IdentityCreateResponse {
    /// 用户绑定实名身份后的 verify uid。
    pub verify_uid: String,
    /// 兼容旧字段名，值与 `verify_uid` 保持一致。
    #[serde(skip_serializing)]
    pub identity_id: String,
}

impl ApiResponseTrait for IdentityCreateResponse {}

impl<'de> Deserialize<'de> for IdentityCreateResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct IdentityCreateResponsePayload {
            verify_uid: String,
            #[serde(default)]
            identity_id: Option<String>,
        }

        let payload = IdentityCreateResponsePayload::deserialize(deserializer)?;
        let identity_id = payload
            .identity_id
            .unwrap_or_else(|| payload.verify_uid.clone());

        Ok(Self {
            verify_uid: payload.verify_uid,
            identity_id,
        })
    }
}

/// 录入身份信息 Builder。
#[derive(Debug, Clone)]
pub struct IdentityCreateBuilder {
    config: Config,
    user_id: String,
    user_id_type: Option<HumanAuthenticationUserIdType>,
    identity_name: String,
    identity_code: String,
    mobile: Option<String>,
}

impl IdentityCreateBuilder {
    /// 创建新的 Builder。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id: String::new(),
            user_id_type: None,
            identity_name: String::new(),
            identity_code: String::new(),
            mobile: None,
        }
    }

    /// 设置目标用户 ID。
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = user_id.into();
        self
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: HumanAuthenticationUserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 设置姓名。
    pub fn identity_name(mut self, identity_name: impl Into<String>) -> Self {
        self.identity_name = identity_name.into();
        self
    }

    /// 设置身份证号。
    pub fn identity_code(mut self, identity_code: impl Into<String>) -> Self {
        self.identity_code = identity_code.into();
        self
    }

    /// 设置手机号。
    pub fn mobile(mut self, mobile: impl Into<String>) -> Self {
        self.mobile = Some(mobile.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<IdentityCreateResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用自定义请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<IdentityCreateResponse> {
        validate_required!(self.identity_name, "identity_name 不能为空");
        validate_required!(self.identity_code, "identity_code 不能为空");

        let request_body = IdentityCreateRequestBody {
            identity_name: self.identity_name,
            identity_code: self.identity_code,
            mobile: self.mobile,
        };

        let mut req: ApiRequest<IdentityCreateResponse> =
            ApiRequest::post(HumanAuthenticationApiV1::IdentityCreate.path())
                .body(serde_json::to_value(&request_body)?);

        if !self.user_id.is_empty() {
            req = req.query("user_id", self.user_id);
        }

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("录入身份信息", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_create_builder_defaults() {
        let builder = IdentityCreateBuilder::new(Config::default());
        assert!(builder.user_id.is_empty());
        assert_eq!(builder.user_id_type, None);
        assert!(builder.identity_name.is_empty());
        assert!(builder.identity_code.is_empty());
        assert_eq!(builder.mobile, None);
    }

    #[test]
    fn test_identity_create_builder_chain() {
        let builder = IdentityCreateBuilder::new(Config::default())
            .user_id("ou_xxx")
            .user_id_type(HumanAuthenticationUserIdType::UserId)
            .identity_name("张三")
            .identity_code("4xxxxxxxx")
            .mobile("13xxxxxxx");
        assert_eq!(builder.user_id, "ou_xxx");
        assert_eq!(
            builder.user_id_type,
            Some(HumanAuthenticationUserIdType::UserId)
        );
        assert_eq!(builder.identity_name, "张三");
        assert_eq!(builder.identity_code, "4xxxxxxxx");
        assert_eq!(builder.mobile.as_deref(), Some("13xxxxxxx"));
    }

    #[test]
    fn test_identity_create_response_deserialization() {
        let json = r#"{"verify_uid":"ou_2eb5483cb377daa5054bc6f86e2089a5"}"#;
        let response: IdentityCreateResponse =
            serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(response.verify_uid, "ou_2eb5483cb377daa5054bc6f86e2089a5");
        assert_eq!(response.identity_id, "ou_2eb5483cb377daa5054bc6f86e2089a5");
    }

    #[test]
    fn test_identity_create_response_prefers_explicit_identity_id_when_present() {
        let json = r#"{
            "verify_uid":"ou_verify",
            "identity_id":"legacy_identity"
        }"#;
        let response: IdentityCreateResponse =
            serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(response.verify_uid, "ou_verify");
        assert_eq!(response.identity_id, "legacy_identity");
    }

    #[test]
    fn test_identity_create_legacy_builder_chain_without_user_id() {
        let builder = IdentityCreateBuilder::new(Config::default())
            .identity_name("张三")
            .identity_code("4xxxxxxxx")
            .mobile("13xxxxxxx");
        assert!(builder.user_id.is_empty());
        assert_eq!(builder.identity_name, "张三");
        assert_eq!(builder.identity_code, "4xxxxxxxx");
        assert_eq!(builder.mobile.as_deref(), Some("13xxxxxxx"));
    }
}
