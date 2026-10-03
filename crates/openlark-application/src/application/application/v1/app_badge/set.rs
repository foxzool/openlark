//! 更新应用红点（历史 v1 模块路径；HTTP 端点为 application/v6）
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/app_badge/set>
//!
//! 与 `application::v6::app_badge::set` 字段对齐；请优先使用 v6 路径。

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 客户端红点数量（`client_badge_num`）。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClientBadgeNum {
    /// h5 / web_app 能力的 badge 数量。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<i32>,
    /// 小程序能力的 badge 数量。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gadget: Option<i32>,
}

/// 设置应用徽标的请求。
#[derive(Debug, Clone)]
pub struct SetAppBadgeRequest {
    config: Arc<Config>,
    /// 查询参数 `user_id_type`。
    user_id_type: Option<String>,
    body: SetAppBadgeBody,
}

/// 设置应用徽标的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetAppBadgeBody {
    /// 用户 ID（类型由 `user_id_type` 决定）。
    pub user_id: String,
    /// 红点数据版本号。
    pub version: String,
    /// 红点额外信息（JSON 字符串）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
    /// PC 端红点数量。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pc: Option<ClientBadgeNum>,
    /// 移动端红点数量。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<ClientBadgeNum>,
}

impl SetAppBadgeBody {
    fn validate(&self) -> SDKResult<()> {
        validate_required!(self.user_id.trim(), "user_id 不能为空");
        validate_required!(self.version.trim(), "version 不能为空");
        Ok(())
    }
}

/// 设置应用徽标的响应 `data`（文档为空对象）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetAppBadgeResponse {}

impl ApiResponseTrait for SetAppBadgeResponse {
    fn empty_success() -> Option<Self> {
        Some(Self::default())
    }
}

impl SetAppBadgeRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            user_id_type: None,
            body: SetAppBadgeBody::default(),
        }
    }

    /// 设置用户 ID 类型（查询参数）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置用户 ID。
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.body.user_id = user_id.into();
        self
    }

    /// 设置红点数据版本号。
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.body.version = version.into();
        self
    }

    /// 设置红点额外信息。
    pub fn extra(mut self, extra: impl Into<String>) -> Self {
        self.body.extra = Some(extra.into());
        self
    }

    /// 设置 PC 端红点数量。
    pub fn pc(mut self, pc: ClientBadgeNum) -> Self {
        self.body.pc = Some(pc);
        self
    }

    /// 设置移动端红点数量。
    pub fn mobile(mut self, mobile: ClientBadgeNum) -> Self {
        self.body.mobile = Some(mobile);
        self
    }

    /// 执行设置应用徽标请求。
    pub async fn execute(self) -> SDKResult<SetAppBadgeResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SetAppBadgeResponse> {
        self.body.validate()?;
        let body = serde_json::to_value(&self.body)?;
        let req: ApiRequest<SetAppBadgeResponse> =
            ApiRequest::post("/open-apis/application/v6/app_badge/set")
                .query_opt("user_id_type", self.user_id_type.as_ref())
                .body(body)
                .with_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request_typed(req, &self.config, Some(option), "更新应用红点").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_body_serialization_matches_official_fields() {
        let body = SetAppBadgeBody {
            user_id: "ou_1".into(),
            version: "1664360599355".into(),
            extra: Some("{}".into()),
            pc: Some(ClientBadgeNum {
                web_app: Some(1),
                gadget: Some(2),
            }),
            mobile: None,
        };
        let v = serde_json::to_value(&body).unwrap();
        assert_eq!(v["user_id"], "ou_1");
        assert_eq!(v["version"], "1664360599355");
        assert!(v.get("app_id").is_none());
        assert!(v.get("badge").is_none());
    }
}
