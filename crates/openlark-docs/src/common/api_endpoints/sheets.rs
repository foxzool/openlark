//! Sheets API 端点目录。

use super::CatalogEndpoint;
use openlark_core::api::{ApiRequest, HttpMethod};
use openlark_core::constants::AccessTokenType;

/// CCM Sheet API Old V2 端点枚举
/// 对应 meta.project = ccm_sheet, meta.version = old
#[derive(Debug, Clone, PartialEq)]
pub enum CcmSheetApiOld {
    /// 操作工作表 (第一个)
    OperateSheets(String), // spreadsheet_token
    /// 更新工作表属性 (第二个)
    UpdateSheetProperties(String), // spreadsheet_token
    /// 增加行列
    DimensionRange(String), // spreadsheet_token
    /// 插入行列
    InsertDimensionRange(String), // spreadsheet_token
    /// 更新行列
    DimensionRangeUpdate(String), // spreadsheet_token
    /// 删除行列
    DimensionRangeDelete(String), // spreadsheet_token
    /// 合并单元格
    MergeCells(String), // spreadsheet_token
    /// 拆分单元格
    UnmergeCells(String), // spreadsheet_token
    /// 设置单元格样式
    Style(String), // spreadsheet_token
    /// 批量设置单元格样式
    StylesBatchUpdate(String), // spreadsheet_token
    /// 插入数据
    ValuesPrepend(String), // spreadsheet_token
    /// 追加数据
    ValuesAppend(String), // spreadsheet_token
    /// 写入图片
    ValuesImage(String), // spreadsheet_token
    /// 读取单个范围
    ValuesRange(String, String), // spreadsheet_token, range
    /// 读取多个范围
    ValuesBatchGet(String), // spreadsheet_token
    /// 向单个范围写入数据
    Values(String), // spreadsheet_token
    /// 向多个范围写入数据
    ValuesBatchUpdate(String), // spreadsheet_token
    /// 增加保护范围
    ProtectedDimension(String), // spreadsheet_token
    /// 修改保护范围
    ProtectedRangeBatchUpdate(String), // spreadsheet_token
    /// 获取保护范围
    ProtectedRangeBatchGet(String), // spreadsheet_token
    /// 删除保护范围
    ProtectedRangeBatchDel(String), // spreadsheet_token
    /// 获取表格元数据
    Metainfo(String), // spreadsheet_token
    /// 更新表格属性
    Properties(String), // spreadsheet_token
    /// 导入表格
    Import,
    /// 查询导入结果
    ImportResult,
    /// 获取条件格式
    ConditionFormats(String), // spreadsheet_token
    /// 批量创建条件格式
    ConditionFormatsBatchCreate(String), // spreadsheet_token
    /// 批量删除条件格式
    ConditionFormatsBatchDelete(String), // spreadsheet_token
    /// 批量更新条件格式
    ConditionFormatsBatchUpdate(String), // spreadsheet_token
    /// 获取数据验证规则
    DataValidation(String), // spreadsheet_token
    /// 创建数据验证规则
    DataValidationCreate(String), // spreadsheet_token
    /// 更新下拉列表设置（PUT）
    DataValidationUpdate(String, String), // spreadsheet_token, sheet_id
    /// 删除下拉列表设置（DELETE，按 range 删除）
    DataValidationDelete(String), // spreadsheet_token
    /// 读取单个范围
    ReadSingleRange(String, String), // spreadsheet_token, range
    /// 读取多个范围
    ReadMultipleRanges(String), // spreadsheet_token
    /// 写入单个范围
    WriteSingleRange(String), // spreadsheet_token
    /// 批量写入范围
    BatchWriteRanges(String), // spreadsheet_token
    /// 追加数据
    AppendValues(String), // spreadsheet_token
    /// 插入数据
    InsertValues(String), // spreadsheet_token
    /// 获取电子表格信息
    GetSpreadsheet(String), // spreadsheet_token
    /// 创建电子表格
    CreateSpreadsheet,
    /// 修改电子表格属性
    UpdateSpreadsheet(String), // spreadsheet_token
    /// 操作工作表
    AddSheet(String), // spreadsheet_token
    /// 查询工作表
    GetSheet(String, String), // spreadsheet_token, sheet_id
    /// 更新工作表
    UpdateSheet(String), // spreadsheet_token
    /// 删除工作表
    DeleteSheet(String), // spreadsheet_token
    /// 创建筛选 (V3)
    CreateFilter(String), // spreadsheet_token
    /// 获取筛选 (V3)
    GetFilter(String), // spreadsheet_token
    /// 更新筛选 (V3)
    UpdateFilter(String), // spreadsheet_token
    /// 删除筛选 (V3)
    DeleteFilter(String), // spreadsheet_token
    /// 创建筛选视图 (V3)
    CreateFilterView(String, String), // spreadsheet_token, sheet_id
    /// 更新筛选视图 (V3)
    UpdateFilterView(String, String, String), // spreadsheet_token, sheet_id, filter_view_id
    /// 查询筛选视图 (V3)
    QueryFilterViews(String, String), // spreadsheet_token, sheet_id
    /// 获取筛选视图 (V3)
    GetFilterView(String, String, String), // spreadsheet_token, sheet_id, filter_view_id
    /// 删除筛选视图 (V3)
    DeleteFilterView(String, String, String), // spreadsheet_token, sheet_id, filter_view_id
    /// 创建筛选条件 (V3)
    CreateFilterCondition(String, String, String), // spreadsheet_token, sheet_id, filter_view_id
    /// 更新筛选条件 (V3)
    UpdateFilterCondition(String, String, String, String), // spreadsheet_token, sheet_id, filter_view_id, condition_id
    /// 查询筛选条件 (V3)
    QueryFilterConditions(String, String, String), // spreadsheet_token, sheet_id, filter_view_id
    /// 获取筛选条件 (V3)
    GetFilterCondition(String, String, String, String), // spreadsheet_token, sheet_id, filter_view_id, condition_id
    /// 删除筛选条件 (V3)
    DeleteFilterCondition(String, String, String, String), // spreadsheet_token, sheet_id, filter_view_id, condition_id
    /// 创建浮动图片 (V3)
    CreateFloatImage(String, String), // spreadsheet_token, sheet_id
    /// 更新浮动图片 (V3)
    UpdateFloatImage(String, String, String), // spreadsheet_token, sheet_id, float_image_id
    /// 获取浮动图片 (V3)
    GetFloatImage(String, String, String), // spreadsheet_token, sheet_id, float_image_id
    /// 查询浮动图片 (V3)
    QueryFloatImages(String, String), // spreadsheet_token, sheet_id
    /// 删除浮动图片 (V3)
    DeleteFloatImage(String, String, String), // spreadsheet_token, sheet_id, float_image_id
    /// 删除范围 (V3)
    DeleteRange(String), // spreadsheet_token
    /// 插入维度 (V3)
    InsertDimension(String), // spreadsheet_token
    /// 移动维度 (V3)
    MoveDimension(String), // spreadsheet_token
    /// 替换范围 (V3)
    ReplaceRange(String), // spreadsheet_token
    /// 查找替换 (V3)
    FindReplace(String), // spreadsheet_token
}

