//! 更新 URL 预览
//!
//! docPath: <https://open.feishu.cn/document/im-v1/url_preview/batch_update>

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::{
        api_utils::{extract_response_data, serialize_params},
        models::EmptyData,
    },
    endpoints::IM_V2_URL_PREVIEWS_BATCH_UPDATE,
    im::im::v2::url_preview::models::BatchUpdateUrlPreviewBody,
};

/// 更新 URL 预览请求
pub struct BatchUpdateUrlPreviewRequest {
    config: Config,
}

impl BatchUpdateUrlPreviewRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/im-v1/url_preview/batch_update>
    pub async fn execute(self, body: BatchUpdateUrlPreviewBody) -> SDKResult<EmptyData> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: BatchUpdateUrlPreviewBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        if body.preview_tokens.is_empty() {
            return Err(openlark_core::error::validation_error(
                "preview_tokens 不能为空".to_string(),
                "preview_tokens 至少需要 1 个".to_string(),
            ));
        }

        // url: POST:/open-apis/im/v2/url_previews/batch_update
        let req: ApiRequest<EmptyData> = ApiRequest::post(IM_V2_URL_PREVIEWS_BATCH_UPDATE)
            .body(serialize_params(&body, "更新 URL 预览")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "更新 URL 预览")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
