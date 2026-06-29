//! 多维表格能力模块，聚合 bitable v1 接口与常用类型导出。

/// 多维表格服务模块
///
/// 提供多维表格应用、数据表、视图管理等功能。
pub mod v1;

// 使用通配符导出所有子模块,避免维护大量重复的导出列表
// v1 模块显式导出
pub use v1::{
    App, AppService, AppSettings, AttachmentInfo, AutoNumberInfo, BatchCreateRecordRequest,
    BatchCreateRecordResponse, BatchCreateRoleMemberRequest, BatchCreateRoleMemberResponse,
    BatchCreateTableRequest, BatchCreateTableResponse, BatchDeleteRecordRequest,
    BatchDeleteRecordResponse, BatchDeleteRoleMemberRequest, BatchDeleteRoleMemberResponse,
    BatchDeleteTableRequest, BatchDeleteTableResponse, BatchGetRecordRequest,
    BatchGetRecordResponse, BatchUpdateRecordRequest, BatchUpdateRecordResponse, BlockRole,
    CopyAppRequest, CopyAppResponse, CopyDashboardRequest, CopyDashboardResponse, CreateAppRequest,
    CreateAppResponse, CreateAppRoleRequest, CreateAppRoleRequestBody, CreateAppRoleResponse,
    CreateFieldRequest, CreateFieldResponse, CreateRecordItem, CreateRecordRequest,
    CreateRecordResponse, CreateRoleMemberRequest, CreateRoleMemberResponse, CreateTableRequest,
    CreateTableResponse, CreateViewData, CreateViewRequest, CreateViewResponse, DeleteAppResponse,
    DeleteAppRoleRequest, DeleteAppRoleResponse, DeleteFieldRequest, DeleteFieldResponse,
    DeleteRecordRequest, DeleteRecordResponse, DeleteRoleMemberRequest, DeleteRoleMemberResponse,
    DeleteTableRequest, DeleteTableResponse, DeleteViewRequest, DeleteViewResponse, DeletedRecord,
    Field, FieldDescription, FieldProperty, FieldType, FilterCondition, FilterInfo, Form,
    FormFieldQuestion, GetAppRequest, GetAppResponse, GetFormRequest, GetFormResponse,
    GetRecordRequest, GetRecordResponse, GetViewRequest, GetViewResponse, KanbanItem, LinkInfo,
    ListAppRoleRequest, ListAppRoleResponse, ListDashboardsRequest, ListDashboardsResponse,
    ListFieldRequest, ListFieldResponse, ListFormFieldQuestionRequest,
    ListFormFieldQuestionResponse, ListRecordRequest, ListRecordResponse, ListRoleMembersRequest,
    ListRoleMembersResponse, ListTablesRequest, ListTablesResponse, ListViewsRequest,
    ListViewsResponse, ListWorkflowRequest, ListWorkflowResponse, LocationInfo, MentionInfo,
    PatchFormFieldQuestionRequest, PatchFormFieldQuestionRequestBuilder,
    PatchFormFieldQuestionResponse, PatchFormFieldRequest, PatchFormRequest, PatchFormResponse,
    PatchTableRequest, PatchTableResponse, PatchViewData, PatchViewRequest, PatchViewResponse,
    PatchedFormFieldQuestion, PeopleInfo, Person, ProgressInfo, Record, RecordFieldValue,
    RecordFields, RecordFieldsBuilder, Role, RoleMemberId, RoleMemberIdType, RoleMemberInfo,
    RoleMemberType, SearchRecordRequest, SearchRecordRequestBody, SearchRecordResponse,
    SortCondition, TableData, TableField, TableInfo, TableRole, TextSegment, TextStyle,
    UpdateAppRequest, UpdateAppResponse, UpdateAppRoleRequest, UpdateAppRoleRequestBody,
    UpdateAppRoleResponse, UpdateFieldRequest, UpdateFieldResponse, UpdateRecordItem,
    UpdateRecordRequest, UpdateRecordResponse, UpdateWorkflowBody, UpdateWorkflowRequest,
    UpdateWorkflowResponse, View, Workflow, WorkflowStatus,
};
// 旧名兼容别名（deprecated alias，v1.0 移除）
#[allow(deprecated)]
pub use v1::PatchFormFieldQuestionBuilder;