impl CcmSheetApiOld {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            CcmSheetApiOld::OperateSheets(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/sheets_batch_update")
            }
            CcmSheetApiOld::UpdateSheetProperties(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/sheets_batch_update")
            }
            CcmSheetApiOld::Style(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/style")
            }
            CcmSheetApiOld::StylesBatchUpdate(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/styles_batch_update")
            }
            CcmSheetApiOld::ValuesPrepend(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_prepend")
            }
            CcmSheetApiOld::ValuesAppend(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_append")
            }
            CcmSheetApiOld::ValuesImage(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_image")
            }
            CcmSheetApiOld::ValuesRange(spreadsheet_token, range) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values/{range}")
            }
            CcmSheetApiOld::ValuesBatchGet(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_batch_get")
            }
            CcmSheetApiOld::Values(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values")
            }
            CcmSheetApiOld::ValuesBatchUpdate(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_batch_update")
            }
            CcmSheetApiOld::DimensionRange(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/dimension_range")
            }
            CcmSheetApiOld::InsertDimensionRange(spreadsheet_token) => {
                format!(
                    "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/insert_dimension_range"
                )
            }
            CcmSheetApiOld::DimensionRangeUpdate(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/dimension_range")
            }
            CcmSheetApiOld::DimensionRangeDelete(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/dimension_range")
            }
            CcmSheetApiOld::MergeCells(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/merge_cells")
            }
            CcmSheetApiOld::UnmergeCells(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/unmerge_cells")
            }
            CcmSheetApiOld::ProtectedDimension(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/protected_dimension")
            }
            CcmSheetApiOld::ProtectedRangeBatchUpdate(spreadsheet_token) => {
                format!(
                    "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/protected_range_batch_update"
                )
            }
            CcmSheetApiOld::ProtectedRangeBatchGet(spreadsheet_token) => {
                format!(
                    "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/protected_range_batch_get"
                )
            }
            CcmSheetApiOld::ProtectedRangeBatchDel(spreadsheet_token) => {
                format!(
                    "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/protected_range_batch_del"
                )
            }
            CcmSheetApiOld::Metainfo(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/metainfo")
            }
            CcmSheetApiOld::Properties(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/properties")
            }
            CcmSheetApiOld::Import => "/open-apis/sheets/v2/import".to_string(),
            CcmSheetApiOld::ImportResult => "/open-apis/sheets/v2/import/result".to_string(),
            CcmSheetApiOld::ConditionFormats(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/condition_formats")
            }
            CcmSheetApiOld::ConditionFormatsBatchCreate(spreadsheet_token) => {
                format!(
                    "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/condition_formats/batch_create"
                )
            }
            CcmSheetApiOld::ConditionFormatsBatchDelete(spreadsheet_token) => {
                format!(
                    "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/condition_formats/batch_delete"
                )
            }
            CcmSheetApiOld::ConditionFormatsBatchUpdate(spreadsheet_token) => {
                format!(
                    "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/condition_formats/batch_update"
                )
            }
            CcmSheetApiOld::DataValidation(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/dataValidation")
            }
            CcmSheetApiOld::DataValidationCreate(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/dataValidation")
            }
            CcmSheetApiOld::DataValidationUpdate(spreadsheet_token, sheet_id) => {
                format!(
                    "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/dataValidation/{sheet_id}"
                )
            }
            CcmSheetApiOld::DataValidationDelete(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/dataValidation")
            }
            CcmSheetApiOld::ReadSingleRange(spreadsheet_token, range) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values/{range}")
            }
            CcmSheetApiOld::ReadMultipleRanges(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_batch_get")
            }
            CcmSheetApiOld::WriteSingleRange(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values")
            }
            CcmSheetApiOld::BatchWriteRanges(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_batch_update")
            }
            CcmSheetApiOld::AppendValues(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_append")
            }
            CcmSheetApiOld::InsertValues(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_prepend")
            }
            CcmSheetApiOld::GetSpreadsheet(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}")
            }
            CcmSheetApiOld::CreateSpreadsheet => "/open-apis/sheets/v3/spreadsheets".to_string(),
            CcmSheetApiOld::UpdateSpreadsheet(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}")
            }
            CcmSheetApiOld::AddSheet(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/sheets_batch_update")
            }
            CcmSheetApiOld::GetSheet(spreadsheet_token, sheet_id) => {
                format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}")
            }
            CcmSheetApiOld::UpdateSheet(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/sheets_batch_update")
            }
            CcmSheetApiOld::DeleteSheet(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/sheets_batch_update")
            }
            CcmSheetApiOld::CreateFilter(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/filterViews")
            }
            CcmSheetApiOld::GetFilter(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/filterViews/query")
            }
            CcmSheetApiOld::UpdateFilter(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/filterViews")
            }
            CcmSheetApiOld::DeleteFilter(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/filterViews")
            }
            CcmSheetApiOld::CreateFilterView(spreadsheet_token, sheet_id) => {
                format!(
                    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views"
                )
            }
            CcmSheetApiOld::UpdateFilterView(spreadsheet_token, sheet_id, filter_view_id) => {
                format!(
                    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}"
                )
            }
            CcmSheetApiOld::QueryFilterViews(spreadsheet_token, sheet_id) => {
                format!(
                    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/query"
                )
            }
            CcmSheetApiOld::GetFilterView(spreadsheet_token, sheet_id, filter_view_id) => {
                format!(
                    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}"
                )
            }
            CcmSheetApiOld::DeleteFilterView(spreadsheet_token, sheet_id, filter_view_id) => {
                format!(
                    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}"
                )
            }
            CcmSheetApiOld::CreateFilterCondition(spreadsheet_token, sheet_id, filter_view_id) => {
                format!(
                    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}/conditions"
                )
            }
            CcmSheetApiOld::UpdateFilterCondition(
                spreadsheet_token,
                sheet_id,
                filter_view_id,
                condition_id,
            ) => {
                format!(
                    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}/conditions/{condition_id}"
                )
            }
            CcmSheetApiOld::QueryFilterConditions(spreadsheet_token, sheet_id, filter_view_id) => {
                format!(
                    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}/conditions/query"
                )
            }
            CcmSheetApiOld::GetFilterCondition(
                spreadsheet_token,
                sheet_id,
                filter_view_id,
                condition_id,
            ) => {
                format!(
                    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}/conditions/{condition_id}"
                )
            }
            CcmSheetApiOld::DeleteFilterCondition(
                spreadsheet_token,
                sheet_id,
                filter_view_id,
                condition_id,
            ) => {
                format!(
                    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}/conditions/{condition_id}"
                )
            }
            CcmSheetApiOld::CreateFloatImage(spreadsheet_token, sheet_id) => {
                format!(
                    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/float_images"
                )
            }
            CcmSheetApiOld::UpdateFloatImage(spreadsheet_token, sheet_id, float_image_id) => {
                format!(
                    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/float_images/{float_image_id}"
                )
            }
            CcmSheetApiOld::GetFloatImage(spreadsheet_token, sheet_id, float_image_id) => {
                format!(
                    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/float_images/{float_image_id}"
                )
            }
            CcmSheetApiOld::QueryFloatImages(spreadsheet_token, sheet_id) => {
                format!(
                    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/float_images/query"
                )
            }
            CcmSheetApiOld::DeleteFloatImage(spreadsheet_token, sheet_id, float_image_id) => {
                format!(
                    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/float_images/{float_image_id}"
                )
            }
            CcmSheetApiOld::DeleteRange(spreadsheet_token) => {
                format!(
                    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/dimensionRange/delete"
                )
            }
            CcmSheetApiOld::InsertDimension(spreadsheet_token) => {
                format!(
                    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/dimensionRange/insert"
                )
            }
            CcmSheetApiOld::MoveDimension(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/dimensionRange/move")
            }
            CcmSheetApiOld::ReplaceRange(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/values/batchReplace")
            }
            CcmSheetApiOld::FindReplace(spreadsheet_token) => {
                format!(
                    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/values/batchFindReplace"
                )
            }
        }
    }

    /// 返回配置了稳定请求语义的请求。
    pub fn to_request<R>(&self) -> ApiRequest<R> {
        <Self as CatalogEndpoint>::to_request(self)
    }
}

