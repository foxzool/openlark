/// 删除浮动图片
///
/// 删除 float_image_id 对应的浮动图片。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/delete
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 删除浮动图片响应体 data（data 为 `{}`）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteFloatImageResponse {}

impl ApiResponseTrait for DeleteFloatImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除浮动图片
pub async fn delete_float_image(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    float_image_id: &str,
) -> SDKResult<DeleteFloatImageResponse> {
    delete_float_image_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        float_image_id,
        RequestOption::default(),
    )
    .await
}

/// 删除浮动图片（带请求选项）
pub async fn delete_float_image_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    float_image_id: &str,
    option: RequestOption,
) -> SDKResult<DeleteFloatImageResponse> {
    let api_endpoint = SheetsApiV3::DeleteFloatImage(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        float_image_id.to_string(),
    );
    let api_request: ApiRequest<DeleteFloatImageResponse> =
        ApiRequest::delete(&api_endpoint.to_url());

    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "删除浮动图片")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：DELETE .../float_images/{float_image_id} → DeleteFloatImageResponse（空 data）。
    #[tokio::test]
    async fn test_delete_float_image_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path(
                "/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/float_images/fi001",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {}
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        delete_float_image(&config, "tokenAbc", "sheetId001", "fi001")
            .await
            .expect("删除浮动图片应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/float_images/fi001"
        );
    }
}
