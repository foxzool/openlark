//! 获取词条列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/baike-v1/entity/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::baike::baike::v1::models::{Entity, UserIdType};
use crate::common::api_endpoints::BaikeApiV1;

/// 获取词条列表响应（data）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListEntityResp {
    #[serde(default)]
    pub entities: Vec<Entity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListEntityResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取词条列表请求
pub struct ListEntityRequest {
    config: Config,
    page_size: Option<i32>,
    page_token: Option<String>,
    provider: Option<String>,
    user_id_type: Option<UserIdType>,
}

impl ListEntityRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            provider: None,
            user_id_type: None,
        }
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    pub fn provider(mut self, provider: impl Into<String>) -> Self {
        self.provider = Some(provider.into());
        self
    }

    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    pub async fn execute(self) -> SDKResult<ListEntityResp> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<ListEntityResp> {
        // ===== 参数校验 =====
        if let Some(page_size) = self.page_size {
            if !(1..=100).contains(&page_size) {
                return Err(openlark_core::error::validation_error(
                    "page_size",
                    "page_size 取值范围必须为 1~100",
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

        // ===== 构建请求 =====
        let mut api_request: ApiRequest<ListEntityResp> =
            ApiRequest::get(&BaikeApiV1::EntityList.to_url());
        if let Some(page_size) = self.page_size {
            api_request = api_request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = &self.page_token {
            api_request = api_request.query("page_token", page_token);
        }
        if let Some(provider) = &self.provider {
            api_request = api_request.query("provider", provider);
        }
        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type.as_str());
        }

        // ===== 发送请求并返回结果 =====
        let response: Response<ListEntityResp> =
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
    fn test_list_entity_request_builder() {
        let config = Config::default();
        let request = ListEntityRequest::new(config)
            .page_size(20)
            .page_token("token123")
            .user_id_type(UserIdType::OpenId);

        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token, Some("token123".to_string()));
        assert!(request.user_id_type.is_some());
    }

    #[test]
    fn test_list_entity_request_with_provider() {
        let config = Config::default();
        let request = ListEntityRequest::new(config)
            .provider("my_system")
            .page_size(50);

        assert_eq!(request.provider, Some("my_system".to_string()));
        assert_eq!(request.page_size, Some(50));
    }

    #[tokio::test]
    async fn test_list_entity_request_validation() {
        let config = Config::default();

        // 测试 page_size 超出范围
        let request = ListEntityRequest::new(config.clone()).page_size(0);
        assert!(request
            .execute_with_options(RequestOption::default())
            .await
            .is_err());

        let request2 = ListEntityRequest::new(config.clone()).page_size(101);
        assert!(request2
            .execute_with_options(RequestOption::default())
            .await
            .is_err());

        // 测试 provider 过短
        let request3 = ListEntityRequest::new(config.clone()).provider("a");
        assert!(request3
            .execute_with_options(RequestOption::default())
            .await
            .is_err());

        // 测试 provider 过长
        let request4 = ListEntityRequest::new(config.clone()).provider("a".repeat(33));
        assert!(request4
            .execute_with_options(RequestOption::default())
            .await
            .is_err());
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(ListEntityResp::data_format(), ResponseFormat::Data);
    }
}
