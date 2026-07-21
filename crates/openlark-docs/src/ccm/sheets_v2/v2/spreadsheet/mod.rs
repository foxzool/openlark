/// CCM Sheet V2 表格基础API 模块
///
/// 表格基础操作API实现，包含表格的增删改查：
/// - get_spreadsheet: 获取表格信息
/// - create_spreadsheet: 创建表格
/// - update_spreadsheet: 更新表格
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};

use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};

/// 表格基础API结构体
#[derive(Debug, Clone)]
pub struct SpreadsheetApi {
    config: Config,
}

impl SpreadsheetApi {
    /// 创建新的表格基础API实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}

// 导出模型定义
/// models 子模块。
pub mod models;
// models 模块显式导出
/// 重新导出相关类型。
pub use models::{
    CreateSpreadsheetParams, CreateSpreadsheetResponse, CreateSpreadsheetResult,
    GetSpreadsheetParams, GetSpreadsheetResponse, SpreadsheetInfo, SpreadsheetSheetInfo,
    UpdateSpreadsheetParams, UpdateSpreadsheetResponse, UpdateSpreadsheetResult, UserInfo,
};

impl ApiResponseTrait for GetSpreadsheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for CreateSpreadsheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for UpdateSpreadsheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取表格信息
///
/// 根据 spreadsheetToken 获取表格的详细信息，包括工作表信息。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet/get
pub async fn get_spreadsheet(
    config: &Config,
    spreadsheet_token: &str,
    params: GetSpreadsheetParams,
) -> SDKResult<GetSpreadsheetResponse> {
    get_spreadsheet_with_options(config, spreadsheet_token, params, RequestOption::default()).await
}

/// 获取表格信息（带选项）
///
/// 根据 spreadsheetToken 获取表格的详细信息，包括工作表信息。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet/get
pub async fn get_spreadsheet_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: GetSpreadsheetParams,
    option: RequestOption,
) -> SDKResult<GetSpreadsheetResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::GetSpreadsheet(spreadsheet_token.to_string());

    // 创建API请求（按 csv 对齐为 GET /open-apis/sheets/v3/spreadsheets/{token}）
    let api_request: ApiRequest<GetSpreadsheetResponse> = api_endpoint
        .to_request()
        .query_opt("include_sheet", params.include_sheet.map(|v| v.to_string()));

    // 发送请求并提取响应数据
    Transport::request_typed(api_request, config, Some(option), "获取表格信息").await
}

/// 创建表格
///
/// 创建新的电子表格，支持指定标题和文件夹位置。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet/create
pub async fn create_spreadsheet(
    config: &Config,
    params: CreateSpreadsheetParams,
) -> SDKResult<CreateSpreadsheetResponse> {
    create_spreadsheet_with_options(config, params, RequestOption::default()).await
}

/// 创建表格（带选项）
///
/// 创建新的电子表格，支持指定标题和文件夹位置。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet/create
pub async fn create_spreadsheet_with_options(
    config: &Config,
    params: CreateSpreadsheetParams,
    option: RequestOption,
) -> SDKResult<CreateSpreadsheetResponse> {
    // 验证必填字段
    validate_required!(params.title.trim(), "表格标题不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::CreateSpreadsheet;

    // 创建API请求
    let api_request: ApiRequest<CreateSpreadsheetResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "创建表格")?);

    // 发送请求并提取响应数据
    Transport::request_typed(api_request, config, Some(option), "创建表格").await
}

/// 更新表格
///
/// 根据 spreadsheetToken 更新表格的基本信息，如标题。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet/patch
pub async fn update_spreadsheet(
    config: &Config,
    spreadsheet_token: &str,
    params: UpdateSpreadsheetParams,
) -> SDKResult<UpdateSpreadsheetResponse> {
    update_spreadsheet_with_options(config, spreadsheet_token, params, RequestOption::default())
        .await
}

/// 更新表格（带选项）
///
/// 根据 spreadsheetToken 更新表格的基本信息，如标题。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet/patch
pub async fn update_spreadsheet_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: UpdateSpreadsheetParams,
    option: RequestOption,
) -> SDKResult<UpdateSpreadsheetResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::UpdateSpreadsheet(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<UpdateSpreadsheetResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "更新表格")?);

    // 发送请求并提取响应数据
    Transport::request_typed(api_request, config, Some(option), "更新表格").await
}

// API函数已经在模块中定义，不需要重复导出

// 模型已在同一个模块中定义，不需要重新导出

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/sheets/v3/spreadsheets/{token} → GetSpreadsheetResponse。
    #[tokio::test]
    async fn test_get_spreadsheet_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/sheets/v3/spreadsheets/token001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {}
            })))
            .mount(&server)
            .await;
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();
        let resp = get_spreadsheet(
            &config,
            "token001",
            GetSpreadsheetParams {
                include_sheet: None,
            },
        )
        .await
        .expect("获取表格应成功");
        assert!(resp.data.is_none());
        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/sheets/v3/spreadsheets/token001"
        );
    }
}
