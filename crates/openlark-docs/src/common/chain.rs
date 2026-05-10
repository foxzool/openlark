//! # openlark-docs 链式调用入口（简化为仅配置获取）
//!
//! ## 设计理念
//!
//! openlark-docs 涵盖多个 bizTag/Project（ccm/base/bitable/baike/minutes 等），
//! 提供简洁的配置获取入口，Request 构建仍使用各 `*RequestBuilder/*Request` 的 `new(config)` / `execute(...)`。
//!
//! ## 推荐入口
//!
//! **公开入口** (推荐用户使用):
//! - `DocsClient` - 文档服务的唯一公开入口
//! - 示例: `DocsClient::new(config).ccm.config().clone()` 用于获取配置
//!
//! ## 推荐调用方式
//!
//! ```rust,ignore
//! use openlark_core::config::Config;
//! use openlark_docs::DocsClient;
//!
//! // 创建客户端
//! let config = Config::builder()
//!     .app_id("app_id")
//!     .app_secret("app_secret")
//!     .build();
//! let docs = DocsClient::new(config);
//!
//! // ✅ 推荐：获取配置后构建 Request
//! // 访问云盘服务
//! let config = docs.ccm.config().clone();
//! // let file = UploadAllRequest::new(config, ...).execute().await?;
//!
//! // 访问多维表格
//! let config = docs.base.bitable.config().clone();
//! // let table = CreateTableRequest::new(config, ...).execute().await?;
//!
//! // 访问知识库
//! let config = docs.ccm.wiki.config().clone();
//! // let node = CreateNodeRequest::new(config, ...).execute().await?;
//! ```

#[cfg(any(feature = "ccm-core", feature = "bitable"))]
use openlark_core::SDKResult;
use openlark_core::config::Config;
#[cfg(feature = "ccm-core")]
use openlark_core::error::{CoreError, business_error, validation_error};
use std::sync::Arc;

/// 统一的 typed pagination 返回页。
///
/// 相比直接暴露各 API 的原始分页字段，该结构统一使用 `next_page_token` 命名，
/// 方便后续在 Drive / Docs helper 中复用同一套分页范式。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedPage<T> {
    /// 当前页结果项。
    pub items: Vec<T>,
    /// 是否还有下一页。
    pub has_more: bool,
    /// 下一页分页标记。
    pub next_page_token: Option<String>,
}

impl<T> TypedPage<T> {
    /// 创建新的实例。
    pub fn new(items: Vec<T>, has_more: bool, next_page_token: Option<String>) -> Self {
        Self {
            items,
            has_more,
            next_page_token,
        }
    }

    /// 提供 `empty` 能力。
    pub fn empty() -> Self {
        Self::new(Vec::new(), false, None)
    }

    /// 提供 `is_last_page` 能力。
    pub fn is_last_page(&self) -> bool {
        !self.has_more
    }

    /// 提供 `into_items` 能力。
    pub fn into_items(self) -> Vec<T> {
        self.items
    }
}

#[cfg(feature = "ccm-core")]
/// 公开项说明。
pub type FolderChildrenPage = TypedPage<crate::ccm::explorer::v2::models::FileItem>;

/// 电子表格范围 helper。
///
/// 统一 sheet 标识与 A1 范围表达，避免业务侧手工拼接
/// `sheet_id!A1:C5` 之类的字符串。
#[cfg(feature = "ccm-core")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SheetRange {
    /// 工作表标识。
    pub sheet_id: String,
    /// 起始单元格。
    pub start_cell: String,
    /// 结束单元格；为空时表示单格或单起点范围。
    pub end_cell: Option<String>,
}

#[cfg(feature = "ccm-core")]
impl SheetRange {
    /// 从工作表 ID + 起始单元格创建范围。
    pub fn new(sheet_id: impl Into<String>, start_cell: impl Into<String>) -> Self {
        Self {
            sheet_id: sheet_id.into(),
            start_cell: start_cell.into(),
            end_cell: None,
        }
    }

    /// 补充结束单元格，形成 `A1:C5` 这类闭区间范围。
    pub fn with_end_cell(mut self, end_cell: impl Into<String>) -> Self {
        self.end_cell = Some(end_cell.into());
        self
    }

    /// 从工作表 ID 与相对范围表达式创建范围。
    ///
    /// `range_expr` 仅应包含单元格部分，例如 `A1` 或 `A1:C5`。
    pub fn from_range_expr(
        sheet_id: impl Into<String>,
        range_expr: impl AsRef<str>,
    ) -> SDKResult<Self> {
        let sheet_id = validate_sheet_range_part("sheet_id", sheet_id.into())?;
        let expr = range_expr.as_ref().trim();
        if expr.is_empty() {
            return Err(validation_error("range_expr", "range_expr 不能为空"));
        }
        if expr.contains('!') {
            return Err(validation_error(
                "range_expr",
                "range_expr 不应包含工作表前缀，请仅传入单元格范围",
            ));
        }

        match expr.split_once(':') {
            Some((start, end)) => Ok(Self::new(
                sheet_id,
                validate_sheet_range_part("start_cell", start)?,
            )
            .with_end_cell(validate_sheet_range_part("end_cell", end)?)),
            None => Ok(Self::new(
                sheet_id,
                validate_sheet_range_part("start_cell", expr)?,
            )),
        }
    }

