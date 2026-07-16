//! API端点定义（类型安全枚举系统）
//!
//! 本模块提供基于枚举的 API 端点定义，用于生产代码中的类型安全调用。
//!
//! # 使用场景
//!
//! ## 生产代码（推荐）
//! 使用枚举端点获得编译时类型检查和动态 URL 生成能力：
//! ```rust
//! use openlark_docs::common::api_endpoints::BitableApiV1;
//!
//! let app_token = "app_token".to_string();
//! let table_id = "table_id".to_string();
//! let endpoint = BitableApiV1::RecordCreate(app_token, table_id);
//! let url = endpoint.to_url(); // 类型安全，动态生成
//! assert!(url.contains("/open-apis/bitable/v1/"));
//! ```
//!
//! # 特性
//! - ✅ **类型安全**：编译时验证参数
//! - ✅ **动态生成**：支持参数化 URL
//! - ✅ **易于维护**：集中管理端点定义
//! - ✅ **避免错误**：消除字符串拼接错误
//!
//! # 与常量端点系统的关系
//!
//! 本模块与 `endpoints/mod.rs` 中的常量端点系统配合使用：
//! - **枚举端点**：用于生产代码（推荐）
//! - **常量端点**：用于测试和文档示例
//!
//! 不建议混合使用两个系统，应根据场景选择合适的端点方式。

use openlark_core::api::{ApiRequest, HttpMethod};
use openlark_core::constants::AccessTokenType;

/// 端点 catalog 的通用语义接口（#424 / #438）。
/// 允许 to_request 等逻辑共享，减少重复。
pub trait CatalogEndpoint {
    /// 返回端点 URL。
    fn to_url(&self) -> String;

    /// 返回 HTTP 方法。
    fn method(&self) -> HttpMethod;

    /// 稳定的访问令牌要求（默认 None）。
    fn supported_access_token_types(&self) -> Option<Vec<AccessTokenType>> {
        None
    }

    /// 构建带正确方法的请求。
    fn to_request<R>(&self) -> ApiRequest<R> {
        self.to_request_with_url(self.to_url())
    }

    /// 使用调用方补充了动态 query 的 URL 构建请求，同时保留 catalog 的 method/auth 语义。
    fn to_request_with_url<R>(&self, url: impl Into<String>) -> ApiRequest<R> {
        let url = url.into();
        let mut req = match self.method() {
            HttpMethod::Get => ApiRequest::get(url),
            HttpMethod::Post => ApiRequest::post(url),
            HttpMethod::Put => ApiRequest::put(url),
            HttpMethod::Delete => ApiRequest::delete(url),
            HttpMethod::Patch => ApiRequest::patch(url),
            _ => ApiRequest::get(self.to_url()),
        };
        if let Some(tokens) = self.supported_access_token_types() {
            req = req.with_supported_access_token_types(tokens);
        }
        req
    }
}

pub mod base;
pub use base::BaseApiV2;

pub mod bitable;
pub use bitable::BitableApiV1;

/// Minutes API V1 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum MinutesApiV1 {
    /// 获取妙记信息
    Get(String),
    /// 订阅妙记变更事件
    Subscription,
    /// 取消订阅妙记变更事件
    Unsubscription,
    /// 下载妙记音视频文件
    MediaGet(String),
    /// 导出妙记文字记录
    TranscriptGet(String),
    /// 获取妙记统计数据
    StatisticsGet(String),
}

impl MinutesApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            MinutesApiV1::Get(minute_token) => {
                format!("/open-apis/minutes/v1/minutes/{minute_token}")
            }
            MinutesApiV1::Subscription => "/open-apis/minutes/v1/minutes/subscription".to_string(),
            MinutesApiV1::Unsubscription => {
                "/open-apis/minutes/v1/minutes/unsubscription".to_string()
            }
            MinutesApiV1::MediaGet(minute_token) => {
                format!("/open-apis/minutes/v1/minutes/{minute_token}/media")
            }
            MinutesApiV1::TranscriptGet(minute_token) => {
                format!("/open-apis/minutes/v1/minutes/{minute_token}/transcript")
            }
            MinutesApiV1::StatisticsGet(minute_token) => {
                format!("/open-apis/minutes/v1/minutes/{minute_token}/statistics")
            }
        }
    }
}

/// Wiki API V1 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum WikiApiV1 {
    /// 搜索Wiki
    NodeSearch,
}

