/// 更新浮动图片
///
/// 更新已有的浮动图片位置和宽高。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/patch
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

/// 更新浮动图片请求体（字段均为可选）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateFloatImageRequest {
    /// 新的范围。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    /// 新的宽度。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    /// 新的高度。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    /// 新的 X 轴偏移。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_x: Option<f64>,
    /// 新的 Y 轴偏移。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_y: Option<f64>,
}

/// 更新浮动图片响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFloatImageResponse {
    /// 更新后的浮动图片。
    pub float_image: FloatImage,
}

impl ApiResponseTrait for UpdateFloatImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新浮动图片
///
/// 更新指定浮动图片的位置和尺寸。
pub async fn update_float_image(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    float_image_id: &str,
    params: UpdateFloatImageRequest,
) -> SDKResult<UpdateFloatImageResponse> {
    update_float_image_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        float_image_id,
        params,
        RequestOption::default(),
    )
    .await
}

/// 更新浮动图片（带请求选项）
///
/// 更新指定浮动图片的位置和尺寸，并允许传入自定义请求选项。
pub async fn update_float_image_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    float_image_id: &str,
    params: UpdateFloatImageRequest,
    option: RequestOption,
) -> SDKResult<UpdateFloatImageResponse> {
    let api_endpoint = SheetsApiV3::PatchFloatImage(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        float_image_id.to_string(),
    );
    let api_request: ApiRequest<UpdateFloatImageResponse> =
        ApiRequest::patch(&api_endpoint.to_url()).body(serialize_params(&params, "更新浮动图片")?);

    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "更新浮动图片")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PATCH .../float_images/{float_image_id} → UpdateFloatImageResponse（float_image）。
    #[tokio::test]
    async fn test_update_float_image_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
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

        let resp = update_float_image(
            &config,
            "tokenAbc",
            "sheetId001",
            "fi001",
            UpdateFloatImageRequest {
                range: Some("A1".into()),
                width: Some(100.0),
                height: None,
                offset_x: None,
                offset_y: None,
            },
        )
        .await
        .expect("更新浮动图片应成功");
        assert_eq!(resp.float_image.float_image_id, "fi001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/float_images/fi001"
        );
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["range"], "A1");
    }
}