    /// 解析完整的 A1 表达式，例如 `sheet_id!A1:C5`。
    pub fn parse(a1_notation: impl AsRef<str>) -> SDKResult<Self> {
        let notation = a1_notation.as_ref().trim();
        let (sheet_id, range_expr) = notation.split_once('!').ok_or_else(|| {
            validation_error(
                "a1_notation",
                "A1 表达式必须包含工作表前缀，例如 sheet_id!A1:C5",
            )
        })?;

        Self::from_range_expr(sheet_id, range_expr)
    }

    /// 返回不带工作表前缀的范围表达式。
    pub fn range_expr(&self) -> String {
        match &self.end_cell {
            Some(end_cell) => format!("{}:{}", self.start_cell, end_cell),
            None => self.start_cell.clone(),
        }
    }

    /// 返回完整的 A1 表达式。
    pub fn to_a1_notation(&self) -> String {
        format!("{}!{}", self.sheet_id, self.range_expr())
    }
}

#[cfg(feature = "ccm-core")]
impl std::fmt::Display for SheetRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_a1_notation())
    }
}

/// 多维表格记录查询 helper。
///
/// 封装常见字段过滤场景，避免业务侧直接拼接 `FilterInfo` /
/// `FilterCondition` 结构体。
#[cfg(feature = "bitable")]
#[derive(Debug, Clone, PartialEq)]
pub struct BitableRecordQuery {
    /// 多维表格 app_token。
    pub app_token: String,
    /// 数据表 table_id。
    pub table_id: String,
    conjunction: String,
    filters: Vec<crate::base::bitable::v1::app::table::record::search::FilterCondition>,
    field_names: Option<Vec<String>>,
    automatic_fields: bool,
}

#[cfg(feature = "bitable")]
impl BitableRecordQuery {
    /// 创建一个新的记录查询 helper。
    pub fn new(app_token: impl Into<String>, table_id: impl Into<String>) -> Self {
        Self {
            app_token: app_token.into(),
            table_id: table_id.into(),
            conjunction: "and".to_string(),
            filters: Vec::new(),
            field_names: None,
            automatic_fields: true,
        }
    }

    /// 指定返回字段名，减少无关字段回传。
    pub fn field_names(mut self, field_names: Vec<String>) -> Self {
        self.field_names = Some(field_names);
        self
    }

    /// 控制是否返回自动字段。
    pub fn automatic_fields(mut self, automatic_fields: bool) -> Self {
        self.automatic_fields = automatic_fields;
        self
    }

    /// 将条件组合方式切换为 `or`。
    pub fn or(mut self) -> Self {
        self.conjunction = "or".to_string();
        self
    }

    /// 按字段精确匹配。
    pub fn where_equals(mut self, field_name: impl Into<String>, value: impl Into<String>) -> Self {
        self.filters.push(
            crate::base::bitable::v1::app::table::record::search::FilterCondition {
                field_name: field_name.into(),
                operator: "is".to_string(),
                value: Some(vec![value.into()]),
            },
        );
        self
    }

    /// 按字段模糊包含匹配。
    pub fn where_contains(
        mut self,
        field_name: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.filters.push(
            crate::base::bitable::v1::app::table::record::search::FilterCondition {
                field_name: field_name.into(),
                operator: "contains".to_string(),
                value: Some(vec![value.into()]),
            },
        );
        self
    }

    /// 按字段命中多个候选值。
    pub fn where_in(mut self, field_name: impl Into<String>, values: Vec<String>) -> Self {
        self.filters.push(
            crate::base::bitable::v1::app::table::record::search::FilterCondition {
                field_name: field_name.into(),
                operator: "isAnyOf".to_string(),
                value: Some(values),
            },
        );
        self
    }

    fn into_parts(
        self,
    ) -> (
        String,
        String,
        Option<Vec<String>>,
        bool,
        Option<crate::base::bitable::v1::app::table::record::search::FilterInfo>,
    ) {
        let filter = if self.filters.is_empty() {
            None
        } else {
            Some(
                crate::base::bitable::v1::app::table::record::search::FilterInfo {
                    conjunction: Some(self.conjunction),
                    conditions: Some(self.filters),
                },
            )
        };

        (
            self.app_token,
            self.table_id,
            self.field_names,
            self.automatic_fields,
            filter,
        )
    }
}

/// 批量写入单个范围的数据单元。
#[cfg(feature = "ccm-core")]
#[derive(Debug, Clone, PartialEq)]
pub struct SheetWriteRange {
    /// 目标范围。
    pub range: SheetRange,
    /// 主维度，默认按行写入。
    pub major_dimension: String,
    /// 单元格值。
    pub values: Vec<Vec<serde_json::Value>>,
}

#[cfg(feature = "ccm-core")]
impl SheetWriteRange {
    /// 创建一条按行写入的范围数据。
    pub fn new(range: SheetRange, values: Vec<Vec<serde_json::Value>>) -> Self {
        Self {
            range,
            major_dimension: "ROWS".to_string(),
            values,
        }
    }

    /// 覆盖主维度，例如 `COLUMNS`。
    pub fn major_dimension(mut self, major_dimension: impl Into<String>) -> Self {
        self.major_dimension = major_dimension.into();
        self
    }
}