impl WikiApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            WikiApiV1::NodeSearch => "/open-apis/wiki/v1/nodes/search".to_string(),
        }
    }
}

pub mod docs;
pub use docs::DocsApiV1;

pub mod docx;
pub use docx::DocxApiV1;

/// Wiki API V2 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum WikiApiV2 {
    /// 获取知识空间列表
    SpaceList,
    /// 获取知识空间信息
    SpaceGet(String),
    /// 创建知识空间
    SpaceCreate,
    /// 更新知识空间设置
    SpaceSettingUpdate(String),
    /// 获取知识空间节点信息
    SpaceGetNode,
    /// 获取知识空间子节点列表
    SpaceNodeList(String),
    /// 创建知识空间节点
    SpaceNodeCreate(String),
    /// 获取知识空间成员列表
    SpaceMemberList(String),
    /// 添加知识空间成员
    SpaceMemberCreate(String),
    /// 删除知识空间成员
    SpaceMemberDelete(String, String), // space_id, member_id
    /// 移动知识空间节点
    SpaceNodeMove(String, String),
    /// 更新知识空间节点标题
    SpaceNodeUpdateTitle(String, String),
    /// 创建知识空间节点副本
    SpaceNodeCopy(String, String),
    /// 移动云空间文档至知识空间
    SpaceNodeMoveDocsToWiki(String),
    /// 获取任务结果
    TaskGet(String),
}

impl WikiApiV2 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            WikiApiV2::SpaceList => "/open-apis/wiki/v2/spaces".to_string(),
            WikiApiV2::SpaceGet(space_id) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}")
            }
            WikiApiV2::SpaceCreate => "/open-apis/wiki/v2/spaces".to_string(),
            WikiApiV2::SpaceSettingUpdate(space_id) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/setting")
            }
            WikiApiV2::SpaceGetNode => "/open-apis/wiki/v2/spaces/get_node".to_string(),
            WikiApiV2::SpaceNodeList(space_id) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/nodes")
            }
            WikiApiV2::SpaceNodeCreate(space_id) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/nodes")
            }
            WikiApiV2::SpaceMemberList(space_id) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/members")
            }
            WikiApiV2::SpaceMemberCreate(space_id) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/members")
            }
            WikiApiV2::SpaceMemberDelete(space_id, member_id) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/members/{member_id}")
            }
            WikiApiV2::SpaceNodeMove(space_id, node_token) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/nodes/{node_token}/move")
            }
            WikiApiV2::SpaceNodeUpdateTitle(space_id, node_token) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/nodes/{node_token}/update_title")
            }
            WikiApiV2::SpaceNodeCopy(space_id, node_token) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/nodes/{node_token}/copy")
            }
            WikiApiV2::SpaceNodeMoveDocsToWiki(space_id) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/nodes/move_docs_to_wiki")
            }
            WikiApiV2::TaskGet(task_id) => {
                format!("/open-apis/wiki/v2/tasks/{task_id}")
            }
        }
    }
}

/// CCM Doc API Old V1 端点枚举
/// 对应 meta.project = ccm_doc, meta.version = old
#[derive(Debug, Clone, PartialEq)]
pub enum CcmDocApiOld {
    /// 创建旧版文档
    Create,
    /// 获取旧版文档元信息
    Meta(String), // doc_token
    /// 获取旧版文档中的电子表格元数据
    SheetMeta(String), // doc_token
    /// 获取旧版文档纯文本内容
    RawContent(String), // doc_token
    /// 获取旧版文档富文本内容
    Content(String), // doc_token
    /// 编辑旧版文档内容
    BatchUpdate(String), // doc_token
}

impl CcmDocApiOld {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            CcmDocApiOld::Create => "/open-apis/doc/v2/create".to_string(),
            CcmDocApiOld::Meta(doc_token) => {
                format!("/open-apis/doc/v2/meta/{doc_token}")
            }
            CcmDocApiOld::SheetMeta(doc_token) => {
                format!("/open-apis/doc/v2/{doc_token}/sheet_meta")
            }
            CcmDocApiOld::RawContent(doc_token) => {
                format!("/open-apis/doc/v2/{doc_token}/raw_content")
            }
            CcmDocApiOld::Content(doc_token) => {
                format!("/open-apis/doc/v2/{doc_token}/content")
            }
            CcmDocApiOld::BatchUpdate(doc_token) => {
                format!("/open-apis/doc/v2/{doc_token}/batch_update")
            }
        }
    }

    /// 返回配置了稳定请求语义的请求。
    pub fn to_request<R>(&self) -> ApiRequest<R> {
        <Self as CatalogEndpoint>::to_request(self)
    }

    /// 返回端点的 HTTP 方法。
    pub fn method(&self) -> HttpMethod {
        match self {
            Self::Create | Self::BatchUpdate(_) => HttpMethod::Post,
            Self::Meta(_) | Self::SheetMeta(_) | Self::RawContent(_) | Self::Content(_) => {
                HttpMethod::Get
            }
        }
    }
}

