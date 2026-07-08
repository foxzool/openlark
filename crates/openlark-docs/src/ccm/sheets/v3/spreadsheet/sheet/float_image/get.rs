/// 获取浮动图片
///
/// 根据 float_image_id 获取对应浮动图片的信息。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/get
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

use super::FloatImage;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 获取浮动图片响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFloatImageResponse {
    /// 浮动图片详情。
    pub float_image: FloatImage,
}

impl ApiResponseTrait for GetFloatImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取浮动图片
///
/// 获取指定浮动图片的详细信息。
pub async fn get_float_image(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    float_image_id: &str,
) -> SDKResult<GetFloatImageResponse> {
    get_float_image_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        float_image_id,
        RequestOption::default(),
    )
    .await
}

/// 获取浮动图片（带请求选项）
///
/// 获取指定浮动图片的详细信息，并允许传入自定义请求选项。
pub async fn get_float_image_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    float_image_id: &str,
    option: RequestOption,
) -> SDKResult<GetFloatImageResponse> {
    let api_endpoint = SheetsApiV3::GetFloatImage(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        float_image_id.to_string(),
    );
    let api_request: ApiRequest<GetFloatImageResponse> = ApiRequest::get(&api_endpoint.to_url());

    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "获取浮动图片")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET .../float_images/{float_image_id} → GetFloatImageResponse（float_image）。
    #[tokio::test]
    async fn test_get_float_image_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/float_images/fi001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "float_image": { "float_image_id": "fi001", "float_image_token": "tok001", "range": "A1", "width": 100, "height": 50, "offset_x": 0, "offset_y": 0 } }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = get_float_image(&config, "tokenAbc", "sheetId001", "fi001")
            .await
            .expect("获取浮动图片应成功");
        assert_eq!(resp.float_image.float_image_id, "fi001");
        assert_eq!(resp.float_image.width, 100);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/float_images/fi001"
        );
    }
}