impl CatalogEndpoint for CcmSheetApiOld {
    fn to_url(&self) -> String {
        CcmSheetApiOld::to_url(self)
    }

    /// 返回端点的 HTTP 方法，并保持旧版 leaf 的现有请求语义。
    fn method(&self) -> HttpMethod {
        match self {
            Self::ValuesRange(_, _)
            | Self::ValuesBatchGet(_)
            | Self::ProtectedRangeBatchGet(_)
            | Self::Metainfo(_)
            | Self::ImportResult
            | Self::ConditionFormats(_)
            | Self::DataValidation(_)
            | Self::ReadSingleRange(_, _)
            | Self::ReadMultipleRanges(_)
            | Self::GetSpreadsheet(_)
            | Self::GetSheet(_, _)
            | Self::QueryFilterViews(_, _)
            | Self::GetFilterView(_, _, _)
            | Self::QueryFilterConditions(_, _, _)
            | Self::GetFilterCondition(_, _, _, _)
            | Self::QueryFloatImages(_, _) => HttpMethod::Get,
            Self::Values(_)
            | Self::WriteSingleRange(_)
            | Self::DataValidationUpdate(_, _)
            | Self::DimensionRangeUpdate(_)
            | Self::Properties(_) => HttpMethod::Put,
            Self::UpdateSpreadsheet(_) | Self::UpdateFilterView(_, _, _) => HttpMethod::Patch,
            Self::DimensionRangeDelete(_)
            | Self::DataValidationDelete(_)
            | Self::DeleteFilterView(_, _, _)
            | Self::DeleteFilterCondition(_, _, _, _) => HttpMethod::Delete,
            Self::OperateSheets(_)
            | Self::UpdateSheetProperties(_)
            | Self::DimensionRange(_)
            | Self::InsertDimensionRange(_)
            | Self::MergeCells(_)
            | Self::UnmergeCells(_)
            | Self::Style(_)
            | Self::StylesBatchUpdate(_)
            | Self::ValuesPrepend(_)
            | Self::ValuesAppend(_)
            | Self::ValuesImage(_)
            | Self::ValuesBatchUpdate(_)
            | Self::ProtectedDimension(_)
            | Self::ProtectedRangeBatchUpdate(_)
            | Self::ProtectedRangeBatchDel(_)
            | Self::Import
            | Self::ConditionFormatsBatchCreate(_)
            | Self::ConditionFormatsBatchDelete(_)
            | Self::ConditionFormatsBatchUpdate(_)
            | Self::DataValidationCreate(_)
            | Self::BatchWriteRanges(_)
            | Self::AppendValues(_)
            | Self::InsertValues(_)
            | Self::CreateSpreadsheet
            | Self::AddSheet(_)
            | Self::UpdateSheet(_)
            | Self::DeleteSheet(_)
            | Self::CreateFilter(_)
            | Self::GetFilter(_)
            | Self::UpdateFilter(_)
            | Self::DeleteFilter(_)
            | Self::CreateFilterView(_, _)
            | Self::CreateFilterCondition(_, _, _)
            | Self::UpdateFilterCondition(_, _, _, _)
            | Self::CreateFloatImage(_, _)
            | Self::UpdateFloatImage(_, _, _)
            | Self::GetFloatImage(_, _, _)
            | Self::DeleteFloatImage(_, _, _)
            | Self::DeleteRange(_)
            | Self::InsertDimension(_)
            | Self::MoveDimension(_)
            | Self::ReplaceRange(_)
            | Self::FindReplace(_) => HttpMethod::Post,
        }
    }
}