impl CatalogEndpoint for CcmDocApiOld {
    fn to_url(&self) -> String {
        CcmDocApiOld::to_url(self)
    }

    fn method(&self) -> HttpMethod {
        CcmDocApiOld::method(self)
    }

    fn supported_access_token_types(&self) -> Option<Vec<AccessTokenType>> {
        Some(vec![AccessTokenType::User, AccessTokenType::Tenant])
    }
}

/// CCM Docs API Old V1 端点枚举
/// 对应 meta.project = ccm_docs, meta.version = old
#[derive(Debug, Clone, PartialEq)]
pub enum CcmDocsApiOld {
    /// 搜索云文档
    SearchObject,
    /// 获取元数据
    Meta,
}

impl CcmDocsApiOld {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            CcmDocsApiOld::SearchObject => "/open-apis/suite/docs-api/search/object".to_string(),
            CcmDocsApiOld::Meta => "/open-apis/suite/docs-api/meta".to_string(),
        }
    }

    /// 返回配置了稳定请求语义的请求。
    pub fn to_request<R>(&self) -> ApiRequest<R> {
        <Self as CatalogEndpoint>::to_request(self)
    }

    /// 返回端点的 HTTP 方法。
    pub fn method(&self) -> HttpMethod {
        match self {
            Self::SearchObject => HttpMethod::Post,
            Self::Meta => HttpMethod::Get,
        }
    }
}

impl CatalogEndpoint for CcmDocsApiOld {
    fn to_url(&self) -> String {
        CcmDocsApiOld::to_url(self)
    }

    fn method(&self) -> HttpMethod {
        CcmDocsApiOld::method(self)
    }

    fn supported_access_token_types(&self) -> Option<Vec<AccessTokenType>> {
        Some(vec![AccessTokenType::User, AccessTokenType::Tenant])
    }
}

pub mod drive;
pub use drive::{
    CcmDriveExplorerApi, CcmDriveExplorerApiOld, DriveApi, PermissionApi, PermissionApiOld,
};
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
}

/// Wiki API 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum WikiApi {
    // Space APIs
    /// 获取知识空间列表
    ListSpaces,
    /// 获取知识空间信息
    GetSpace,
    /// 创建知识空间
    CreateSpace,

    // Space Member APIs
    /// 获取知识空间成员列表
    ListSpaceMembers(String), // space_id
    /// 添加知识空间成员
    CreateSpaceMember(String), // space_id
    /// 删除知识空间成员
    DeleteSpaceMember(String, String), // space_id, member_id

    // Space Setting APIs
    /// 更新知识空间设置
    UpdateSpaceSetting(String), // space_id

    // Space Node APIs
    /// 创建知识空间节点
    CreateSpaceNode(String), // space_id
    /// 获取知识空间节点信息
    GetSpaceNode,
    /// 获取知识空间子节点列表
    ListSpaceNodes,
    /// 移动知识空间节点
    MoveSpaceNode(String, String), // space_id, node_token
    /// 更新知识空间节点标题
    UpdateSpaceNodeTitle(String, String), // space_id, node_token
    /// 创建知识空间节点副本
    CopySpaceNode(String, String), // space_id, node_token
    /// 移动云空间文档至知识空间
    MoveDocsToWiki(String), // space_id

    // Task APIs
    /// 获取任务结果
    GetTask(String), // task_id

    // Node Search API (V1)
    /// 搜索Wiki节点
    SearchNodes,
}

impl WikiApi {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            // Space APIs
            WikiApi::ListSpaces => "/open-apis/wiki/v2/spaces".to_string(),
            WikiApi::GetSpace => "/open-apis/wiki/v2/spaces/get_node".to_string(),
            WikiApi::CreateSpace => "/open-apis/wiki/v2/spaces".to_string(),