#[cfg(feature = "ccm-core")]
impl From<SheetWriteRange> for crate::ccm::sheets_v2::v2::data_io::models::BatchWriteData {
    fn from(value: SheetWriteRange) -> Self {
        Self {
            data_range: value.range.to_string(),
            major_dimension: value.major_dimension,
            values: value.values,
        }
    }
}

/// Drive 下载范围 helper。
///
/// 用于避免业务侧手工拼接 `bytes=0-1023` 这类 Range 头。
#[cfg(feature = "ccm-core")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriveDownloadRange {
    /// 起始字节位置。
    pub start: u64,
    /// 结束字节位置；为空表示读取到文件尾部。
    pub end: Option<u64>,
}

#[cfg(feature = "ccm-core")]
impl DriveDownloadRange {
    /// 创建从 `start` 开始的下载范围。
    pub fn from_start(start: u64) -> Self {
        Self { start, end: None }
    }

    /// 指定结束位置。
    pub fn with_end(mut self, end: u64) -> Self {
        self.end = Some(end);
        self
    }

    /// 生成 HTTP Range 头。
    pub fn to_header_value(&self) -> String {
        match self.end {
            Some(end) => format!("bytes={}-{}", self.start, end),
            None => format!("bytes={}-", self.start),
        }
    }
}

#[cfg(feature = "ccm-core")]
impl std::fmt::Display for DriveDownloadRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_header_value())
    }
}

/// Wiki 节点路径 helper。
///
/// 统一处理 `产品文档/发布计划/周报` 这类按标题导航的路径表达。
#[cfg(feature = "ccm-core")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WikiNodePath {
    segments: Vec<String>,
}

#[cfg(feature = "ccm-core")]
impl WikiNodePath {
    /// 基于路径片段创建 Wiki 路径。
    pub fn new(segments: Vec<String>) -> SDKResult<Self> {
        let segments = segments
            .into_iter()
            .map(validate_wiki_path_segment)
            .collect::<SDKResult<Vec<_>>>()?;
        if segments.is_empty() {
            return Err(validation_error(
                "wiki_path",
                "wiki_path 至少需要一个路径片段",
            ));
        }
        Ok(Self { segments })
    }

    /// 从 `/` 分隔的路径字符串解析 Wiki 路径。
    pub fn parse(path: impl AsRef<str>) -> SDKResult<Self> {
        let raw = path.as_ref().trim().trim_matches('/');
        if raw.is_empty() {
            return Err(validation_error("wiki_path", "wiki_path 不能为空"));
        }
        Self::new(raw.split('/').map(str::to_string).collect())
    }

    /// 返回路径片段。
    pub fn segments(&self) -> &[String] {
        &self.segments
    }
}

#[cfg(feature = "ccm-core")]
impl std::fmt::Display for WikiNodePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.segments.join("/"))
    }
}

/// Drive 上传文件 helper。
///
/// 统一封装文件名、字节内容与可选 checksum，并在 helper 层自动补全
/// `parent_type=explorer` 与 `size=file.len()` 等默认策略。
#[cfg(feature = "ccm-core")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriveUploadFile {
    /// 文件名。
    pub file_name: String,
    /// 文件内容。
    pub content: Vec<u8>,
    /// 可选的 Adler-32 checksum。
    pub checksum: Option<String>,
}

#[cfg(feature = "ccm-core")]
impl DriveUploadFile {
    /// 创建上传文件。
    pub fn new(file_name: impl Into<String>, content: Vec<u8>) -> Self {
        Self {
            file_name: file_name.into(),
            content,
            checksum: None,
        }
    }

    /// 设置校验和。
    pub fn checksum(mut self, checksum: impl Into<String>) -> Self {
        self.checksum = Some(checksum.into());
        self
    }

    /// 计算文件大小。
    pub fn size(&self) -> usize {
        self.content.len()
    }

    /// 构建底层上传请求。
    pub fn into_request(
        self,
        config: Config,
        folder_token: impl Into<String>,
    ) -> crate::ccm::drive::v1::file::UploadAllRequest {
        let mut request = crate::ccm::drive::v1::file::UploadAllRequest::new(
            config,
            self.file_name,
            folder_token,
            "explorer",
            self.content.len(),
            self.content,
        );
        if let Some(checksum) = self.checksum {
            request = request.checksum(checksum);
        }
        request
    }
}

#[cfg(feature = "ccm-core")]
impl From<crate::ccm::explorer::v2::models::FolderChildrenData>
    for TypedPage<crate::ccm::explorer::v2::models::FileItem>
{
    fn from(data: crate::ccm::explorer::v2::models::FolderChildrenData) -> Self {
        Self::new(data.items, data.has_more, data.page_token)
    }
}

/// 文件夹子项分页 helper。
///
/// 用于按页读取 Drive Explorer 文件夹内容，并统一分页返回形态。
#[cfg(feature = "ccm-core")]
#[derive(Debug, Clone)]
pub struct FolderChildrenPager {
    config: Arc<Config>,
    folder_token: String,
    doc_type: Option<String>,
    page_size: i32,
    next_page_token: Option<String>,
    exhausted: bool,
}

#[cfg(feature = "ccm-core")]
impl FolderChildrenPager {
    fn new(config: Arc<Config>, folder_token: impl Into<String>) -> Self {
        Self {
            config,
            folder_token: folder_token.into(),
            doc_type: None,
            page_size: crate::common::constants::DEFAULT_PAGE_SIZE,
            next_page_token: None,
            exhausted: false,
        }
    }

