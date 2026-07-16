/// CCM Sheet V2 数据读写API 模块
///
/// 表格数据读写API实现，包含8个核心API：
/// - read_single_range: 读取单个范围
/// - read_multiple_ranges: 读取多个范围
/// - write_single_range: 写入单个范围
/// - batch_write_ranges: 批量写入多个范围
/// - append_values: 追加数据
/// - insert_values: 插入数据
/// - values_prepend: 插入数据到范围之前
/// - values_image: 写入图片
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};

use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};

/// 数据读写API结构体
#[derive(Debug, Clone)]
pub struct DataIOApi {
    config: Config,
}

impl DataIOApi {
    /// 创建新的数据读写API实例
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
    AppendResult, AppendValuesParams, AppendValuesResponse, BatchUpdateResult, BatchWriteData,
    BatchWriteRangesParams, BatchWriteRangesResponse, ImageResult, ImageValue, InsertResult,
    InsertValuesParams, InsertValuesResponse, MultipleRangeData, RangeData,
    ReadMultipleRangesParams, ReadMultipleRangesResponse, ReadSingleRangeParams,
    ReadSingleRangeResponse, UpdateRangeResult, ValuesImageParams, ValuesImageResponse,
    WriteSingleRangeParams, WriteSingleRangeResponse,
};

impl ApiResponseTrait for ReadSingleRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for ReadMultipleRangesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for WriteSingleRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for BatchWriteRangesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for AppendValuesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for InsertValuesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for ValuesImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 读取单个范围
///
/// 根据 spreadsheetToken 和 range 读取表格单个范围的值，返回数据限制为10M。
/// docPath: /document/server-docs/docs/sheets-v3/data-operation/reading-a-single-range
pub async fn read_single_range(
    config: &Config,
    spreadsheet_token: &str,
    params: ReadSingleRangeParams,
) -> SDKResult<ReadSingleRangeResponse> {
    read_single_range_with_options(config, spreadsheet_token, params, RequestOption::default())
        .await
}

/// 读取单个范围（带选项）
///
/// 根据 spreadsheetToken 和 range 读取表格单个范围的值，返回数据限制为10M。
/// docPath: /document/server-docs/docs/sheets-v3/data-operation/reading-a-single-range
pub async fn read_single_range_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: ReadSingleRangeParams,
    option: RequestOption,
) -> SDKResult<ReadSingleRangeResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.value_range, "数据范围不能为空");

    // 使用 csv 对齐后的旧版稳定链路：/open-apis/sheets/v2/spreadsheets/{token}/values/{range}
    let api_endpoint =
        CcmSheetApiOld::ReadSingleRange(spreadsheet_token.to_string(), params.value_range.clone());

    // 创建API请求
    let api_request: ApiRequest<ReadSingleRangeResponse> = api_endpoint
        .to_request()
        .query_opt("valueRenderOption", params.value_render_option.as_ref())
        .query_opt("dateTimeRenderOption", params.date_render_option.as_ref());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "读取单个范围")
}

/// 读取多个范围
///
/// 根据 spreadsheetToken 和 ranges 读取表格多个范围的值，返回数据限制为10M。
/// docPath: /document/server-docs/docs/sheets-v3/data-operation/reading-multiple-ranges
pub async fn read_multiple_ranges(
    config: &Config,
    spreadsheet_token: &str,
    params: ReadMultipleRangesParams,
) -> SDKResult<ReadMultipleRangesResponse> {
    read_multiple_ranges_with_options(config, spreadsheet_token, params, RequestOption::default())
        .await
}

/// 读取多个范围（带选项）
///
/// 根据 spreadsheetToken 和 ranges 读取表格多个范围的值，返回数据限制为10M。
/// docPath: /document/server-docs/docs/sheets-v3/data-operation/reading-multiple-ranges
pub async fn read_multiple_ranges_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: ReadMultipleRangesParams,
    option: RequestOption,
) -> SDKResult<ReadMultipleRangesResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");

    if params.ranges.is_empty() {
        return Err(openlark_core::error::CoreError::validation(
            "ranges",
            "范围列表不能为空",
        ));
    }

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::ReadMultipleRanges(spreadsheet_token.to_string());
    let ranges = params.ranges.join(",");

    // 创建API请求
    let api_request: ApiRequest<ReadMultipleRangesResponse> = api_endpoint
        .to_request()
        .query("ranges", ranges)
        .query_opt("valueRenderOption", params.value_render_option.as_ref())
        .query_opt("dateTimeRenderOption", params.date_render_option.as_ref());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "读取多个范围")
}

/// 写入单个范围
///
/// 根据 spreadsheetToken 和 range 向单个范围写入数据，若范围内有数据，将被更新覆盖。
/// docPath: /document/server-docs/docs/sheets-v3/data-operation/write-data-to-a-single-range
pub async fn write_single_range(
    config: &Config,
    spreadsheet_token: &str,
    params: WriteSingleRangeParams,
) -> SDKResult<WriteSingleRangeResponse> {
    write_single_range_with_options(config, spreadsheet_token, params, RequestOption::default())
        .await
}