            // Space Member APIs
            WikiApi::ListSpaceMembers(space_id) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/members")
            }
            WikiApi::CreateSpaceMember(space_id) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/members")
            }
            WikiApi::DeleteSpaceMember(space_id, member_id) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/members/{member_id}")
            }

            // Space Setting APIs
            WikiApi::UpdateSpaceSetting(space_id) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/setting")
            }

            // Space Node APIs
            WikiApi::CreateSpaceNode(space_id) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/nodes")
            }
            WikiApi::GetSpaceNode => "/open-apis/wiki/v2/spaces/get_node".to_string(),
            WikiApi::ListSpaceNodes => "/open-apis/wiki/v2/space.node/list".to_string(),
            WikiApi::MoveSpaceNode(space_id, node_token) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/nodes/{node_token}/move")
            }
            WikiApi::UpdateSpaceNodeTitle(space_id, node_token) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/nodes/{node_token}/update_title")
            }
            WikiApi::CopySpaceNode(space_id, node_token) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/nodes/{node_token}/copy")
            }
            WikiApi::MoveDocsToWiki(space_id) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/nodes/move_docs_to_wiki")
            }

            // Task APIs
            WikiApi::GetTask(task_id) => {
                format!("/open-apis/wiki/v2/tasks/{task_id}")
            }

            // Node Search API (V1)
            WikiApi::SearchNodes => "/open-apis/wiki/v1/nodes/search".to_string(),
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
}

// Sheets API v3 端点
/// 电子表格 v3 API 基础路径。
pub const SHEETS_API_V3: &str = "/open-apis/sheets/v3";

// ============================================================================
// Baike API v1 端点定义
// ============================================================================

/// Baike知识库 API v1 端点
#[derive(Debug, Clone, PartialEq)]
pub enum BaikeApiV1 {
    /// 草稿管理
    DraftCreate,
    /// 更新草稿（参数：draft_id）
    DraftUpdate(String), // draft_id

    /// 词条管理
    EntityCreate,
    /// 更新词条（参数：entity_id）
    EntityUpdate(String), // entity_id
    /// 获取词条（参数：entity_id）
    EntityGet(String), // entity_id
    /// 删除词条（参数：entity_id）
    EntityDelete(String), // entity_id
    /// 列出词条
    EntityList,
    /// 词条匹配
    EntityMatch,
    /// 搜索词条
    EntitySearch,
    /// 词条高亮
    EntityHighlight,
    /// 词条抽取
    EntityExtract,

    /// 分类管理
    ClassificationList,

    /// 文件管理
    FileUpload,
    /// 下载文件（参数：file_token）
    FileDownload(String), // file_token
}

impl BaikeApiV1 {
    /// 提供 `to_url` 能力。
    pub fn to_url(&self) -> String {
        match self {
            BaikeApiV1::DraftCreate => "/open-apis/baike/v1/drafts".to_string(),
            BaikeApiV1::DraftUpdate(draft_id) => {
                format!("/open-apis/baike/v1/drafts/{draft_id}")
            }
            BaikeApiV1::EntityCreate => "/open-apis/baike/v1/entities".to_string(),
            BaikeApiV1::EntityUpdate(entity_id) => {
                format!("/open-apis/baike/v1/entities/{entity_id}")
            }
            BaikeApiV1::EntityGet(entity_id) => {
                format!("/open-apis/baike/v1/entities/{entity_id}")
            }
            BaikeApiV1::EntityDelete(entity_id) => {
                format!("/open-apis/baike/v1/entities/{entity_id}")
            }
            BaikeApiV1::EntityList => "/open-apis/baike/v1/entities".to_string(),
            BaikeApiV1::EntityMatch => "/open-apis/baike/v1/entities/match".to_string(),
            BaikeApiV1::EntitySearch => "/open-apis/baike/v1/entities/search".to_string(),
            BaikeApiV1::EntityHighlight => "/open-apis/baike/v1/entities/highlight".to_string(),
            BaikeApiV1::EntityExtract => "/open-apis/baike/v1/entities/extract".to_string(),
            BaikeApiV1::ClassificationList => "/open-apis/baike/v1/classifications".to_string(),
            BaikeApiV1::FileUpload => "/open-apis/baike/v1/files/upload".to_string(),
            BaikeApiV1::FileDownload(file_token) => {
                format!("/open-apis/baike/v1/files/{file_token}/download")
            }
        }
    }
}