    /// 设置文件类型过滤。
    pub fn doc_type(mut self, doc_type: impl Into<String>) -> Self {
        self.doc_type = Some(doc_type.into());
        self
    }

    /// 设置分页大小，自动限制在 1..=MAX_PAGE_SIZE 范围内。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = page_size.clamp(1, crate::common::constants::MAX_PAGE_SIZE);
        self
    }

    /// 从指定分页 token 恢复读取。
    pub fn next_page_token(mut self, next_page_token: impl Into<String>) -> Self {
        self.next_page_token = Some(next_page_token.into());
        self
    }

    /// 查看当前即将请求的下一页 token。
    pub fn pending_page_token(&self) -> Option<&str> {
        self.next_page_token.as_deref()
    }

    /// 读取下一页结果。
    pub async fn fetch_next_page(&mut self) -> SDKResult<FolderChildrenPage> {
        use crate::ccm::explorer::v2::{GetFolderChildrenParams, GetFolderChildrenRequest};

        if self.exhausted {
            return Ok(TypedPage::empty());
        }

        let response = GetFolderChildrenRequest::new(
            self.config.as_ref().clone(),
            &self.folder_token,
            Some(GetFolderChildrenParams {
                page_size: Some(self.page_size),
                page_token: self.next_page_token.clone(),
                doc_type: self.doc_type.clone(),
            }),
        )
        .execute()
        .await?;

        let page = response
            .data
            .map(TypedPage::from)
            .unwrap_or_else(TypedPage::empty);
        self.exhausted = !page.has_more;
        self.next_page_token = if page.has_more {
            page.next_page_token.clone()
        } else {
            None
        };

        Ok(page)
    }

    /// 收集当前 pager 剩余的所有结果。
    pub async fn collect_all(
        mut self,
    ) -> SDKResult<Vec<crate::ccm::explorer::v2::models::FileItem>> {
        let mut items = Vec::new();

        loop {
            let page = self.fetch_next_page().await?;
            let is_last_page = page.is_last_page();
            items.extend(page.into_items());
            if is_last_page {
                break;
            }
        }

        Ok(items)
    }
}

/// Docs 链式入口：`docs.ccm.config()` / `docs.base.bitable.config()`（按 feature 裁剪）
#[derive(Debug, Clone)]
pub struct DocsClient {
    config: Arc<Config>,

    #[cfg(feature = "ccm-core")]
    /// 公开项说明。
    pub ccm: CcmClient,

    #[cfg(any(feature = "base", feature = "bitable"))]
    /// 公开项说明。
    pub base: BaseClient,

    #[cfg(any(feature = "baike", feature = "lingo"))]
    /// 公开项说明。
    pub baike: BaikeClient,

    #[cfg(feature = "minutes")]
    /// 公开项说明。
    pub minutes: MinutesClient,
}

