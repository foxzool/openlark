/// CCM Sheet V2 工作表API 模块
///
/// 工作表操作API实现，包含工作表的增删改查：
/// - add_sheet: 添加工作表
/// - get_sheet: 获取工作表信息
/// - update_sheet: 更新工作表
/// - delete_sheet: 删除工作表
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};

use crate::common::{
    api_endpoints::{CcmSheetApiOld, SheetsApiV3},
    api_utils::*,
};

/// 工作表API结构体
#[derive(Debug, Clone)]
pub struct SheetApi {
    config: Config,
}

impl SheetApi {
    /// 创建新的工作表API实例
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
    AddSheetParams, AddSheetResponse, AddSheetResult, DeleteSheetParams, DeleteSheetResponse,
    DeleteSheetResult, GetSheetParams, GetSheetResponse, SheetDetailInfo, UpdateSheetParams,
    UpdateSheetResponse, UpdateSheetResult,
};

impl ApiResponseTrait for AddSheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for GetSheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for UpdateSheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for DeleteSheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 添加工作表
///
/// 根据 spreadsheetToken 在电子表格中添加新的工作表。
/// docPath: /document/ukTMukTMukTM/uYTMzUjL2EzM14iNxMTN
pub async fn add_sheet(
    config: &Config,
    spreadsheet_token: &str,
    params: AddSheetParams,
) -> SDKResult<AddSheetResponse> {
    add_sheet_with_options(config, spreadsheet_token, params, RequestOption::default()).await
}

/// 添加工作表（带请求选项）
///
/// 根据 spreadsheetToken 在电子表格中添加新的工作表。
/// docPath: /document/ukTMukTMukTM/uYTMzUjL2EzM14iNxMTN
pub async fn add_sheet_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: AddSheetParams,
    option: RequestOption,
) -> SDKResult<AddSheetResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.title.trim(), "工作表标题不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::AddSheet(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<AddSheetResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "添加工作表")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "添加工作表")
}

/// 获取工作表信息
///
/// 根据 spreadsheetToken 和 sheet_id 获取工作表的详细信息。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/get
pub async fn get_sheet(
    config: &Config,
    spreadsheet_token: &str,
    params: GetSheetParams,
) -> SDKResult<GetSheetResponse> {
    get_sheet_with_options(config, spreadsheet_token, params, RequestOption::default()).await
}

/// 获取工作表信息（带请求选项）
///
/// 根据 spreadsheetToken 和 sheet_id 获取工作表的详细信息。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/get
pub async fn get_sheet_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: GetSheetParams,
    option: RequestOption,
) -> SDKResult<GetSheetResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.sheet_id.trim(), "工作表ID不能为空");

    // 按 csv 定义使用 v3 查询单个工作表接口
    let api_endpoint =
        SheetsApiV3::GetSheet(spreadsheet_token.to_string(), params.sheet_id.to_string());

    // 创建API请求
    let api_request: ApiRequest<GetSheetResponse> = api_endpoint.to_request();

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "获取工作表信息")
}

/// 更新工作表
///
/// 根据 spreadsheetToken 和 sheet_id 更新工作表的基本信息，如标题或位置。
/// docPath: /document/ukTMukTMukTM/ugjMzUjL4IzM14COyMTN
pub async fn update_sheet(
    config: &Config,
    spreadsheet_token: &str,
    params: UpdateSheetParams,
) -> SDKResult<UpdateSheetResponse> {
    update_sheet_with_options(config, spreadsheet_token, params, RequestOption::default()).await
}

/// 更新工作表（带请求选项）
///
/// 根据 spreadsheetToken 和 sheet_id 更新工作表的基本信息，如标题或位置。
/// docPath: /document/ukTMukTMukTM/ugjMzUjL4IzM14COyMTN
pub async fn update_sheet_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: UpdateSheetParams,
    option: RequestOption,
) -> SDKResult<UpdateSheetResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.sheet_id.trim(), "工作表ID不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::UpdateSheet(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<UpdateSheetResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "更新工作表")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "更新工作表")
}

/// 删除工作表
///
/// 根据 spreadsheetToken 和 sheet_id 删除指定的工作表。
/// docPath: /document/ukTMukTMukTM/uYTMzUjL2EzM14iNxMTN
pub async fn delete_sheet(
    config: &Config,
    spreadsheet_token: &str,
    params: DeleteSheetParams,
) -> SDKResult<DeleteSheetResponse> {
    delete_sheet_with_options(config, spreadsheet_token, params, RequestOption::default()).await
}

/// 删除工作表（带请求选项）
///
/// 根据 spreadsheetToken 和 sheet_id 删除指定的工作表。
/// docPath: /document/ukTMukTMukTM/uYTMzUjL2EzM14iNxMTN
pub async fn delete_sheet_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: DeleteSheetParams,
    option: RequestOption,
) -> SDKResult<DeleteSheetResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.sheet_id.trim(), "工作表ID不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::DeleteSheet(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<DeleteSheetResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "删除工作表")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "删除工作表")
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

    /// 端到端：POST .../sheets_batch_update → AddSheetResponse。
    #[tokio::test]
    async fn test_add_sheet_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/sheets/v2/spreadsheets/token001/sheets_batch_update",
            ))
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
        let resp = add_sheet(
            &config,
            "token001",
            AddSheetParams {
                title: "新工作表".into(),
                index: None,
            },
        )
        .await
        .expect("添加工作表应成功");
        assert!(resp.data.is_none());
        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/sheets/v2/spreadsheets/token001/sheets_batch_update"
        );
    }
}
