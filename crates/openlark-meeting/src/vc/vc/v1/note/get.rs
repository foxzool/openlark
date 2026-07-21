//! 获取纪要详情
//!
//! docPath:

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use std::sync::Arc;

/// 获取纪要详情请求。
#[derive(Debug, Clone)]
pub struct NoteGetRequest {
    config: Arc<Config>,
    note_id: String,
}

impl NoteGetRequest {
    /// 创建请求。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            note_id: String::new(),
        }
    }

    /// 设置路径参数 `note_id`。
    pub fn note_id(mut self, note_id: impl Into<String>) -> Self {
        self.note_id = note_id.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.note_id, "note_id 不能为空");
        let path = format!("/open-apis/vc/v1/notes/{}", self.note_id);
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(path);
        Transport::request_typed(req, &self.config, Some(option), "获取纪要详情").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _request = NoteGetRequest::new(config);
    }
}
