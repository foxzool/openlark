/// 创建筛选
///
/// 在子表内创建筛选。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/create
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

use super::Condition;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 创建筛选请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFilterRequest {
    /// 筛选范围。
    pub range: String,
    /// 列标识。
    pub col: String,
    /// 筛选条件。
    pub condition: Condition,
}

/// 创建筛选响应体 data（data 为 `{}`）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateFilterResponse {}

impl ApiResponseTrait for CreateFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建筛选
///
/// 在指定子表中创建筛选条件。
pub async fn create_filter(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    params: CreateFilterRequest,
) -> SDKResult<CreateFilterResponse> {
    create_filter_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        params,
        RequestOption::default(),
    )
    .await
}

/// 创建筛选（带请求选项）
///
/// 在指定子表中创建筛选条件，并允许传入自定义请求选项。
pub async fn create_filter_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    params: CreateFilterRequest,
    option: RequestOption,
) -> SDKResult<CreateFilterResponse> {
    let api_endpoint =
        SheetsApiV3::CreateFilter(spreadsheet_token.to_string(), sheet_id.to_string());
    let api_request: ApiRequest<CreateFilterResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "创建筛选")?);

    Transport::request_typed(api_request, config, Some(option), "创建筛选").await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST .../sheets/{sheet_id}/filter → CreateFilterResponse（空 data）。
    #[tokio::test]
    async fn test_create_filter_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/filter",
            ))
            .and(header("Authorization", "Bearer test-tenant-token"))
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

        create_filter_with_options(
            &config,
            "tokenAbc",
            "sheetId001",
            CreateFilterRequest {
                range: "A1:A10".into(),
                col: "0".into(),
                condition: Condition {
                    column_id: "A".into(),
                    operator: "equals".into(),
                    value: None,
                    ignore_case: None,
                },
            },
            RequestOption::builder()
                .tenant_access_token("test-tenant-token")
                .build(),
        )
        .await
        .expect("创建筛选应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/filter"
        );
        // 校验请求体透传
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["range"], "A1:A10");
        assert_eq!(sent["col"], "0");
        assert_eq!(sent["condition"]["operator"], "equals");
    }
}
