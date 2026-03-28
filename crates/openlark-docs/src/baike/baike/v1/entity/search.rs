//! 模糊搜索词条
//!
//! docPath: <https://open.feishu.cn/document/server-docs/baike-v1/entity/search>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchEntityReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_filter: Option<ClassificationFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creators: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchEntityResponse {
    #[serde(default)]
    pub entities: Vec<Entity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for SearchEntityResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ClassificationFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
}

use crate::baike::baike::v1::models::{Entity, UserIdType};
use crate::common::api_endpoints::BaikeApiV1;

/// 模糊搜索词条请求
pub struct SearchEntityRequest {
    config: Config,
    page_size: Option<i32>,
    page_token: Option<String>,
    user_id_type: Option<UserIdType>,
    req: SearchEntityReqBody,
}

impl SearchEntityRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            user_id_type: None,
            req: SearchEntityReqBody::default(),
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

    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.req.query = Some(query.into());
        self
    }

    pub fn classification_filter(mut self, filter: ClassificationFilter) -> Self {
        self.req.classification_filter = Some(filter);
        self
    }

    pub fn sources(mut self, sources: Vec<i32>) -> Self {
        self.req.sources = Some(sources);
        self
    }

    pub fn creators(mut self, creators: Vec<String>) -> Self {
        self.req.creators = Some(creators);
        self
    }

    pub async fn execute(self) -> SDKResult<SearchEntityResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SearchEntityResponse> {
        // ===== 参数校验 =====
        if let Some(page_size) = self.page_size {
            if !(1..=100).contains(&page_size) {
                return Err(openlark_core::error::validation_error(
                    "page_size",
                    "page_size 取值范围必须为 1~100",
                ));
            }
        }
        if let Some(query) = &self.req.query {
            let len = query.chars().count();
            if !(1..=100).contains(&len) {
                return Err(openlark_core::error::validation_error(
                    "query",
                    "query 长度必须在 1~100 字符之间",
                ));
            }
        }
        if let Some(sources) = &self.req.sources {
            for (idx, source) in sources.iter().enumerate() {
                match source {
                    1..=4 => {}
                    _ => {
                        return Err(openlark_core::error::validation_error(
                            &format!("sources[{}]", idx),
                            "sources 仅支持 1/2/3/4",
                        ));
                    }
                }
            }
        }
        if let Some(creators) = &self.req.creators {
            for (idx, creator) in creators.iter().enumerate() {
                if creator.trim().is_empty() {
                    return Err(openlark_core::error::validation_error(
                        &format!("creators[{}]", idx),
                        "creators 不能包含空字符串",
                    ));
                }
            }
        }

        // ===== 构建请求 =====
        let mut api_request: ApiRequest<SearchEntityResponse> =
            ApiRequest::post(&BaikeApiV1::EntitySearch.to_url())
                .body(serde_json::to_value(&self.req)?);

        if let Some(page_size) = self.page_size {
            api_request = api_request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = &self.page_token {
            api_request = api_request.query("page_token", page_token);
        }
        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type.as_str());
        }

        // ===== 发送请求并返回结果 =====
        let response: Response<SearchEntityResponse> =
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
    fn test_search_entity_request_builder() {
        let config = Config::default();
        let request = SearchEntityRequest::new(config)
            .query("测试词条")
            .page_size(20)
            .user_id_type(UserIdType::OpenId);

        assert_eq!(request.req.query, Some("测试词条".to_string()));
        assert_eq!(request.page_size, Some(20));
        assert!(request.user_id_type.is_some());
    }

    #[test]
    fn test_search_entity_request_with_filters() {
        let config = Config::default();
        let filter = ClassificationFilter {
            include: Some(vec!["分类1".to_string(), "分类2".to_string()]),
            exclude: Some(vec!["分类3".to_string()]),
        };
        let request = SearchEntityRequest::new(config)
            .query("搜索关键词")
            .classification_filter(filter)
            .sources(vec![1, 2])
            .creators(vec!["user_123".to_string()]);

        assert_eq!(request.req.query, Some("搜索关键词".to_string()));
        assert!(request.req.classification_filter.is_some());
        assert_eq!(request.req.sources, Some(vec![1, 2]));
        assert_eq!(request.req.creators, Some(vec!["user_123".to_string()]));
    }

    #[tokio::test]
    async fn test_search_entity_request_validation() {
        let config = Config::default();

        // 测试 page_size 超出范围
        let request = SearchEntityRequest::new(config.clone()).page_size(0);
        assert!(request
            .execute_with_options(RequestOption::default())
            .await
            .is_err());

        let request2 = SearchEntityRequest::new(config.clone()).page_size(101);
        assert!(request2
            .execute_with_options(RequestOption::default())
            .await
            .is_err());

        // 测试 query 过长
        let request3 = SearchEntityRequest::new(config.clone()).query("a".repeat(101));
        assert!(request3
            .execute_with_options(RequestOption::default())
            .await
            .is_err());

        // 测试 sources 包含无效值
        let request4 = SearchEntityRequest::new(config.clone()).sources(vec![1, 5, 3]);
        assert!(request4
            .execute_with_options(RequestOption::default())
            .await
            .is_err());

        // 测试 creators 包含空字符串
        let request5 = SearchEntityRequest::new(config.clone())
            .creators(vec!["user_123".to_string(), "".to_string()]);
        assert!(request5
            .execute_with_options(RequestOption::default())
            .await
            .is_err());
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(SearchEntityResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_classification_filter_default() {
        let filter = ClassificationFilter::default();
        assert!(filter.include.is_none());
        assert!(filter.exclude.is_none());
    }

    #[test]
    fn test_search_entity_req_body_default() {
        let req = SearchEntityReqBody::default();
        assert!(req.query.is_none());
        assert!(req.classification_filter.is_none());
        assert!(req.sources.is_none());
        assert!(req.creators.is_none());
    }
}
