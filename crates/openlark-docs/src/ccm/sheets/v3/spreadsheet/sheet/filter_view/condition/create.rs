/// 创建筛选条件
///
/// 在筛选视图的筛选范围的某一列创建筛选条件。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/create
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

/// 创建筛选条件请求体。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFilterConditionRequest {
    /// 设置筛选条件的列，用字母表示（如 "E"）
    pub condition_id: String,
    /// 筛选类型。
    pub filter_type: String,
    /// 比较类型。
    pub compare_type: String,
    /// 期望值列表。
    pub expected: Vec<String>,
}

/// 创建筛选条件响应体 data。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFilterConditionResponse {
    /// 新建后的筛选条件。
    pub condition: Condition,
}

impl ApiResponseTrait for CreateFilterConditionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建筛选条件。
pub async fn create_filter_condition(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
    params: CreateFilterConditionRequest,
) -> SDKResult<CreateFilterConditionResponse> {
    create_filter_condition_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        filter_view_id,
        params,
        RequestOption::default(),
    )
    .await
}

/// 创建筛选条件（支持请求选项）。
pub async fn create_filter_condition_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
    params: CreateFilterConditionRequest,
    option: RequestOption,
) -> SDKResult<CreateFilterConditionResponse> {
    let api_endpoint = SheetsApiV3::CreateFilterCondition(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        filter_view_id.to_string(),
    );
    let api_request: ApiRequest<CreateFilterConditionResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "创建筛选条件")?);

    Transport::request_typed(api_request, config, Some(option), "创建筛选条件").await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST .../filter_views/{fv}/conditions → CreateFilterConditionResponse（condition）。
    #[tokio::test]
    async fn test_create_filter_condition_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/filter_views/fv001/conditions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "condition": { "filter_view_id": "fv001", "column_id": "E", "operator": "equals" }
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = create_filter_condition(
            &config,
            "tokenAbc",
            "sheetId001",
            "fv001",
            CreateFilterConditionRequest {
                condition_id: "E".into(),
                filter_type: "noCheck".into(),
                compare_type: "none".into(),
                expected: vec![],
            },
        )
        .await
        .expect("创建筛选条件应成功");
        assert_eq!(resp.condition.column_id, "E");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/filter_views/fv001/conditions"
        );
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["condition_id"], "E");
    }
}
