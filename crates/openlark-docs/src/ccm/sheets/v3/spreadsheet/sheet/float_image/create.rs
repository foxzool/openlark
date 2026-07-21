/// 创建浮动图片
///
/// 根据传入的参数创建一张浮动图片。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/create
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

/// 创建浮动图片请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFloatImageRequest {
    /// 浮动图片 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub float_image_id: Option<String>,
    /// 浮动图片 token（素材 file_token）
    pub float_image_token: String,
    /// 浮动图片左上角所在单元格位置（只允许单个单元格）
    pub range: String,
    /// 宽度。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    /// 高度。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    /// X 轴偏移。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_x: Option<f64>,
    /// Y 轴偏移。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_y: Option<f64>,
}

/// 创建浮动图片响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFloatImageResponse {
    /// 新建后的浮动图片。
    pub float_image: FloatImage,
}

impl ApiResponseTrait for CreateFloatImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建浮动图片
///
/// 在指定工作表中创建一张浮动图片。
pub async fn create_float_image(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    params: CreateFloatImageRequest,
) -> SDKResult<CreateFloatImageResponse> {
    create_float_image_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        params,
        RequestOption::default(),
    )
    .await
}

/// 创建浮动图片（带请求选项）
///
/// 在指定工作表中创建一张浮动图片，并允许传入自定义请求选项。
pub async fn create_float_image_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    params: CreateFloatImageRequest,
    option: RequestOption,
) -> SDKResult<CreateFloatImageResponse> {
    let api_endpoint =
        SheetsApiV3::CreateFloatImage(spreadsheet_token.to_string(), sheet_id.to_string());
    let api_request: ApiRequest<CreateFloatImageResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "创建浮动图片")?);

    Transport::request_typed(api_request, config, Some(option), "创建浮动图片").await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST .../float_images → CreateFloatImageResponse（float_image）。
    #[tokio::test]
    async fn test_create_float_image_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/float_images"))
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

        let resp = create_float_image(
            &config,
            "tokenAbc",
            "sheetId001",
            CreateFloatImageRequest {
                float_image_id: None,
                float_image_token: "tok001".into(),
                range: "A1".into(),
                width: None,
                height: None,
                offset_x: None,
                offset_y: None,
            },
        )
        .await
        .expect("创建浮动图片应成功");
        assert_eq!(resp.float_image.float_image_id, "fi001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/float_images"
        );
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["float_image_token"], "tok001");
    }
}