/// 写入单个范围（带选项）
///
/// 根据 spreadsheetToken 和 range 向单个范围写入数据，若范围内有数据，将被更新覆盖。
/// docPath: /document/server-docs/docs/sheets-v3/data-operation/write-data-to-a-single-range
pub async fn write_single_range_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: WriteSingleRangeParams,
    option: RequestOption,
) -> SDKResult<WriteSingleRangeResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.range, "数据范围不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::WriteSingleRange(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<WriteSingleRangeResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "写入单个范围")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "写入单个范围")
}

/// 批量写入多个范围
///
/// 根据 spreadsheetToken 和 range 向多个范围写入数据，若范围内有数据，将被更新覆盖。
/// docPath: /document/server-docs/docs/sheets-v3/data-operation/write-data-to-multiple-ranges
pub async fn batch_write_ranges(
    config: &Config,
    spreadsheet_token: &str,
    params: BatchWriteRangesParams,
) -> SDKResult<BatchWriteRangesResponse> {
    batch_write_ranges_with_options(config, spreadsheet_token, params, RequestOption::default())
        .await
}

/// 批量写入多个范围（带选项）
///
/// 根据 spreadsheetToken 和 range 向多个范围写入数据，若范围内有数据，将被更新覆盖。
/// docPath: /document/server-docs/docs/sheets-v3/data-operation/write-data-to-multiple-ranges
pub async fn batch_write_ranges_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: BatchWriteRangesParams,
    option: RequestOption,
) -> SDKResult<BatchWriteRangesResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");

    if params.data.is_empty() {
        return Err(openlark_core::error::CoreError::validation(
            "data",
            "写入数据列表不能为空",
        ));
    }

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::BatchWriteRanges(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<BatchWriteRangesResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "批量写入范围")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "批量写入范围")
}

/// 追加数据
///
/// 根据 spreadsheetToken 和 range 遇到空行则进行覆盖追加或新增行追加数据。
/// docPath: /document/server-docs/docs/sheets-v3/data-operation/append-data
pub async fn append_values(
    config: &Config,
    spreadsheet_token: &str,
    params: AppendValuesParams,
) -> SDKResult<AppendValuesResponse> {
    append_values_with_options(config, spreadsheet_token, params, RequestOption::default()).await
}

/// 追加数据（带选项）
///
/// 根据 spreadsheetToken 和 range 遇到空行则进行覆盖追加或新增行追加数据。
/// docPath: /document/server-docs/docs/sheets-v3/data-operation/append-data
pub async fn append_values_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: AppendValuesParams,
    option: RequestOption,
) -> SDKResult<AppendValuesResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.range, "追加范围不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::AppendValues(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<AppendValuesResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "追加数据")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "追加数据")
}

/// 插入数据
///
/// 根据 spreadsheetToken 和 range 向范围之前增加相应数据的行和相应的数据，相当于数组的插入操作。
/// docPath: /document/server-docs/docs/sheets-v3/data-operation/prepend-data
pub async fn insert_values(
    config: &Config,
    spreadsheet_token: &str,
    params: InsertValuesParams,
) -> SDKResult<InsertValuesResponse> {
    insert_values_with_options(config, spreadsheet_token, params, RequestOption::default()).await
}

/// 插入数据（带选项）
///
/// 根据 spreadsheetToken 和 range 向范围之前增加相应数据的行和相应的数据，相当于数组的插入操作。
/// docPath: /document/server-docs/docs/sheets-v3/data-operation/prepend-data
pub async fn insert_values_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: InsertValuesParams,
    option: RequestOption,
) -> SDKResult<InsertValuesResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.range, "插入范围不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::InsertValues(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<InsertValuesResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "插入数据")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "插入数据")
}

/// 写入图片
///
/// 根据 spreadsheetToken 和 range 向指定范围写入图片，支持批量写入图片数据。
/// docPath: /document/server-docs/docs/sheets-v3/data-operation/writing-images-to-a-range
pub async fn values_image(
    config: &Config,
    spreadsheet_token: &str,
    params: ValuesImageParams,
) -> SDKResult<ValuesImageResponse> {
    values_image_with_options(config, spreadsheet_token, params, RequestOption::default()).await
}

/// 写入图片（带选项）
///
/// 根据 spreadsheetToken 和 range 向指定范围写入图片，支持批量写入图片数据。
/// docPath: /document/server-docs/docs/sheets-v3/data-operation/writing-images-to-a-range
pub async fn values_image_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: ValuesImageParams,
    option: RequestOption,
) -> SDKResult<ValuesImageResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.range, "图片范围不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::ValuesImage(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<ValuesImageResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "写入图片")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "写入图片")
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

    /// 端到端：GET .../values/{range} → ReadSingleRangeResponse。
    #[tokio::test]
    async fn test_read_single_range_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/sheets/v2/spreadsheets/token001/values/A1:B2",
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
        let resp = read_single_range(
            &config,
            "token001",
            ReadSingleRangeParams {
                value_range: "A1:B2".into(),
                value_render_option: None,
                date_render_option: None,
            },
        )
        .await
        .expect("读取单个范围应成功");
        assert!(resp.data.is_none());
        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert!(
            received[0]
                .url
                .path()
                .starts_with("/open-apis/sheets/v2/spreadsheets/token001/values/")
        );
    }
}