/// Sheets API v3 端点枚举
/// 对应 meta.project = sheets, meta.version = v3
#[derive(Debug, Clone, PartialEq)]
pub enum SheetsApiV3 {
    // =====================
    // spreadsheet
    // =====================
    /// 创建电子表格
    CreateSpreadsheet,
    /// 获取电子表格信息
    GetSpreadsheet(String), // spreadsheet_token
    /// 修改电子表格属性
    PatchSpreadsheet(String), // spreadsheet_token

    // =====================
    // spreadsheet.sheet
    // =====================
    /// 获取工作表列表
    QuerySheets(String), // spreadsheet_token
    /// 查询工作表
    GetSheet(String, String), // (spreadsheet_token, sheet_id)
    /// 移动行列
    MoveDimension(String, String), // (spreadsheet_token, sheet_id)
    /// 查找单元格
    FindCells(String, String), // (spreadsheet_token, sheet_id)
    /// 替换单元格
    ReplaceCells(String, String), // (spreadsheet_token, sheet_id)

    // =====================
    // spreadsheet.sheet.filter
    // =====================
    /// 创建筛选
    CreateFilter(String, String), // (spreadsheet_token, sheet_id)
    /// 更新筛选
    UpdateFilter(String, String), // (spreadsheet_token, sheet_id)
    /// 获取筛选
    GetFilter(String, String), // (spreadsheet_token, sheet_id)
    /// 删除筛选
    DeleteFilter(String, String), // (spreadsheet_token, sheet_id)

