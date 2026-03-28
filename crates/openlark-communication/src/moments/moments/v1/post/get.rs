//! 查询帖子信息
//!
//! docPath: <https://open.feishu.cn/document/moments-v1/post/get>

use std::collections::HashMap;

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::extract_response_data;
use crate::endpoints::moments::MOMENTS_V1_POST_GET;

/// 查询帖子信息响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPostResponse {
    pub post: Post,
}

impl ApiResponseTrait for GetPostResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 帖子信息（字段随文档演进，未显式建模字段使用 `extra` 透传）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_key_list: Option<Vec<String>>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 查询帖子信息请求
///
/// 用于查询指定帖子的详细信息。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `post_id`: 帖子 ID，必填
/// - `user_id_type`: 用户 ID 类型，可选
///
/// # 示例
///
/// ```rust,ignore
/// let request = GetPostRequest::new(config)
///     .post_id("post_xxx")
///     .user_id_type("open_id")
///     .execute().await?;
/// ```
pub struct GetPostRequest {
    config: Config,
    post_id: String,
    user_id_type: Option<String>,
}

impl GetPostRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            post_id: String::new(),
            user_id_type: None,
        }
    }

    /// 帖子 ID（路径参数）
    pub fn post_id(mut self, post_id: impl Into<String>) -> Self {
        self.post_id = post_id.into();
        self
    }

    /// 用户 ID 类型（查询参数，可选）
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/moments-v1/post/get>
    pub async fn execute(self) -> SDKResult<GetPostResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetPostResponse> {
        // === 必填字段验证 ===
        validate_required!(self.post_id, "post_id 不能为空");

        // url: GET:/open-apis/moments/v1/posts/:post_id
        let url = MOMENTS_V1_POST_GET.replace("{post_id}", &self.post_id);
        let mut req: ApiRequest<GetPostResponse> = ApiRequest::get(url);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", &user_id_type);
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询帖子信息")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_get_post_request_builder() {
        let config = Config::default();
        let request = GetPostRequest::new(config)
            .post_id("post_xxx")
            .user_id_type("open_id");
        assert_eq!(request.post_id, "post_xxx");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_get_post_request_default_values() {
        let config = Config::default();
        let request = GetPostRequest::new(config);
        assert_eq!(request.post_id, "");
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_get_post_request_with_post_id_only() {
        let config = Config::default();
        let request = GetPostRequest::new(config).post_id("post_123");
        assert_eq!(request.post_id, "post_123");
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_get_post_request_chaining() {
        let config = Config::default();
        let request = GetPostRequest::new(config)
            .post_id("post_xxx")
            .user_id_type("user_id");
        assert_eq!(request.post_id, "post_xxx");
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
    }

    #[test]
    fn test_get_post_request_with_different_user_id_types() {
        let config = Config::default();
        let request1 = GetPostRequest::new(config.clone()).user_id_type("open_id");
        let request2 = GetPostRequest::new(config).user_id_type("union_id");
        assert_eq!(request1.user_id_type, Some("open_id".to_string()));
        assert_eq!(request2.user_id_type, Some("union_id".to_string()));
    }
}
