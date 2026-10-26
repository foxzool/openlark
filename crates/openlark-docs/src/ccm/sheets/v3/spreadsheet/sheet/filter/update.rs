/// 更新筛选
///
/// 更新子表筛选范围中的列筛选条件。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/update
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

/// 更新筛选请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFilterBody {
    /// 列标识。
    pub col: String,
    /// 新的筛选条件。
    pub condition: Condition,
}

/// 兼容别名：历史导出名保留为请求体类型。
pub type UpdateFilterRequest = UpdateFilterBody;

/// 更新筛选响应体 data（data 为 `{}`）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateFilterResponse {}

impl ApiResponseTrait for UpdateFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新筛选
///
/// 更新指定子表的列筛选条件。
pub async fn update_filter(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    params: UpdateFilterBody,
) -> SDKResult<UpdateFilterResponse> {
    update_filter_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        params,
        RequestOption::default(),
    )
    .await
}

/// 更新筛选（带选项）
///
/// 更新指定子表的列筛选条件，并允许传入自定义请求选项。
pub async fn update_filter_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    params: UpdateFilterBody,
    option: RequestOption,
) -> SDKResult<UpdateFilterResponse> {
    let api_endpoint =
        SheetsApiV3::UpdateFilter(spreadsheet_token.to_string(), sheet_id.to_string());
    let api_request: ApiRequest<UpdateFilterResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "更新筛选")?);

    Transport::request_typed(api_request, config, Some(option), "更新筛选").await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PUT .../sheets/{sheet_id}/filter → UpdateFilterResponse（空 data）。
    /// 请求体按官方示例：col/condition{filter_type,compare_type,expected}。
    #[tokio::test]
    async fn test_update_filter_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path(
                "/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/filter",
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

        update_filter(
            &config,
            "tokenAbc",
            "sheetId001",
            UpdateFilterBody {
                col: "G".into(),
                condition: Condition {
                    filter_type: "text".into(),
                    compare_type: Some("beginsWith".into()),
                    expected: vec!["a".into()],
                },
            },
        )
        .await
        .expect("更新筛选应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/filter"
        );
        // 校验请求体与官方示例一致
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["col"], "G");
        assert_eq!(sent["condition"]["filter_type"], "text");
        assert_eq!(sent["condition"]["compare_type"], "beginsWith");
        assert_eq!(sent["condition"]["expected"], json!(["a"]));
    }
}