impl DocsClient {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        let config = Arc::new(config);
        Self {
            config: config.clone(),
            #[cfg(feature = "ccm-core")]
            ccm: CcmClient::new(config.clone()),
            #[cfg(any(feature = "base", feature = "bitable"))]
            base: BaseClient::new(config.clone()),
            #[cfg(any(feature = "baike", feature = "lingo"))]
            baike: BaikeClient::new(config.clone()),
            #[cfg(feature = "minutes")]
            minutes: MinutesClient::new(config),
        }
    }

    /// 返回共享配置。
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 创建文件夹子项分页 helper。
    #[cfg(feature = "ccm-core")]
    pub fn folder_children_pager(&self, folder_token: impl Into<String>) -> FolderChildrenPager {
        FolderChildrenPager::new(self.config.clone(), folder_token)
    }

    /// 获取文件夹下的全部子项，自动处理分页。
    #[cfg(feature = "ccm-core")]
    pub async fn list_folder_children_all(
        &self,
        folder_token: &str,
        doc_type: Option<&str>,
    ) -> SDKResult<Vec<crate::ccm::explorer::v2::models::FileItem>> {
        let mut pager = self
            .folder_children_pager(folder_token)
            .page_size(crate::common::constants::MAX_PAGE_SIZE);
        if let Some(doc_type) = doc_type {
            pager = pager.doc_type(doc_type);
        }

        pager.collect_all().await
    }

    /// 读取多维表格全部记录，自动处理分页。
    #[cfg(feature = "bitable")]
    pub async fn search_bitable_records_all(
        &self,
        app_token: &str,
        table_id: &str,
    ) -> SDKResult<Vec<crate::base::bitable::v1::app::table::record::models::Record>> {
        use crate::base::bitable::v1::app::table::record::search::SearchRecordRequest;

        SearchRecordRequest::new(self.config().clone())
            .app_token(app_token.to_string())
            .table_id(table_id.to_string())
            .automatic_fields(true)
            .fetch_all()
            .await
    }

    /// 使用 helper 风格执行常见多维表格过滤查询，并自动处理分页。
    #[cfg(feature = "bitable")]
    pub async fn query_bitable_records(
        &self,
        query: BitableRecordQuery,
    ) -> SDKResult<Vec<crate::base::bitable::v1::app::table::record::models::Record>> {
        use crate::base::bitable::v1::app::table::record::search::SearchRecordRequest;

        let (app_token, table_id, field_names, automatic_fields, filter) = query.into_parts();
        let mut request = SearchRecordRequest::new(self.config().clone())
            .app_token(app_token)
            .table_id(table_id)
            .automatic_fields(automatic_fields);

        if let Some(field_names) = field_names {
            request = request.field_names(field_names);
        }

        if let Some(filter) = filter {
            request = request.filter(filter);
        }

        request.fetch_all().await
    }

    /// 读取多个单元格范围，返回聚合后的范围数据。
    #[cfg(feature = "ccm-core")]
    pub async fn read_multiple_ranges(
        &self,
        spreadsheet_token: &str,
        ranges: Vec<String>,
    ) -> SDKResult<crate::ccm::sheets_v2::v2::data_io::models::MultipleRangeData> {
        use crate::ccm::sheets_v2::v2::data_io::{
            ReadMultipleRangesParams, read_multiple_ranges as read_multiple_ranges_api,
        };

        let response = read_multiple_ranges_api(
            self.config(),
            spreadsheet_token,
            ReadMultipleRangesParams {
                ranges,
                value_render_option: None,
                date_render_option: None,
            },
        )
        .await?;

        response
            .data
            .ok_or_else(|| CoreError::api_data_error("读取多个范围"))
    }

    /// 使用 typed `SheetRange` 批量读取多个范围。
    #[cfg(feature = "ccm-core")]
    pub async fn read_sheet_ranges(
        &self,
        spreadsheet_token: &str,
        ranges: Vec<SheetRange>,
    ) -> SDKResult<crate::ccm::sheets_v2::v2::data_io::models::MultipleRangeData> {
        self.read_multiple_ranges(
            spreadsheet_token,
            ranges.into_iter().map(|range| range.to_string()).collect(),
        )
        .await
    }

    /// 批量写入多个单元格范围。
    #[cfg(feature = "ccm-core")]
    pub async fn write_multiple_ranges(
        &self,
        spreadsheet_token: &str,
        data: Vec<crate::ccm::sheets_v2::v2::data_io::models::BatchWriteData>,
    ) -> SDKResult<crate::ccm::sheets_v2::v2::data_io::models::BatchUpdateResult> {
        use crate::ccm::sheets_v2::v2::data_io::{BatchWriteRangesParams, batch_write_ranges};

        let response = batch_write_ranges(
            self.config(),
            spreadsheet_token,
            BatchWriteRangesParams {
                data,
                include_style: None,
            },
        )
        .await?;

        response
            .data
            .ok_or_else(|| CoreError::api_data_error("批量写入多个范围"))
    }

    /// 使用 typed `SheetRange` 批量写入多个范围。
    #[cfg(feature = "ccm-core")]
    pub async fn write_sheet_ranges(
        &self,
        spreadsheet_token: &str,
        ranges: Vec<SheetWriteRange>,
    ) -> SDKResult<crate::ccm::sheets_v2::v2::data_io::models::BatchUpdateResult> {
        self.write_multiple_ranges(
            spreadsheet_token,
            ranges.into_iter().map(Into::into).collect(),
        )
        .await
    }

    /// 使用 typed `SheetRange` 追加数据。
    #[cfg(feature = "ccm-core")]
    pub async fn append_sheet_range(
        &self,
        spreadsheet_token: &str,
        range: SheetRange,
        values: Vec<Vec<serde_json::Value>>,
    ) -> SDKResult<crate::ccm::sheets_v2::v2::data_io::models::AppendResult> {
        use crate::ccm::sheets_v2::v2::data_io::{AppendValuesParams, append_values};

        let response = append_values(
            self.config(),
            spreadsheet_token,
            AppendValuesParams {
                range: range.to_string(),
                major_dimension: None,
                values,
            },
        )
        .await?;

        response
            .data
            .ok_or_else(|| CoreError::api_data_error("追加工作表范围"))
    }

    /// 使用默认的 `parent_type=explorer` 与自动 size 推断上传 Drive 文件。
    #[cfg(feature = "ccm-core")]
    pub async fn upload_drive_file(
        &self,
        folder_token: &str,
        file: DriveUploadFile,
    ) -> SDKResult<crate::ccm::drive::v1::file::UploadAllResponse> {
        file.into_request(self.config().clone(), folder_token)
            .execute()
            .await
    }

    /// 下载完整 Drive 文件内容。
    #[cfg(feature = "ccm-core")]
    pub async fn download_drive_file(&self, file_token: &str) -> SDKResult<Vec<u8>> {
        use crate::ccm::drive::v1::file::DownloadFileRequest;

        DownloadFileRequest::new(self.config().clone(), file_token)
            .execute()
            .await?
            .into_result()
    }

    /// 按范围下载 Drive 文件内容。
    #[cfg(feature = "ccm-core")]
    pub async fn download_drive_file_range(
        &self,
        file_token: &str,
        range: DriveDownloadRange,
    ) -> SDKResult<Vec<u8>> {
        use crate::ccm::drive::v1::file::DownloadFileRequest;

        DownloadFileRequest::new(self.config().clone(), file_token)
            .range(range.to_string())
            .execute()
            .await?
            .into_result()
    }

    /// 获取指定知识空间下的所有节点，自动处理分页。
    #[cfg(feature = "ccm-core")]
    pub async fn list_wiki_space_nodes_all(
        &self,
        space_id: &str,
        parent_node_token: Option<&str>,
    ) -> SDKResult<Vec<crate::ccm::wiki::v2::models::WikiSpaceNode>> {
        use crate::ccm::wiki::v2::space::node::{
            ListWikiSpaceNodesParams, ListWikiSpaceNodesRequest,
        };

        let mut items = Vec::new();
        let mut page_token: Option<String> = None;

        loop {
            let response = ListWikiSpaceNodesRequest::new(self.config().clone())
                .space_id(space_id)
                .execute(Some(ListWikiSpaceNodesParams {
                    parent_node_token: parent_node_token.map(str::to_string),
                    page_size: Some(crate::common::constants::MAX_PAGE_SIZE),
                    page_token: page_token.clone(),
                }))
                .await?;

            items.extend(response.items);

            if !response.has_more.unwrap_or(false) {
                break;
            }

            page_token = response.page_token;
        }

        Ok(items)
    }

    /// 在指定层级下按标题查找单个 Wiki 节点。
    #[cfg(feature = "ccm-core")]
    pub async fn find_wiki_node_by_title(
        &self,
        space_id: &str,
        title: &str,
        parent_node_token: Option<&str>,
    ) -> SDKResult<crate::ccm::wiki::v2::models::WikiSpaceNode> {
        let items = self
            .list_wiki_space_nodes_all(space_id, parent_node_token)
            .await?;
        find_unique_wiki_node_by_title(&items, title)
    }

    /// 通过路径逐级导航 Wiki 节点。
    #[cfg(feature = "ccm-core")]
    pub async fn find_wiki_node_by_path(
        &self,
        space_id: &str,
        path: impl AsRef<str>,
    ) -> SDKResult<crate::ccm::wiki::v2::models::WikiSpaceNode> {
        let path = WikiNodePath::parse(path)?;
        let mut parent_node_token: Option<String> = None;
        let mut current_node = None;

        for segment in path.segments() {
            let node = self
                .find_wiki_node_by_title(space_id, segment, parent_node_token.as_deref())
                .await?;
            parent_node_token = Some(node.node_token.clone());
            current_node = Some(node);
        }

        current_node.ok_or_else(|| business_error(format!("未找到 Wiki 路径: {path}")))
    }

    /// 根据工作表标题查找工作表。
    #[cfg(feature = "ccm-core")]
    pub async fn find_sheet_by_title(
        &self,
        spreadsheet_token: &str,
        title: &str,
    ) -> SDKResult<crate::ccm::sheets_v2::v2::spreadsheet::models::SpreadsheetSheetInfo> {
        let sheets = self.list_sheet_infos(spreadsheet_token).await?;

        find_sheet_info(&sheets, title)
            .ok_or_else(|| business_error(format!("未找到工作表: {title}")))
    }

    /// 按工作表标题解析单个范围，返回统一的 `SheetRange` 表达。
    #[cfg(feature = "ccm-core")]
    pub async fn resolve_sheet_range_by_title(
        &self,
        spreadsheet_token: &str,
        title: &str,
        range_expr: &str,
    ) -> SDKResult<SheetRange> {
        let sheet = self.find_sheet_by_title(spreadsheet_token, title).await?;
        SheetRange::from_range_expr(sheet.sheet_id, range_expr)
    }

    #[cfg(feature = "ccm-core")]
    /// 列出 sheet infos。
    pub async fn list_sheet_infos(
        &self,
        spreadsheet_token: &str,
    ) -> SDKResult<Vec<crate::ccm::sheets_v2::v2::spreadsheet::models::SpreadsheetSheetInfo>> {
        use crate::ccm::sheets::v3::spreadsheet::sheet::query::query_sheets;
        use crate::ccm::sheets_v2::v2::spreadsheet::models::SpreadsheetSheetInfo;

        log::info!("[OPENLARK DEBUG] list_sheet_infos called with token: {spreadsheet_token}");

        let response = query_sheets(self.config(), spreadsheet_token).await?;

        log::info!(
            "[OPENLARK DEBUG] query_sheets response count: {}",
            response.sheets.len()
        );

        let sheets: Vec<SpreadsheetSheetInfo> =
            response.sheets.into_iter().map(map_v3_sheet_info).collect();

        if sheets.is_empty() {
            return Err(CoreError::api_data_error("获取工作表列表"));
        }

        Ok(sheets)
    }
}