    // =====================
    // spreadsheet.sheet.filter_view
    // =====================
    /// 创建筛选视图
    CreateFilterView(String, String), // (spreadsheet_token, sheet_id)
    /// 查询筛选视图
    QueryFilterViews(String, String), // (spreadsheet_token, sheet_id)
    /// 获取筛选视图
    GetFilterView(String, String, String), // (spreadsheet_token, sheet_id, filter_view_id)
    /// 更新筛选视图
    PatchFilterView(String, String, String), // (spreadsheet_token, sheet_id, filter_view_id)
    /// 删除筛选视图
    DeleteFilterView(String, String, String), // (spreadsheet_token, sheet_id, filter_view_id)

    // =====================
    // spreadsheet.sheet.filter_view.condition
    // =====================
    /// 创建筛选条件
    CreateFilterCondition(String, String, String), // (spreadsheet_token, sheet_id, filter_view_id)
    /// 查询筛选条件
    QueryFilterConditions(String, String, String), // (spreadsheet_token, sheet_id, filter_view_id)
    /// 获取筛选条件
    GetFilterCondition(String, String, String, String), // (spreadsheet_token, sheet_id, filter_view_id, condition_id)
    /// 更新筛选条件
    UpdateFilterCondition(String, String, String, String), // (spreadsheet_token, sheet_id, filter_view_id, condition_id)
    /// 删除筛选条件
    DeleteFilterCondition(String, String, String, String), // (spreadsheet_token, sheet_id, filter_view_id, condition_id)