// Baike API v1 端点
/// 飞书百科 v1 API 基础路径。
pub const BAIKE_API_V1: &str = "/open-apis/baike/v1";

// ============================================================================
// Lingo API v1 端点定义
// ============================================================================

/// Lingo语言服务 API v1 端点
#[derive(Debug, Clone, PartialEq)]
pub enum LingoApiV1 {
    /// 草稿管理
    DraftCreate,
    /// 更新草稿（参数：draft_id）
    DraftUpdate(String), // draft_id

    /// 词条管理
    EntityCreate,
    /// 更新词条（参数：entity_id）
    EntityUpdate(String), // entity_id
    /// 删除词条（参数：entity_id）
    EntityDelete(String), // entity_id
    /// 获取词条（参数：entity_id）
    EntityGet(String), // entity_id
    /// 列出词条
    EntityList,
    /// 词条匹配
    EntityMatch,
    /// 搜索词条
    EntitySearch,
    /// 词条高亮
    EntityHighlight,
    /// 批量获取词条
    EntityBatchGet,
    /// 批量更新词条
    EntityBatchUpdate,
    /// 词条搜索推荐
    EntitySearchRecommend,
    /// 获取词条历史（参数：entity_id）
    EntityHistoryGet(String), // entity_id
    /// 列出词条历史
    EntityHistoryList,

    /// 分类管理
    ClassificationList,

    /// 词库管理
    RepoList,

    /// 文件管理
    FileUpload,
    /// 下载文件（参数：file_token）
    FileDownload(String), // file_token

    /// 智能处理
    GenerateSummary,
    /// 提取关键词
    ExtractKeywords,
    /// 翻译文本
    TranslateText,
}

impl LingoApiV1 {
    /// 提供 `to_url` 能力。
    pub fn to_url(&self) -> String {
        match self {
            LingoApiV1::DraftCreate => "/open-apis/lingo/v1/drafts".to_string(),
            LingoApiV1::DraftUpdate(draft_id) => {
                format!("/open-apis/lingo/v1/drafts/{draft_id}")
            }
            LingoApiV1::EntityCreate => "/open-apis/lingo/v1/entities".to_string(),
            LingoApiV1::EntityUpdate(entity_id) => {
                format!("/open-apis/lingo/v1/entities/{entity_id}")
            }
            LingoApiV1::EntityDelete(entity_id) => {
                format!("/open-apis/lingo/v1/entities/{entity_id}")
            }
            LingoApiV1::EntityGet(entity_id) => {
                format!("/open-apis/lingo/v1/entities/{entity_id}")
            }
            LingoApiV1::EntityList => "/open-apis/lingo/v1/entities".to_string(),
            LingoApiV1::EntityMatch => "/open-apis/lingo/v1/entities/match".to_string(),
            LingoApiV1::EntitySearch => "/open-apis/lingo/v1/entities/search".to_string(),
            LingoApiV1::EntityHighlight => "/open-apis/lingo/v1/entities/highlight".to_string(),
            LingoApiV1::EntityBatchGet => "/open-apis/lingo/v1/entities:batchGet".to_string(),
            LingoApiV1::EntityBatchUpdate => "/open-apis/lingo/v1/entities:batchUpdate".to_string(),
            LingoApiV1::EntitySearchRecommend => {
                "/open-apis/lingo/v1/entities:searchRecommend".to_string()
            }
            LingoApiV1::EntityHistoryGet(entity_id) => {
                format!("/open-apis/lingo/v1/entities/{entity_id}/history")
            }
            LingoApiV1::EntityHistoryList => "/open-apis/lingo/v1/entityHistory".to_string(),
            LingoApiV1::ClassificationList => "/open-apis/lingo/v1/classifications".to_string(),
            LingoApiV1::RepoList => "/open-apis/lingo/v1/repos".to_string(),
            LingoApiV1::FileUpload => "/open-apis/lingo/v1/files/upload".to_string(),
            LingoApiV1::FileDownload(file_token) => {
                format!("/open-apis/lingo/v1/files/{file_token}/download")
            }
            LingoApiV1::GenerateSummary => "/open-apis/lingo/v1/text:generateSummary".to_string(),
            LingoApiV1::ExtractKeywords => "/open-apis/lingo/v1/text:extractKeywords".to_string(),
            LingoApiV1::TranslateText => "/open-apis/lingo/v1/text:translate".to_string(),
        }
    }
}