#[cfg(feature = "ccm-core")]
fn map_v3_sheet_info(
    sheet: crate::ccm::sheets::v3::spreadsheet::Sheet,
) -> crate::ccm::sheets_v2::v2::spreadsheet::models::SpreadsheetSheetInfo {
    crate::ccm::sheets_v2::v2::spreadsheet::models::SpreadsheetSheetInfo {
        sheet_id: sheet.sheet_id,
        title: sheet.title,
        sheet_type: sheet.resource_type,
        row_count: sheet.grid_properties.row_count,
        column_count: sheet.grid_properties.column_count,
    }
}

#[cfg(feature = "ccm-core")]
fn find_sheet_info(
    sheets: &[crate::ccm::sheets_v2::v2::spreadsheet::models::SpreadsheetSheetInfo],
    title: &str,
) -> Option<crate::ccm::sheets_v2::v2::spreadsheet::models::SpreadsheetSheetInfo> {
    sheets.iter().find(|sheet| sheet.title == title).cloned()
}

#[cfg(feature = "ccm-core")]
fn validate_sheet_range_part(field: &str, value: impl Into<String>) -> SDKResult<String> {
    let value = value.into().trim().to_string();
    if value.is_empty() {
        return Err(validation_error(field, &format!("{field} 不能为空")));
    }
    Ok(value)
}

