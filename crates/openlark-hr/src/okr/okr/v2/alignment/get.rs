//! 获取 OKR 对齐
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v2/alignment/get>

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
use crate::okr::okr::v2::common::models::Alignment;

/// 获取 OKR 对齐请求。
#[derive(Debug, Clone)]
pub struct Request {
    config: Arc<Config>,
    alignment_id: String,
}

impl Request {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            alignment_id: String::new(),
        }
    }

    /// 设置路径参数 `alignment_id`。
    pub fn alignment_id(mut self, val: impl Into<String>) -> Self {
        self.alignment_id = val.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetAlignmentResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetAlignmentResponse> {
        validate_required!(self.alignment_id, "alignment_id 不能为空");
        let path = OkrApiV2::AlignmentGet(self.alignment_id).to_url();
        let req: ApiRequest<GetAlignmentResponse> = ApiRequest::get(path);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("获取 OKR 对齐", "响应数据为空"))
    }
}

/// 获取 OKR 对齐响应。
#[derive(Debug, Clone, Deserialize)]
pub struct GetAlignmentResponse {
    /// 对齐详情。
    pub alignment: Alignment,
}

impl ApiResponseTrait for GetAlignmentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
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
    fn test_get_alignment_response_deserialize() {
        let json = serde_json::json!({
            "alignment": {
                "id": "A-123",
                "create_time": "1700000000000",
                "update_time": "1700000000000",
                "from_owner": {"owner_type": "user", "user_id": "ou_from"},
                "to_owner": {"owner_type": "user", "user_id": "ou_to"},
                "from_entity_type": 1,
                "from_entity_id": "E-1",
                "to_entity_type": 2,
                "to_entity_id": "E-2"
            }
        });
        let resp: GetAlignmentResponse = serde_json::from_value(json).expect("反序列化失败");
        assert_eq!(resp.alignment.id, "A-123");
        assert_eq!(resp.alignment.from_entity_type, 1);
        assert_eq!(resp.alignment.to_owner.owner_type, "user");
    }
}