// Lingo API v1 端点
/// 飞书词典 v1 API 基础路径。
pub const LINGO_API_V1: &str = "/open-apis/lingo/v1";

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::api::{ApiRequest, HttpMethod};
    use openlark_core::constants::AccessTokenType;

    // ========== BaseApiV2 Tests ==========
    #[test]
    fn test_base_api_v2_role_create() {
        let endpoint = BaseApiV2::RoleCreate("app_token_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/base/v2/apps/app_token_123/roles"
        );
        assert_eq!(endpoint.method(), HttpMethod::Post);
        let req: ApiRequest<()> = endpoint.to_request();
        assert_eq!(req.method(), &HttpMethod::Post);
        // #438: catalog 拥有认证要求
        assert_eq!(
            endpoint.supported_access_token_types(),
            Some(vec![AccessTokenType::User, AccessTokenType::Tenant])
        );
        assert_eq!(
            req.supported_access_token_types(),
            vec![AccessTokenType::User, AccessTokenType::Tenant]
        );
    }

    #[test]
    fn test_base_api_v2_role_update() {
        let endpoint =
            BaseApiV2::RoleUpdate("app_token_123".to_string(), "role_id_456".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/base/v2/apps/app_token_123/roles/role_id_456"
        );
        assert_eq!(endpoint.method(), HttpMethod::Put);
        let req: ApiRequest<()> = endpoint.to_request();
        assert_eq!(req.method(), &HttpMethod::Put);
        assert_eq!(
            endpoint.supported_access_token_types(),
            Some(vec![AccessTokenType::User, AccessTokenType::Tenant])
        );
        assert_eq!(
            req.supported_access_token_types(),
            vec![AccessTokenType::User, AccessTokenType::Tenant]
        );
    }

    #[test]
    fn test_base_api_v2_role_list() {
        let endpoint = BaseApiV2::RoleList("app_token_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/base/v2/apps/app_token_123/roles"
        );
        assert_eq!(endpoint.method(), HttpMethod::Get);
        let req: ApiRequest<()> = endpoint.to_request();
        assert_eq!(req.method(), &HttpMethod::Get);
        assert_eq!(
            endpoint.supported_access_token_types(),
            Some(vec![AccessTokenType::User, AccessTokenType::Tenant])
        );
        assert_eq!(
            req.supported_access_token_types(),
            vec![AccessTokenType::User, AccessTokenType::Tenant]
        );
    }

    #[test]
    fn test_base_api_v2_role_delete() {
        let endpoint =
            BaseApiV2::RoleDelete("app_token_123".to_string(), "role_id_456".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/base/v2/apps/app_token_123/roles/role_id_456"
        );
        assert_eq!(endpoint.method(), HttpMethod::Delete);
        let req: ApiRequest<()> = endpoint.to_request();
        assert_eq!(req.method(), &HttpMethod::Delete);
        assert_eq!(
            endpoint.supported_access_token_types(),
            Some(vec![AccessTokenType::User, AccessTokenType::Tenant])
        );
        assert_eq!(
            req.supported_access_token_types(),
            vec![AccessTokenType::User, AccessTokenType::Tenant]
        );
    }

    #[test]
    fn test_base_api_v2_with_special_chars() {
        let endpoint = BaseApiV2::RoleCreate("app-token_123".to_string());
        assert!(endpoint.to_url().contains("app-token_123"));
    }

    // ========== MinutesApiV1 Tests ==========
    #[test]
    fn test_minutes_api_v1_get() {
        let endpoint = MinutesApiV1::Get("minute_token_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/minutes/v1/minutes/minute_token_123"
        );
    }

    #[test]
    fn test_minutes_api_v1_media_get() {
        let endpoint = MinutesApiV1::MediaGet("minute_token_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/minutes/v1/minutes/minute_token_123/media"
        );
    }

    #[test]
    fn test_minutes_api_v1_transcript_get() {
        let endpoint = MinutesApiV1::TranscriptGet("minute_token_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/minutes/v1/minutes/minute_token_123/transcript"
        );
    }

    #[test]
    fn test_minutes_api_v1_statistics_get() {
        let endpoint = MinutesApiV1::StatisticsGet("minute_token_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/minutes/v1/minutes/minute_token_123/statistics"
        );
    }

    #[test]
    fn minute_subscription_issue_194_endpoints() {
        assert_eq!(
            MinutesApiV1::Subscription.to_url(),
            "/open-apis/minutes/v1/minutes/subscription"
        );
        assert_eq!(
            MinutesApiV1::Unsubscription.to_url(),
            "/open-apis/minutes/v1/minutes/unsubscription"
        );
    }

    // ========== WikiApiV1 Tests ==========
    #[test]
    fn test_wiki_api_v1_node_search() {
        let endpoint = WikiApiV1::NodeSearch;
        assert_eq!(endpoint.to_url(), "/open-apis/wiki/v1/nodes/search");
    }

    // ========== DocsApiV1 Tests ==========
    #[test]
    fn test_docs_api_v1_content_get() {
        let endpoint = DocsApiV1::ContentGet;
        assert_eq!(endpoint.to_url(), "/open-apis/docs/v1/content");
    }

    // ========== DocxApiV1 Tests ==========
    #[test]
    fn test_docx_api_v1_document_create() {
        let endpoint = DocxApiV1::DocumentCreate;
        assert_eq!(endpoint.to_url(), "/open-apis/docx/v1/documents");
    }

    #[test]
    fn test_docx_api_v1_document_get() {
        let endpoint = DocxApiV1::DocumentGet("doc_id_123".to_string());
        assert_eq!(endpoint.to_url(), "/open-apis/docx/v1/documents/doc_id_123");
    }

    #[test]
    fn test_docx_api_v1_document_block_list() {
        let endpoint = DocxApiV1::DocumentBlockList("doc_id_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/docx/v1/documents/doc_id_123/blocks"
        );
    }

    #[test]
    fn test_docx_api_v1_chat_announcement_get() {
        let endpoint = DocxApiV1::ChatAnnouncementGet("chat_id_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/docx/v1/chats/chat_id_123/announcement"
        );
    }

    #[test]
    fn test_docx_api_v1_document_convert() {
        let endpoint = DocxApiV1::DocumentConvert;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/docx/documents/blocks/convert"
        );
    }

    #[test]
    fn test_docx_api_v1_document_block_children_create() {
        let endpoint = DocxApiV1::DocumentBlockChildrenCreate(
            "doc_id_123".to_string(),
            "block_id_456".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/docx/v1/documents/doc_id_123/blocks/block_id_456/children"
        );
    }

    // ========== WikiApiV2 Tests ==========
    #[test]
    fn test_wiki_api_v2_space_list() {
        let endpoint = WikiApiV2::SpaceList;
        assert_eq!(endpoint.to_url(), "/open-apis/wiki/v2/spaces");
    }

    #[test]
    fn test_wiki_api_v2_space_get() {
        let endpoint = WikiApiV2::SpaceGet("space_id_123".to_string());
        assert_eq!(endpoint.to_url(), "/open-apis/wiki/v2/spaces/space_id_123");
    }

    #[test]
    fn test_wiki_api_v2_space_create() {
        let endpoint = WikiApiV2::SpaceCreate;
        assert_eq!(endpoint.to_url(), "/open-apis/wiki/v2/spaces");
    }

    #[test]
    fn test_wiki_api_v2_space_node_list() {
        let endpoint = WikiApiV2::SpaceNodeList("space_id_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/wiki/v2/spaces/space_id_123/nodes"
        );
    }

    #[test]
    fn test_wiki_api_v2_space_member_delete() {
        let endpoint =
            WikiApiV2::SpaceMemberDelete("space_id_123".to_string(), "member_id_456".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/wiki/v2/spaces/space_id_123/members/member_id_456"
        );
    }

    #[test]
    fn test_wiki_api_v2_task_get() {
        let endpoint = WikiApiV2::TaskGet("task_id_123".to_string());
        assert_eq!(endpoint.to_url(), "/open-apis/wiki/v2/tasks/task_id_123");
    }

    // ========== CcmDocApiOld Tests ==========
    #[test]
    fn test_ccm_doc_api_old_create() {
        let endpoint = CcmDocApiOld::Create;
        assert_eq!(endpoint.to_url(), "/open-apis/doc/v2/create");
    }

    #[test]
    fn test_ccm_doc_api_old_meta() {
        let endpoint = CcmDocApiOld::Meta("doc_token_123".to_string());
        assert_eq!(endpoint.to_url(), "/open-apis/doc/v2/meta/doc_token_123");
    }

    #[test]
    fn test_ccm_doc_api_old_raw_content() {
        let endpoint = CcmDocApiOld::RawContent("doc_token_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/doc/v2/doc_token_123/raw_content"
        );
    }

    #[test]
    fn test_ccm_doc_api_old_batch_update() {
        let endpoint = CcmDocApiOld::BatchUpdate("doc_token_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/doc/v2/doc_token_123/batch_update"
        );
    }

    // ========== CcmDocsApiOld Tests ==========
    #[test]
    fn test_ccm_docs_api_old_search_object() {
        let endpoint = CcmDocsApiOld::SearchObject;
        assert_eq!(endpoint.to_url(), "/open-apis/suite/docs-api/search/object");
    }

    #[test]
    fn test_ccm_docs_api_old_meta() {
        let endpoint = CcmDocsApiOld::Meta;
        assert_eq!(endpoint.to_url(), "/open-apis/suite/docs-api/meta");
    }

    // ========== CcmDriveExplorerApiOld Tests ==========
    #[test]
    fn test_ccm_drive_explorer_api_old_root_folder_meta() {
        let endpoint = CcmDriveExplorerApiOld::RootFolderMeta;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/drive/explorer/v2/root_folder/meta"
        );
    }

    #[test]
    fn test_ccm_drive_explorer_api_old_folder_meta() {
        let endpoint = CcmDriveExplorerApiOld::FolderMeta("folder_token_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/drive/explorer/v2/folder/folder_token_123/meta"
        );
    }

    #[test]
    fn test_ccm_drive_explorer_api_old_file_copy() {
        let endpoint = CcmDriveExplorerApiOld::FileCopy("file_token_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/drive/explorer/v2/file/copy/files/file_token_123"
        );
    }

    // ========== CcmDriveExplorerApi Tests ==========
    #[test]
    fn test_ccm_drive_explorer_api_root_folder_meta() {
        let endpoint = CcmDriveExplorerApi::RootFolderMeta;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/drive/v1/explorer/root_folder/meta"
        );
    }

    #[test]
    fn test_ccm_drive_explorer_api_folder_meta() {
        let endpoint = CcmDriveExplorerApi::FolderMeta("folder_token_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/drive/v1/explorer/folder/folder_token_123/meta"
        );
    }

    #[test]
    fn test_ccm_drive_explorer_api_folder() {
        let endpoint = CcmDriveExplorerApi::Folder;
        assert_eq!(endpoint.to_url(), "/open-apis/drive/v1/explorer/folder");
    }

    #[test]
    fn test_ccm_drive_explorer_api_to_url_with_params() {
        let endpoint = CcmDriveExplorerApi::RootFolderMeta;
        let params = vec![("key", "value".to_string())];
        let url = endpoint.to_url_with_params(&params);
        assert!(url.contains("?"));
        assert!(url.contains("key=value"));
    }

    #[test]
    fn test_ccm_drive_explorer_api_to_url_with_empty_params() {
        let endpoint = CcmDriveExplorerApi::RootFolderMeta;
        let params: Vec<(&str, String)> = vec![];
        let url = endpoint.to_url_with_params(&params);
        assert!(!url.contains("?"));
    }

    #[test]
    fn test_ccm_drive_explorer_api_to_url_with_special_chars() {
        let endpoint = CcmDriveExplorerApi::RootFolderMeta;
        let params = vec![("query", "hello world".to_string())];
        let url = endpoint.to_url_with_params(&params);
        assert!(url.contains("%20"));
    }

    // ========== PermissionApi Tests ==========
    #[test]
    fn test_permission_api_member_permitted() {
        let endpoint = PermissionApi::MemberPermitted;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/drive/v1/permission/member/permitted"
        );
    }

    #[test]
    fn test_permission_api_member_transfer() {
        let endpoint = PermissionApi::MemberTransfer;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/drive/v1/permission/member/transfer"
        );
    }

    #[test]
    fn test_permission_api_public() {
        let endpoint = PermissionApi::Public;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/drive/v1/permission/v2/public/"
        );
    }

    // ========== PermissionApiOld Tests ==========
    #[test]
    fn test_permission_api_old_member_permitted() {
        let endpoint = PermissionApiOld::MemberPermitted;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/drive/v1/permission/member/permitted"
        );
    }

    #[test]
    fn test_permission_api_old_public() {
        let endpoint = PermissionApiOld::Public;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/drive/v1/permission/v2/public/"
        );
    }
}