    // =====================
    // spreadsheet.sheet.float_image
    // =====================
    /// 创建浮动图片
    CreateFloatImage(String, String), // (spreadsheet_token, sheet_id)
    /// 查询浮动图片
    QueryFloatImages(String, String), // (spreadsheet_token, sheet_id)
    /// 获取浮动图片
    GetFloatImage(String, String, String), // (spreadsheet_token, sheet_id, float_image_id)
    /// 更新浮动图片
    PatchFloatImage(String, String, String), // (spreadsheet_token, sheet_id, float_image_id)
    /// 删除浮动图片
    DeleteFloatImage(String, String, String), // (spreadsheet_token, sheet_id, float_image_id)
}

impl SheetsApiV3 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            SheetsApiV3::CreateSpreadsheet => "/open-apis/sheets/v3/spreadsheets".to_string(),
            SheetsApiV3::GetSpreadsheet(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}")
            }
            SheetsApiV3::PatchSpreadsheet(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}")
            }

            SheetsApiV3::QuerySheets(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/query")
            }
            SheetsApiV3::GetSheet(spreadsheet_token, sheet_id) => {
                format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}")
            }
            SheetsApiV3::MoveDimension(spreadsheet_token, sheet_id) => format!(
                "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/move_dimension"
            ),
            SheetsApiV3::FindCells(spreadsheet_token, sheet_id) => format!(
                "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/find"
            ),
            SheetsApiV3::ReplaceCells(spreadsheet_token, sheet_id) => format!(
                "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/replace"
            ),

            SheetsApiV3::CreateFilter(spreadsheet_token, sheet_id)
            | SheetsApiV3::UpdateFilter(spreadsheet_token, sheet_id)
            | SheetsApiV3::GetFilter(spreadsheet_token, sheet_id)
            | SheetsApiV3::DeleteFilter(spreadsheet_token, sheet_id) => format!(
                "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter"
            ),

            SheetsApiV3::CreateFilterView(spreadsheet_token, sheet_id) => format!(
                "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views"
            ),
            SheetsApiV3::QueryFilterViews(spreadsheet_token, sheet_id) => format!(
                "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/query"
            ),
            SheetsApiV3::GetFilterView(spreadsheet_token, sheet_id, filter_view_id)
            | SheetsApiV3::PatchFilterView(spreadsheet_token, sheet_id, filter_view_id)
            | SheetsApiV3::DeleteFilterView(spreadsheet_token, sheet_id, filter_view_id) => {
                format!(
                    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}"
                )
            }

            SheetsApiV3::CreateFilterCondition(spreadsheet_token, sheet_id, filter_view_id) => {
                format!(
                    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}/conditions"
                )
            }
            SheetsApiV3::QueryFilterConditions(spreadsheet_token, sheet_id, filter_view_id) => {
                format!(
                    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}/conditions/query"
                )
            }
            SheetsApiV3::GetFilterCondition(
                spreadsheet_token,
                sheet_id,
                filter_view_id,
                condition_id,
            )
            | SheetsApiV3::UpdateFilterCondition(
                spreadsheet_token,
                sheet_id,
                filter_view_id,
                condition_id,
            )
            | SheetsApiV3::DeleteFilterCondition(
                spreadsheet_token,
                sheet_id,
                filter_view_id,
                condition_id,
            ) => format!(
                "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}/conditions/{condition_id}"
            ),

            SheetsApiV3::CreateFloatImage(spreadsheet_token, sheet_id) => format!(
                "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/float_images"
            ),
            SheetsApiV3::QueryFloatImages(spreadsheet_token, sheet_id) => format!(
                "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/float_images/query"
            ),
            SheetsApiV3::GetFloatImage(spreadsheet_token, sheet_id, float_image_id)
            | SheetsApiV3::PatchFloatImage(spreadsheet_token, sheet_id, float_image_id)
            | SheetsApiV3::DeleteFloatImage(spreadsheet_token, sheet_id, float_image_id) => {
                format!(
                    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/float_images/{float_image_id}"
                )
            }
        }
    }

    /// 返回配置了稳定请求语义的请求。
    pub fn to_request<R>(&self) -> ApiRequest<R> {
        <Self as CatalogEndpoint>::to_request(self)
    }
}

