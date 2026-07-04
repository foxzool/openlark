//! 获取所有 OKR 分类
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v2/category/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::Deserialize;
use std::sync::Arc;

/// 获取所有 OKR 分类请求。
#[derive(Debug, Clone)]
pub struct Request {
    config: Arc<Config>,
}

impl Request {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<ListCategoryResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListCategoryResponse> {
        let req: ApiRequest<ListCategoryResponse> = ApiRequest::get("/open-apis/okr/v2/categories");
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取所有 OKR 分类", "响应数据为空")
        })
    }
}

/// 获取所有 OKR 分类响应。
#[derive(Debug, Clone, Deserialize)]
pub struct ListCategoryResponse {
    /// 是否还有更多项。
    #[serde(default)]
    pub has_more: Option<bool>,
    /// 分页标记，当 has_more 为 true 时返回，否则不返回。
    #[serde(default)]
    pub page_token: Option<String>,
    /// 分类列表。
    #[serde(default)]
    pub items: Option<Vec<Category>>,
}

impl ApiResponseTrait for ListCategoryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// OKR 分类。
#[derive(Debug, Clone, Deserialize)]
pub struct Category {
    /// 分类 ID。
    pub id: String,
    /// 分类的创建时间，毫秒级时间戳。
    pub create_time: String,
    /// 分类的更新时间，毫秒级时间戳。
    pub update_time: String,
    /// 分类类型。
    pub category_type: String,
    /// 是否启用。
    pub enabled: bool,
    /// 颜色。
    pub color: String,
    /// 名称。
    pub name: CategoryName,
}

/// 分类名称。
#[derive(Debug, Clone, Deserialize)]
pub struct CategoryName {
    /// 中文名。
    #[serde(default)]
    pub zh: Option<String>,
    /// 英文名。
    #[serde(default)]
    pub en: Option<String>,
    /// 日文名。
    #[serde(default)]
    pub ja: Option<String>,
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
    fn test_list_category_response_deserialize() {
        let json = serde_json::json!({
            "has_more": true,
            "page_token": "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0=",
            "items": [
                {
                    "id": "7342342398472398473",
                    "create_time": "1760604634563",
                    "update_time": "1760604634563",
                    "category_type": "person",
                    "enabled": true,
                    "color": "blue",
                    "name": {
                        "zh": "中文",
                        "en": "英文",
                        "ja": "日文"
                    }
                }
            ]
        });
        let resp: ListCategoryResponse = serde_json::from_value(json).expect("反序列化失败");
        assert_eq!(resp.has_more, Some(true));
        assert_eq!(
            resp.page_token,
            Some("eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0=".to_string())
        );
        let items = resp.items.expect("items 不应为空");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, "7342342398472398473");
        assert_eq!(items[0].category_type, "person");
        assert!(items[0].enabled);
        assert_eq!(items[0].color, "blue");
        assert_eq!(items[0].name.zh, Some("中文".to_string()));
        assert_eq!(items[0].name.en, Some("英文".to_string()));
        assert_eq!(items[0].name.ja, Some("日文".to_string()));
    }
}
