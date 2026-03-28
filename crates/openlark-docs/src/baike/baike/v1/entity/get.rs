//! 获取词条详情
//!
//! docPath: <https://open.feishu.cn/document/server-docs/baike-v1/entity/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::baike::baike::v1::models::{Entity, UserIdType};
use crate::common::api_endpoints::BaikeApiV1;

/// 获取词条详情响应（data）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetEntityResp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<Entity>,
}

impl ApiResponseTrait for GetEntityResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取词条详情请求
pub struct GetEntityRequest {
    config: Config,
    entity_id: String,
    provider: Option<String>,
    outer_id: Option<String>,
    user_id_type: Option<UserIdType>,
}

impl GetEntityRequest {
    pub fn new(config: Config, entity_id: impl Into<String>) -> Self {
        Self {
            config,
            entity_id: entity_id.into(),
            provider: None,
            outer_id: None,
            user_id_type: None,
        }
    }

    /// 外部系统（可选）
    pub fn provider(mut self, provider: impl Into<String>) -> Self {
        self.provider = Some(provider.into());
        self
    }

    /// 外部系统词条唯一 ID（可选）
    pub fn outer_id(mut self, outer_id: impl Into<String>) -> Self {
        self.outer_id = Some(outer_id.into());
        self
    }

    /// 用户 ID 类型（query: user_id_type）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    pub async fn execute(self) -> SDKResult<GetEntityResp> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<GetEntityResp> {
        // ===== 参数校验 =====
        validate_required!(self.entity_id, "entity_id 不能为空");

        let using_outer = self.provider.is_some() || self.outer_id.is_some();
        if using_outer {
            if self.entity_id != "enterprise_0" {
                return Err(openlark_core::error::validation_error(
                    "entity_id",
                    "当通过 provider + outer_id 获取词条时，entity_id 必须固定为 enterprise_0",
                ));
            }
            if self.provider.is_none() || self.outer_id.is_none() {
                return Err(openlark_core::error::validation_error(
                    "provider/outer_id",
                    "provider 与 outer_id 需同时传入",
                ));
            }
        }
        if let Some(provider) = &self.provider {
            let len = provider.chars().count();
            if !(2..=32).contains(&len) {
                return Err(openlark_core::error::validation_error(
                    "provider",
                    "provider 长度必须在 2~32 字符之间",
                ));
            }
        }
        if let Some(outer_id) = &self.outer_id {
            let len = outer_id.chars().count();
            if !(1..=64).contains(&len) {
                return Err(openlark_core::error::validation_error(
                    "outer_id",
                    "outer_id 长度必须在 1~64 字符之间",
                ));
            }
        }

        // ===== 构建请求 =====
        let mut api_request: ApiRequest<GetEntityResp> =
            ApiRequest::get(&BaikeApiV1::EntityGet(self.entity_id).to_url());
        if let Some(provider) = &self.provider {
            api_request = api_request.query("provider", provider);
        }
        if let Some(outer_id) = &self.outer_id {
            api_request = api_request.query("outer_id", outer_id);
        }
        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type.as_str());
        }

        // ===== 发送请求并返回结果 =====
        let response: Response<GetEntityResp> =
            Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::baike::baike::v1::models::UserIdType;

    #[test]
    fn test_get_entity_request_builder() {
        let config = Config::default();
        let request = GetEntityRequest::new(config, "entity_123").user_id_type(UserIdType::OpenId);

        assert_eq!(request.entity_id, "entity_123");
        assert!(request.user_id_type.is_some());
    }

    #[test]
    fn test_get_entity_request_with_outer_id() {
        let config = Config::default();
        let request = GetEntityRequest::new(config, "enterprise_0")
            .provider("my_system")
            .outer_id("outer_12345")
            .user_id_type(UserIdType::UserId);

        assert_eq!(request.entity_id, "enterprise_0");
        assert_eq!(request.provider, Some("my_system".to_string()));
        assert_eq!(request.outer_id, Some("outer_12345".to_string()));
    }

    #[tokio::test]
    async fn test_get_entity_request_validation() {
        let config = Config::default();

        // 测试 entity_id 为空
        let request = GetEntityRequest::new(config.clone(), "");
        assert!(request
            .execute_with_options(RequestOption::default())
            .await
            .is_err());

        // 测试 provider 过短
        let request2 = GetEntityRequest::new(config.clone(), "enterprise_0")
            .provider("a")
            .outer_id("outer_123");
        assert!(request2
            .execute_with_options(RequestOption::default())
            .await
            .is_err());

        // 测试 provider 过长
        let request3 = GetEntityRequest::new(config.clone(), "enterprise_0")
            .provider("a".repeat(33))
            .outer_id("outer_123");
        assert!(request3
            .execute_with_options(RequestOption::default())
            .await
            .is_err());

        // 测试 outer_id 过长
        let request4 = GetEntityRequest::new(config.clone(), "enterprise_0")
            .provider("my_system")
            .outer_id("a".repeat(65));
        assert!(request4
            .execute_with_options(RequestOption::default())
            .await
            .is_err());

        // 测试缺少 outer_id
        let request5 = GetEntityRequest::new(config.clone(), "enterprise_0").provider("my_system");
        assert!(request5
            .execute_with_options(RequestOption::default())
            .await
            .is_err());
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(GetEntityResp::data_format(), ResponseFormat::Data);
    }
}