#[cfg(feature = "ccm-core")]
fn validate_wiki_path_segment(value: impl Into<String>) -> SDKResult<String> {
    let value = value.into().trim().to_string();
    if value.is_empty() {
        return Err(validation_error(
            "wiki_path_segment",
            "wiki_path_segment 不能为空",
        ));
    }
    Ok(value)
}

#[cfg(feature = "ccm-core")]
fn find_unique_wiki_node_by_title(
    nodes: &[crate::ccm::wiki::v2::models::WikiSpaceNode],
    title: &str,
) -> SDKResult<crate::ccm::wiki::v2::models::WikiSpaceNode> {
    let title = title.trim();
    if title.is_empty() {
        return Err(validation_error("title", "title 不能为空"));
    }

    let mut matches = nodes
        .iter()
        .filter(|node| node.title.as_deref() == Some(title))
        .cloned();

    let first = matches
        .next()
        .ok_or_else(|| business_error(format!("未找到 Wiki 节点标题: {title}")))?;

    if matches.next().is_some() {
        return Err(business_error(format!(
            "找到多个同名 Wiki 节点，请缩小范围: {title}"
        )));
    }

    Ok(first)
}

/// ccm：`docs.ccm`（云文档协同）
#[cfg(feature = "ccm-core")]
#[derive(Debug, Clone)]
pub struct CcmClient {
    config: Arc<Config>,
}

#[cfg(feature = "ccm-core")]
impl CcmClient {
    fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 返回共享配置。
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// base：`docs.base`（base/bitable 都归口在 base 模块下）
#[cfg(any(feature = "base", feature = "bitable"))]
#[derive(Debug, Clone)]
pub struct BaseClient {
    config: Arc<Config>,
}

#[cfg(any(feature = "base", feature = "bitable"))]
impl BaseClient {
    fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 返回共享配置。
    pub fn config(&self) -> &Config {
        &self.config
    }

    #[cfg(feature = "bitable")]
    /// 返回多维表格客户端。
    pub fn bitable(&self) -> BitableClient {
        BitableClient::new(self.config.clone())
    }
}

/// bitable：`docs.base.bitable`（多维表格）
#[cfg(feature = "bitable")]
#[derive(Debug, Clone)]
pub struct BitableClient {
    config: Arc<Config>,
}

#[cfg(feature = "bitable")]
impl BitableClient {
    fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 返回共享配置。
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// baike：`docs.baike`（baike/lingo 相关）
#[cfg(any(feature = "baike", feature = "lingo"))]
#[derive(Debug, Clone)]
pub struct BaikeClient {
    config: Arc<Config>,
}

#[cfg(any(feature = "baike", feature = "lingo"))]
impl BaikeClient {
    fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 返回共享配置。
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// minutes：`docs.minutes`（会议纪要）
#[cfg(feature = "minutes")]
#[derive(Debug, Clone)]
pub struct MinutesClient {
    config: Arc<Config>,
}

#[cfg(feature = "minutes")]
impl MinutesClient {
    fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 返回共享配置。
    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }

    #[test]
    fn test_typed_page_last_page_state() {
        let page = TypedPage::new(vec![1, 2], false, None);
        assert!(page.is_last_page());
        assert_eq!(page.into_items(), vec![1, 2]);
    }

    #[cfg(feature = "ccm-core")]
    #[test]
    fn test_drive_download_range_formats_header() {
        let full = DriveDownloadRange::from_start(0).with_end(1023);
        let tail = DriveDownloadRange::from_start(2048);

        assert_eq!(full.to_string(), "bytes=0-1023");
        assert_eq!(tail.to_string(), "bytes=2048-");
    }

    #[cfg(feature = "ccm-core")]
    #[test]
    fn test_drive_upload_file_builds_default_request() {
        let upload = DriveUploadFile::new("report.csv", vec![1, 2, 3]).checksum("abc123");
        let request = upload.into_request(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
            "folder_token",
        );

        assert_eq!(request.file_name, "report.csv");
        assert_eq!(request.parent_node, "folder_token");
        assert_eq!(request.parent_type, "explorer");
        assert_eq!(request.size, 3);
        assert_eq!(request.checksum.as_deref(), Some("abc123"));
        assert_eq!(request.file, vec![1, 2, 3]);
    }

    #[cfg(feature = "ccm-core")]
    #[test]
    fn test_wiki_node_path_parses_segments() {
        let path = WikiNodePath::parse("/产品文档/发布计划/周报/").unwrap();

        assert_eq!(
            path.segments(),
            &vec![
                "产品文档".to_string(),
                "发布计划".to_string(),
                "周报".to_string()
            ]
        );
        assert_eq!(path.to_string(), "产品文档/发布计划/周报");
    }

    #[cfg(feature = "ccm-core")]
    #[test]
    fn test_find_unique_wiki_node_by_title_rejects_duplicates() {
        let nodes = vec![
            crate::ccm::wiki::v2::models::WikiSpaceNode {
                space_id: "space_1".to_string(),
                node_token: "node_a".to_string(),
                obj_token: None,
                obj_type: None,
                parent_node_token: None,
                title: Some("周报".to_string()),
                url: None,
            },
            crate::ccm::wiki::v2::models::WikiSpaceNode {
                space_id: "space_1".to_string(),
                node_token: "node_b".to_string(),
                obj_token: None,
                obj_type: None,
                parent_node_token: None,
                title: Some("周报".to_string()),
                url: None,
            },
        ];

        let error = find_unique_wiki_node_by_title(&nodes, "周报").unwrap_err();
        assert!(error.to_string().contains("多个同名"));
    }

