//! 获取用户 OKR 周期列表
//!
//! docPath: https://open.feishu.cn/document/server-docs/okr-v2/cycle/list

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use std::sync::Arc;

/// 获取用户 OKR 周期列表请求。
#[derive(Debug, Clone)]
pub struct Request {
    config: Arc<Config>,
    user_id: String,
    user_id_type: Option<String>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl Request {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            user_id: String::new(),
            user_id_type: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置用户 ID（必填）。
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = user_id.into();
        self
    }

    /// 设置用户 ID 类型（可选）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置分页大小（可选）。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（可选）。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.user_id, "user_id 不能为空");

        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::get("/open-apis/okr/v2/cycles").query("user_id", &self.user_id);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type);
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取用户 OKR 周期列表", "响应数据为空")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let req = Request::new(config);
        assert!(req.user_id.is_empty());
        assert!(req.user_id_type.is_none());
        assert!(req.page_size.is_none());
        assert!(req.page_token.is_none());
    }

    #[test]
    fn builder_sets_params() {
        let config = Arc::new(Config::default());
        let req = Request::new(config)
            .user_id("user_123")
            .user_id_type("open_id")
            .page_size(50)
            .page_token("token_abc");
        assert_eq!(req.user_id, "user_123");
        assert_eq!(req.user_id_type, Some("open_id".to_string()));
        assert_eq!(req.page_size, Some(50));
        assert_eq!(req.page_token, Some("token_abc".to_string()));
    }

    #[test]
    fn test_url_construction() {
        use crate::common::api_endpoints::OkrApiV2;
        let url = OkrApiV2::CycleList.to_url();
        assert_eq!(url, "/open-apis/okr/v2/cycles");
    }
}