impl CatalogEndpoint for SheetsApiV3 {
    fn to_url(&self) -> String {
        SheetsApiV3::to_url(self)
    }

    fn method(&self) -> HttpMethod {
        match self {
            Self::GetSpreadsheet(_)
            | Self::QuerySheets(_)
            | Self::GetSheet(_, _)
            | Self::GetFilter(_, _)
            | Self::QueryFilterViews(_, _)
            | Self::GetFilterView(_, _, _)
            | Self::QueryFilterConditions(_, _, _)
            | Self::GetFilterCondition(_, _, _, _)
            | Self::QueryFloatImages(_, _)
            | Self::GetFloatImage(_, _, _) => HttpMethod::Get,
            Self::CreateSpreadsheet
            | Self::CreateFilter(_, _)
            | Self::CreateFilterView(_, _)
            | Self::CreateFilterCondition(_, _, _)
            | Self::CreateFloatImage(_, _) => HttpMethod::Post,
            Self::MoveDimension(_, _) | Self::FindCells(_, _) | Self::ReplaceCells(_, _) => {
                HttpMethod::Post
            }
            Self::PatchSpreadsheet(_)
            | Self::PatchFilterView(_, _, _)
            | Self::PatchFloatImage(_, _, _) => HttpMethod::Patch,
            Self::UpdateFilter(_, _) | Self::UpdateFilterCondition(_, _, _, _) => HttpMethod::Put,
            Self::DeleteFilter(_, _)
            | Self::DeleteFilterView(_, _, _)
            | Self::DeleteFilterCondition(_, _, _, _)
            | Self::DeleteFloatImage(_, _, _) => HttpMethod::Delete,
        }
    }

    fn supported_access_token_types(&self) -> Option<Vec<AccessTokenType>> {
        Some(vec![AccessTokenType::User, AccessTokenType::Tenant])
    }
}