    #[cfg(feature = "bitable")]
    #[test]
    fn test_bitable_record_query_builds_default_and_filters() {
        let query = BitableRecordQuery::new("app_token", "table_id")
            .where_equals("状态", "进行中")
            .where_contains("负责人", "张三");

        let (app_token, table_id, field_names, automatic_fields, filter) = query.into_parts();
        let filter = filter.expect("filter should exist");

        assert_eq!(app_token, "app_token");
        assert_eq!(table_id, "table_id");
        assert!(field_names.is_none());
        assert!(automatic_fields);
        assert_eq!(filter.conjunction.as_deref(), Some("and"));
        assert_eq!(filter.conditions.as_ref().map(Vec::len), Some(2));
        assert_eq!(filter.conditions.as_ref().unwrap()[0].operator, "is");
        assert_eq!(filter.conditions.as_ref().unwrap()[1].operator, "contains");
    }

    #[cfg(feature = "bitable")]
    #[test]
    fn test_bitable_record_query_supports_or_and_value_lists() {
        let query = BitableRecordQuery::new("app_token", "table_id")
            .or()
            .field_names(vec!["状态".to_string(), "负责人".to_string()])
            .automatic_fields(false)
            .where_in("状态", vec!["已完成".to_string(), "已归档".to_string()]);

        let (_, _, field_names, automatic_fields, filter) = query.into_parts();
        let filter = filter.expect("filter should exist");
        let condition = filter.conditions.as_ref().unwrap().first().unwrap();

        assert_eq!(field_names.unwrap().len(), 2);
        assert!(!automatic_fields);
        assert_eq!(filter.conjunction.as_deref(), Some("or"));
        assert_eq!(condition.operator, "isAnyOf");
        assert_eq!(
            condition.value.as_ref().expect("values should exist"),
            &vec!["已完成".to_string(), "已归档".to_string()]
        );
    }

    #[cfg(feature = "ccm-core")]
    #[test]
    fn test_sheet_range_builds_a1_notation() {
        let range = SheetRange::from_range_expr("sheet_001", "A1:C5").unwrap();

        assert_eq!(range.sheet_id, "sheet_001");
        assert_eq!(range.start_cell, "A1");
        assert_eq!(range.end_cell.as_deref(), Some("C5"));
        assert_eq!(range.to_string(), "sheet_001!A1:C5");
    }

    #[cfg(feature = "ccm-core")]
    #[test]
    fn test_sheet_range_parses_single_cell_notation() {
        let range = SheetRange::parse("sheet_001!B2").unwrap();

        assert_eq!(range.sheet_id, "sheet_001");
        assert_eq!(range.start_cell, "B2");
        assert!(range.end_cell.is_none());
        assert_eq!(range.range_expr(), "B2");
    }

    #[cfg(feature = "ccm-core")]
    #[test]
    fn test_sheet_range_rejects_embedded_sheet_prefix() {
        let error = SheetRange::from_range_expr("sheet_001", "sheet_002!A1:C5").unwrap_err();
        assert!(error.to_string().contains("range_expr"));
    }

    #[cfg(feature = "ccm-core")]
    #[test]
    fn test_sheet_write_range_maps_to_batch_write_data() {
        let write = SheetWriteRange::new(
            SheetRange::from_range_expr("sheet_001", "A1:B2").unwrap(),
            vec![vec![serde_json::json!("A1"), serde_json::json!("B1")]],
        )
        .major_dimension("ROWS");

        let batch: crate::ccm::sheets_v2::v2::data_io::models::BatchWriteData = write.into();
        assert_eq!(batch.data_range, "sheet_001!A1:B2");
        assert_eq!(batch.major_dimension, "ROWS");
        assert_eq!(batch.values.len(), 1);
    }

    #[cfg(feature = "ccm-core")]
    #[test]
    fn test_folder_children_page_maps_next_page_token() {
        let data = crate::ccm::explorer::v2::models::FolderChildrenData {
            items: vec![crate::ccm::explorer::v2::models::FileItem {
                file_token: "folder_a".to_string(),
                title: "Alpha".to_string(),
                doc_type: "folder".to_string(),
                is_folder: true,
                create_time: 1,
                update_time: 2,
            }],
            has_more: true,
            page_token: Some("page_2".to_string()),
        };

        let page: FolderChildrenPage = TypedPage::from(data);
        assert_eq!(page.items.len(), 1);
        assert_eq!(page.items[0].title, "Alpha");
        assert!(page.has_more);
        assert_eq!(page.next_page_token.as_deref(), Some("page_2"));
    }

    #[cfg(feature = "ccm-core")]
    #[test]
    fn test_folder_children_pager_resume_token() {
        let client = DocsClient::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );

        let pager = client
            .folder_children_pager("folder_token")
            .page_size(999)
            .doc_type("folder")
            .next_page_token("page_2");

        assert_eq!(pager.pending_page_token(), Some("page_2"));
    }
}
