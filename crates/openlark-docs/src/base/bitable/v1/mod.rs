//! 多维表格 v1 接口模块，聚合 app 与字段类型定义及导出。

/// app 子模块。
pub mod app;
mod field_types;

// 使用通配符导出所有子模块
// app 模块显式导出
/// 重新导出相关类型。
pub use app::{
    App, AppService, AppSettings, BatchCreateRecordRequest, BatchCreateRecordResponse,
    BatchCreateRoleMemberRequest, BatchCreateRoleMemberResponse, BatchCreateTableRequest,
    BatchCreateTableResponse, BatchDeleteRecordRequest, BatchDeleteRecordResponse,
    BatchDeleteRoleMemberRequest, BatchDeleteRoleMemberResponse, BatchDeleteTableRequest,
    BatchDeleteTableResponse, BatchGetRecordRequest, BatchGetRecordResponse,
    BatchUpdateRecordRequest, BatchUpdateRecordResponse, BlockRole, CopyAppRequest,
    CopyAppResponse, CopyDashboardRequest, CopyDashboardResponse, CreateAppRequest,
    CreateAppResponse, CreateAppRoleRequest, CreateAppRoleRequestBody, CreateAppRoleResponse,
    CreateFieldRequest, CreateFieldResponse, CreateRecordItem, CreateRecordRequest,
    CreateRecordResponse, CreateRoleMemberRequest, CreateRoleMemberResponse, CreateTableRequest,
    CreateTableResponse, CreateViewData, CreateViewRequest, CreateViewResponse, DeleteAppResponse,
    DeleteAppRoleRequest, DeleteAppRoleResponse, DeleteFieldRequest, DeleteFieldResponse,
    DeleteRecordRequest, DeleteRecordResponse, DeleteRoleMemberRequest, DeleteRoleMemberResponse,
    DeleteTableRequest, DeleteTableResponse, DeleteViewRequest, DeleteViewResponse, DeletedRecord,
    Field, FieldDescription, FieldProperty, FieldType, FilterCondition, FilterInfo, Form,
    FormFieldQuestion, GetAppRequest, GetAppResponse, GetFormRequest, GetFormResponse,
    GetRecordRequest, GetRecordResponse, GetViewRequest, GetViewResponse, ListAppRoleRequest,
    ListAppRoleResponse, ListDashboardsRequest, ListDashboardsResponse, ListFieldRequest,
    ListFieldResponse, ListFormFieldQuestionRequest, ListFormFieldQuestionResponse,
    ListRecordRequest, ListRecordResponse, ListRoleMembersRequest, ListRoleMembersResponse,
    ListTablesRequest, ListTablesResponse, ListViewsRequest, ListViewsResponse,
    ListWorkflowRequest, ListWorkflowResponse, PatchFormFieldQuestionRequest,
    PatchFormFieldQuestionRequestBuilder, PatchFormFieldQuestionResponse, PatchFormFieldRequest,
    PatchFormRequest, PatchFormResponse, PatchTableRequest, PatchTableResponse, PatchViewData,
    PatchViewRequest, PatchViewResponse, PatchedFormFieldQuestion, Person, Record, Role,
    RoleMemberId, RoleMemberIdType, RoleMemberInfo, RoleMemberType, SearchRecordRequest,
    SearchRecordRequestBody, SearchRecordResponse, SortCondition, TableData, TableField, TableInfo,
    TableRole, UpdateAppRequest, UpdateAppResponse, UpdateAppRoleRequest, UpdateAppRoleRequestBody,
    UpdateAppRoleResponse, UpdateFieldRequest, UpdateFieldResponse, UpdateRecordItem,
    UpdateRecordRequest, UpdateRecordResponse, UpdateWorkflowBody, UpdateWorkflowRequest,
    UpdateWorkflowResponse, View, Workflow, WorkflowStatus,
};

// 旧名兼容别名（deprecated alias，v1.0 移除）
#[allow(deprecated)]
pub use app::PatchFormFieldQuestionBuilder;
// field_types 模块显式导出
/// 重新导出相关类型。
pub use field_types::{
    AttachmentInfo, AutoNumberInfo, KanbanItem, LinkInfo, LocationInfo, MentionInfo, PeopleInfo,
    ProgressInfo, RecordFieldValue, RecordFields, RecordFieldsBuilder, TextSegment, TextStyle,
};
