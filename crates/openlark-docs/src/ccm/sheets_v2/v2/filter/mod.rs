/// CCM Sheet V2 筛选API 模块
///
/// 筛选功能API实现，包含筛选的增删改查：
/// - create_filter: 创建筛选
/// - get_filter: 获取筛选
/// - update_filter: 更新筛选
/// - delete_filter: 删除筛选
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};

use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};

/// 筛选功能API结构体
#[derive(Debug, Clone)]
pub struct FilterApi {
    config: Config,
}

impl FilterApi {
    /// 创建新的筛选功能API实例
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
    CreateFilterParams, CreateFilterResponse, DeleteFilterParams, DeleteFilterResponse,
    DeleteFilterResult, FilterCondition, FilterInfo, FilterResult, FilterSpec, GetFilterParams,
    GetFilterResponse, UpdateFilterParams, UpdateFilterResponse,
};

impl ApiResponseTrait for CreateFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for GetFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for UpdateFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for DeleteFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建筛选
///
/// 根据 spreadsheetToken 和 range 在指定范围创建筛选。
/// docPath: /document/server-docs/docs/sheets-v3/filter/create-filter
pub async fn create_filter(
    config: &Config,
    spreadsheet_token: &str,
    params: CreateFilterParams,
) -> SDKResult<CreateFilterResponse> {
    create_filter_with_options(config, spreadsheet_token, params, RequestOption::default()).await
}

/// 创建筛选（带选项）
///
/// 根据 spreadsheetToken 和 range 在指定范围创建筛选。
/// docPath: /document/server-docs/docs/sheets-v3/filter/create-filter
pub async fn create_filter_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: CreateFilterParams,
    option: RequestOption,
) -> SDKResult<CreateFilterResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.range, "筛选范围不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::CreateFilter(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<CreateFilterResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "创建筛选")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "创建筛选")
}

/// 获取筛选
///
/// 根据 spreadsheetToken 和 filter_id 获取筛选信息。
/// docPath: /document/server-docs/docs/sheets-v3/filter/get-filter
pub async fn get_filter(
    config: &Config,
    spreadsheet_token: &str,
    params: GetFilterParams,
) -> SDKResult<GetFilterResponse> {
    get_filter_with_options(config, spreadsheet_token, params, RequestOption::default()).await
}

/// 获取筛选（带选项）
///
/// 根据 spreadsheetToken 和 filter_id 获取筛选信息。
/// docPath: /document/server-docs/docs/sheets-v3/filter/get-filter
pub async fn get_filter_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: GetFilterParams,
    option: RequestOption,
) -> SDKResult<GetFilterResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.range, "筛选范围不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::GetFilter(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<GetFilterResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "获取筛选")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "获取筛选")
}

/// 更新筛选
///
/// 根据 spreadsheetToken 和 filter_id 更新筛选条件。
/// docPath: /document/server-docs/docs/sheets-v3/filter/update-filter
pub async fn update_filter(
    config: &Config,
    spreadsheet_token: &str,
    params: UpdateFilterParams,
) -> SDKResult<UpdateFilterResponse> {
    update_filter_with_options(config, spreadsheet_token, params, RequestOption::default()).await
}

/// 更新筛选（带选项）
///
/// 根据 spreadsheetToken 和 filter_id 更新筛选条件。
/// docPath: /document/server-docs/docs/sheets-v3/filter/update-filter
pub async fn update_filter_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: UpdateFilterParams,
    option: RequestOption,
) -> SDKResult<UpdateFilterResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.filter_id.trim(), "筛选ID不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::UpdateFilter(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<UpdateFilterResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "更新筛选")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "更新筛选")
}

/// 删除筛选
///
/// 根据 spreadsheetToken 和 filter_id 删除筛选。
/// docPath: /document/server-docs/docs/sheets-v3/filter/delete-filter
pub async fn delete_filter(
    config: &Config,
    spreadsheet_token: &str,
    params: DeleteFilterParams,
) -> SDKResult<DeleteFilterResponse> {
    delete_filter_with_options(config, spreadsheet_token, params, RequestOption::default()).await
}

/// 删除筛选（带选项）
///
/// 根据 spreadsheetToken 和 filter_id 删除筛选。
/// docPath: /document/server-docs/docs/sheets-v3/filter/delete-filter
pub async fn delete_filter_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: DeleteFilterParams,
    option: RequestOption,
) -> SDKResult<DeleteFilterResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.filter_id.trim(), "筛选ID不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::DeleteFilter(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<DeleteFilterResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "删除筛选")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "删除筛选")
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

    /// 端到端：POST .../filterViews → CreateFilterResponse。
    #[tokio::test]
    async fn test_create_filter_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/sheets/v3/spreadsheets/token001/filterViews",
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
        let resp = create_filter(
            &config,
            "token001",
            CreateFilterParams {
                range: "Sheet1!A1:C10".into(),
                filter_spec: FilterSpec {
                    filter_specs: vec![],
                },
            },
        )
        .await
        .expect("创建筛选应成功");
        assert!(resp.data.is_none());
        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/sheets/v3/spreadsheets/token001/filterViews"
        );
    }
}
