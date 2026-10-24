//! 下载白板为图片（v1）
//! docPath: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/download_as_image>
//!
//! 官方 GET 接口无请求体与查询参数，成功时返回图片二进制流。

use openlark_core::{
    SDKResult,
    api::{ApiRequest, Response},
    config::Config,
    validate_required,
};
use std::sync::Arc;

/// 下载白板图片请求构建器。
#[derive(Debug, Clone)]
pub struct DownloadWhiteboardAsImageRequestV1 {
    config: Arc<Config>,
    board_id: String,
}

impl DownloadWhiteboardAsImageRequestV1 {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, board_id: impl Into<String>) -> Self {
        Self {
            config,
            board_id: board_id.into(),
        }
    }

    /// 使用默认请求选项执行请求，返回图片二进制内容。
    pub async fn execute(self) -> SDKResult<Response<Vec<u8>>> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求，返回图片二进制内容。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<Response<Vec<u8>>> {
        validate_required!(self.board_id.trim(), "白板 ID 不能为空");

        let api_endpoint =
            crate::common::api_endpoints::BoardApiV1::WhiteboardDownloadAsImage(self.board_id);
        let request = ApiRequest::<Vec<u8>>::get(api_endpoint.to_url());

        openlark_core::http::Transport::request(request, &self.config, Some(option)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path, query_param_is_missing};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[test]
    fn test_whiteboard_download_as_image_v1_url() {
        let endpoint = crate::common::api_endpoints::BoardApiV1::WhiteboardDownloadAsImage(
            "test_board_id".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/board/v1/whiteboards/test_board_id/download_as_image"
        );
    }

    /// 端到端：GET /open-apis/board/v1/whiteboards/{whiteboard_id}/download_as_image，
    /// 无请求体与查询参数，成功返回图片二进制流。
    #[tokio::test]
    async fn test_download_whiteboard_as_image_returns_binary_on_success() {
        let server = MockServer::start().await;
        let body = b"fake-png-binary-payload".to_vec();
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/board/v1/whiteboards/wb_123/download_as_image",
            ))
            .and(query_param_is_missing("format"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(body.clone()))
            .mount(&server)
            .await;

        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let resp = DownloadWhiteboardAsImageRequestV1::new(config, "wb_123")
            .execute()
            .await
            .expect("下载白板图片应成功");
        assert_eq!(resp.data.expect("响应应包含二进制数据"), body);

        let received = server.received_requests().await.expect("应收到请求");
        assert_eq!(received.len(), 1);
        assert!(
            received[0].body.is_empty(),
            "官方 GET 无请求体: {:?}",
            received[0].body
        );
        assert_eq!(
            received[0].url.path(),
            "/open-apis/board/v1/whiteboards/wb_123/download_as_image"
        );
    }
}
