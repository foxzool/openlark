//! 获取关键结果下的进展记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v2/key_result.progress/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::common::api_endpoints::OkrApiV2;
use crate::okr::okr::v2::common::models::ContentBlock;

/// 获取关键结果下的进展记录请求。
#[derive(Debug, Clone)]
pub struct Request {
    config: Arc<Config>,
    key_result_id: String,
}

impl Request {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            key_result_id: String::new(),
        }
    }

    /// 设置路径参数 `key_result_id`。
    pub fn key_result_id(mut self, val: impl Into<String>) -> Self {
        self.key_result_id = val.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<ListKeyResultProgressResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListKeyResultProgressResponse> {
        validate_required!(self.key_result_id, "key_result_id 不能为空");
        let path = OkrApiV2::KeyResultProgressList(self.key_result_id).to_url();
        let req: ApiRequest<ListKeyResultProgressResponse> = ApiRequest::get(path);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取关键结果下的进展记录", "响应数据为空")
        })
    }
}

/// 获取关键结果下的进展记录响应。
#[derive(Debug, Clone, Deserialize)]
pub struct ListKeyResultProgressResponse {
    /// 是否还有更多项。
    #[serde(default)]
    pub has_more: Option<bool>,
    /// 分页标记，当 has_more 为 true 时返回，否则不返回。
    #[serde(default)]
    pub page_token: Option<String>,
    /// 进展列表。
    #[serde(default)]
    pub items: Option<Vec<KeyResultProgress>>,
}

impl ApiResponseTrait for ListKeyResultProgressResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 关键结果进展。
#[derive(Debug, Clone, Deserialize)]
pub struct KeyResultProgress {
    /// 进展的 ID。
    pub id: String,
    /// 进展的创建时间，毫秒级时间戳。
    pub create_time: String,
    /// 进展的更新时间，毫秒级时间戳。
    pub update_time: String,
    /// 所有者。
    pub owner: KeyResultProgressOwner,
    /// 进展所属的实体类型。
    pub entity_type: i32,
    /// 进展所属的实体 ID。
    pub entity_id: String,
    /// 进展的内容（文档 block 结构，见 [`ContentBlock`]）。
    #[serde(default)]
    pub content: Option<ContentBlock>,
    /// 进展的进度。
    #[serde(default)]
    pub progress_rate: Option<KeyResultProgressRate>,
}

/// 关键结果进展所有者。
#[derive(Debug, Clone, Deserialize)]
pub struct KeyResultProgressOwner {
    /// 所有者类型（如 "user"）。
    pub owner_type: String,
    /// 员工 ID。
    #[serde(default)]
    pub user_id: Option<String>,
}

/// 关键结果进展进度。
#[derive(Debug, Clone, Deserialize)]
pub struct KeyResultProgressRate {
    /// 进展百分比，保留两位小数。
    #[serde(default)]
    pub progress_percent: Option<f64>,
    /// 进展状态。
    #[serde(default)]
    pub progress_status: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _req = Request::new(config);
    }

    #[test]
    fn test_list_key_result_progress_response_deserialize() {
        let json = serde_json::json!({
            "has_more": true,
            "page_token": "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0=",
            "items": [
                {
                    "id": "P-123",
                    "create_time": "1700000000000",
                    "update_time": "1700000000000",
                    "owner": {"owner_type": "user", "user_id": "ou_xxx"},
                    "entity_type": 3,
                    "entity_id": "KR-123",
                    "progress_rate": {"progress_percent": 50.21, "progress_status": 0}
                }
            ]
        });
        let resp: ListKeyResultProgressResponse =
            serde_json::from_value(json).expect("反序列化失败");
        assert_eq!(resp.has_more, Some(true));
        assert_eq!(
            resp.page_token,
            Some("eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0=".to_string())
        );
        let items = resp.items.expect("items 不应为空");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, "P-123");
        assert_eq!(items[0].entity_type, 3);
        assert_eq!(items[0].entity_id, "KR-123");
        assert_eq!(items[0].owner.user_id, Some("ou_xxx".to_string()));
        assert!(items[0].content.is_none());
        let rate = items[0]
            .progress_rate
            .as_ref()
            .expect("progress_rate 不应为空");
        assert_eq!(rate.progress_percent, Some(50.21));
        assert_eq!(rate.progress_status, Some(0));
    }

    #[test]
    fn test_list_key_result_progress_response_deserialize_empty() {
        let json = serde_json::json!({});
        let resp: ListKeyResultProgressResponse =
            serde_json::from_value(json).expect("反序列化失败");
        assert!(resp.items.is_none());
        assert!(resp.has_more.is_none());
        assert!(resp.page_token.is_none());
    }
}
