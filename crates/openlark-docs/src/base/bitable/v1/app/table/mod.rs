/// batch_create 子模块。
pub mod batch_create;
/// batch_delete 子模块。
pub mod batch_delete;
/// create 子模块。
pub mod create;
/// delete 子模块。
pub mod delete;
/// field 子模块。
pub mod field;
/// field_group 子模块。
pub mod field_group;
/// form 子模块。
pub mod form;
/// 表格管理模块
///
/// 提供多维表格数据表的 CRUD 操作，包括：
/// - 单个/批量创建数据表
/// - 单个/批量删除数据表
/// - 更新数据表
/// - 列出数据表
/// - 字段管理（field 子模块）
/// - 记录管理（record 子模块）
/// - 表单管理（form 子模块）
/// - 视图管理（view 子模块）
pub mod list;
/// patch 子模块。
pub mod patch;
/// record 子模块。
pub mod record;
/// view 子模块。
pub mod view;

// 显式导出 - 避免使用 glob reexport
/// 重新导出相关类型。
pub use list::{ListTablesRequest, ListTablesResponse, TableInfo};

/// 重新导出相关类型。
pub use batch_create::{BatchCreateTableRequest, BatchCreateTableResponse};

/// 重新导出相关类型。
pub use create::{
    CreateTableRequest, CreateTableResponse, FieldDescription, TableData, TableField,
};

// 旧名兼容别名（deprecated alias，v1.0 移除）
#[allow(deprecated)]
pub use form::PatchFormFieldQuestionBuilder;

/// 重新导出相关类型。
pub use delete::{DeleteTableRequest, DeleteTableResponse};

/// 重新导出相关类型。
pub use patch::{PatchTableRequest, PatchTableResponse};

/// 重新导出相关类型。
pub use batch_delete::{BatchDeleteTableRequest, BatchDeleteTableResponse};

// 显式导出子模块类型
/// 重新导出相关类型。
pub use field::{
    CreateFieldRequest, CreateFieldResponse, DeleteFieldRequest, DeleteFieldResponse, Field,
    FieldProperty, FieldType, ListFieldRequest, ListFieldResponse, UpdateFieldRequest,
    UpdateFieldResponse,
};

/// 重新导出相关类型。
pub use field_group::{
    CreateFieldGroupItem, CreateFieldGroupRequest, CreateFieldGroupResponse, FieldGroupChild,
};

/// 重新导出相关类型。
pub use record::{
    BatchCreateRecordRequest, BatchCreateRecordResponse, BatchDeleteRecordRequest,
    BatchDeleteRecordResponse, BatchGetRecordRequest, BatchGetRecordResponse,
    BatchUpdateRecordRequest, BatchUpdateRecordResponse, CreateRecordItem, CreateRecordRequest,
    CreateRecordResponse, DeleteRecordRequest, DeleteRecordResponse, DeletedRecord,
    FilterCondition, FilterInfo, GetRecordRequest, GetRecordResponse, ListRecordRequest,
    ListRecordResponse, Person, Record, SearchRecordRequest, SearchRecordRequestBody,
    SearchRecordResponse, SortCondition, UpdateRecordItem, UpdateRecordRequest,
    UpdateRecordResponse,
};

/// 重新导出相关类型。
pub use form::{
    Form, FormFieldQuestion, GetFormRequest, GetFormResponse, ListFormFieldQuestionRequest,
    ListFormFieldQuestionResponse, PatchFormFieldQuestionRequest,
    PatchFormFieldQuestionRequestBuilder, PatchFormFieldQuestionResponse, PatchFormFieldRequest,
    PatchFormRequest, PatchFormResponse, PatchedFormFieldQuestion, UpgradeFormRequest,
    UpgradeFormResponse, UpgradedForm,
};

/// 重新导出相关类型。
pub use view::{
    CreateViewData, CreateViewRequest, CreateViewResponse, DeleteViewRequest, DeleteViewResponse,
    GetViewRequest, GetViewResponse, ListViewsRequest, ListViewsResponse, PatchViewData,
    PatchViewRequest, PatchViewResponse, View,
};
