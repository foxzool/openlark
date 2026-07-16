//! Bitable API V1 端点枚举
//!
//! 提供 Bitable v1 的端点定义，支持 method、path 和认证要求的统一管理（#439 迁移）。
//! 从 api_endpoints.rs 提取为独立子模块，避免主文件持续膨胀。

use super::CatalogEndpoint;
use openlark_core::api::{ApiRequest, HttpMethod};
use openlark_core::constants::AccessTokenType;

/// Bitable API V1 端点枚举（#424 深化了请求语义：method + path + auth 在此集中）。
#[derive(Debug, Clone, PartialEq)]
pub enum BitableApiV1 {
    /// App管理相关
    AppCreate,
    /// 复制多维表格应用（参数：app_token）
    AppCopy(String),
    /// 获取多维表格应用信息（参数：app_token）
    AppGet(String),
    /// 更新多维表格应用信息（参数：app_token）
    AppUpdate(String),
    /// 列出多维表格仪表盘（参数：app_token）
    DashboardList(String),
    /// 复制仪表盘（参数：app_token, block_id）
    DashboardCopy(String, String),
    /// 自动化流程
    BlockWorkflowList(String),
    /// 列出自动化工作流（参数：app_token）
    WorkflowList(String),
    /// 更新自动化工作流（参数：app_token, workflow_id）
    WorkflowUpdate(String, String),

    /// 表格管理相关
    TableCreate(String),
    /// 批量创建数据表（参数：app_token）
    TableBatchCreate(String),
    /// 更新数据表（参数：app_token, table_id）
    TableUpdate(String, String),
    /// 删除数据表（参数：app_token, table_id）
    TableDelete(String, String),
    /// 批量删除数据表（参数：app_token）
    TableBatchDelete(String),
    /// 获取数据表信息（参数：app_token, table_id）
    TableGet(String, String),
    /// 列出数据表（参数：app_token）
    TableList(String),
    /// 增量更新数据表（参数：app_token, table_id）
    TablePatch(String, String),

    /// 字段管理相关
    FieldCreate(String, String),
    /// 创建字段分组（参数：app_token, table_id）
    FieldGroupCreate(String, String),
    /// 更新字段（参数：app_token, table_id, field_id）
    FieldUpdate(String, String, String),
    /// 删除字段（参数：app_token, table_id, field_id）
    FieldDelete(String, String, String),
    /// 列出字段（参数：app_token, table_id）
    FieldList(String, String),

    /// 视图管理相关
    ViewCreate(String, String),
    /// 更新视图（参数：app_token, table_id, view_id）
    ViewUpdate(String, String, String),
    /// 删除视图（参数：app_token, table_id, view_id）
    ViewDelete(String, String, String),
    /// 获取视图（参数：app_token, table_id, view_id）
    ViewGet(String, String, String),
    /// 列出视图（参数：app_token, table_id）
    ViewList(String, String),
    /// 增量更新视图（参数：app_token, table_id, view_id）
    ViewPatch(String, String, String),

    /// 记录管理相关
    RecordCreate(String, String),
    /// 批量创建记录（参数：app_token, table_id）
    RecordBatchCreate(String, String),
    /// 获取记录（参数：app_token, table_id, record_id）
    RecordGet(String, String, String),
    /// 批量获取记录（参数：app_token, table_id）
    RecordBatchGet(String, String),
    /// 更新记录（参数：app_token, table_id, record_id）
    RecordUpdate(String, String, String),
    /// 批量更新记录（参数：app_token, table_id）
    RecordBatchUpdate(String, String),
    /// 删除记录（参数：app_token, table_id, record_id）
    RecordDelete(String, String, String),
    /// 批量删除记录（参数：app_token, table_id）
    RecordBatchDelete(String, String),
    /// 列出记录（参数：app_token, table_id）
    RecordList(String, String),
    /// 查询记录（参数：app_token, table_id）
    RecordSearch(String, String),

    /// 表单管理相关
    FormGet(String, String, String),
    /// 更新表单（参数：app_token, table_id, form_id）
    FormPatch(String, String, String),
    /// 升级表单（参数：app_token, table_id, form_id）
    FormUpgrade(String, String, String),
    /// 列出表单字段（参数：app_token, table_id, form_id）
    FormFieldList(String, String, String),
    /// 更新表单字段（参数：app_token, table_id, form_id, field_id）
    FormFieldPatch(String, String, String, String),

    /// 权限管理相关
    RoleCreate(String),
    /// 更新自定义角色（参数：app_token, role_id）
    RoleUpdate(String, String),
    /// 删除自定义角色（参数：app_token, role_id）
    RoleDelete(String, String),
    /// 列出自定义角色（参数：app_token）
    RoleList(String),
    /// 新增角色成员（参数：app_token, role_id）
    RoleMemberCreate(String, String),
    /// 批量新增角色成员（参数：app_token, role_id）
    RoleMemberBatchCreate(String, String),
    /// 删除角色成员（参数：app_token, role_id, member_id）
    RoleMemberDelete(String, String, String),
    /// 批量删除角色成员（参数：app_token, role_id）
    RoleMemberBatchDelete(String, String),
    /// 列出角色成员（参数：app_token, role_id）
    RoleMemberList(String, String),
}

impl BitableApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            // App管理
            BitableApiV1::AppCreate => "/open-apis/bitable/v1/apps".to_string(),
            BitableApiV1::AppCopy(app_token) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/copy")
            }
            BitableApiV1::AppGet(app_token) => {
                format!("/open-apis/bitable/v1/apps/{app_token}")
            }
            BitableApiV1::AppUpdate(app_token) => {
                format!("/open-apis/bitable/v1/apps/{app_token}")
            }
            BitableApiV1::DashboardList(app_token) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/dashboards")
            }
            BitableApiV1::DashboardCopy(app_token, block_id) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/dashboards/{block_id}/copy")
            }
            BitableApiV1::BlockWorkflowList(app_token) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/block_workflows")
            }
            BitableApiV1::WorkflowList(app_token) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/workflows")
            }
            BitableApiV1::WorkflowUpdate(app_token, workflow_id) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/workflows/{workflow_id}")
            }

            // 表格管理
            BitableApiV1::TableCreate(app_token) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/tables")
            }
            BitableApiV1::TableBatchCreate(app_token) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/tables/batch_create")
            }
            BitableApiV1::TableUpdate(app_token, table_id) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}")
            }
            BitableApiV1::TableDelete(app_token, table_id) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}")
            }
            BitableApiV1::TableBatchDelete(app_token) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/tables/batch_delete")
            }
            BitableApiV1::TableGet(app_token, table_id) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}")
            }
            BitableApiV1::TableList(app_token) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/tables")
            }
            BitableApiV1::TablePatch(app_token, table_id) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}")
            }

            // 字段管理
            BitableApiV1::FieldCreate(app_token, table_id) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields")
            }
            BitableApiV1::FieldGroupCreate(app_token, table_id) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/field_groups")
            }
            BitableApiV1::FieldUpdate(app_token, table_id, field_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields/{field_id}"
                )
            }
            BitableApiV1::FieldDelete(app_token, table_id, field_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields/{field_id}"
                )
            }
            BitableApiV1::FieldList(app_token, table_id) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields")
            }

            // 视图管理
            BitableApiV1::ViewCreate(app_token, table_id) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views")
            }
            BitableApiV1::ViewUpdate(app_token, table_id, view_id) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views/{view_id}")
            }
            BitableApiV1::ViewDelete(app_token, table_id, view_id) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views/{view_id}")
            }
            BitableApiV1::ViewGet(app_token, table_id, view_id) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views/{view_id}")
            }
            BitableApiV1::ViewList(app_token, table_id) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views")
            }
            BitableApiV1::ViewPatch(app_token, table_id, view_id) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views/{view_id}")
            }

            // 记录管理
            BitableApiV1::RecordCreate(app_token, table_id) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records")
            }
            BitableApiV1::RecordBatchCreate(app_token, table_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_create"
                )
            }
            BitableApiV1::RecordGet(app_token, table_id, record_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/{record_id}"
                )
            }
            BitableApiV1::RecordBatchGet(app_token, table_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_get"
                )
            }
            BitableApiV1::RecordUpdate(app_token, table_id, record_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/{record_id}"
                )
            }
            BitableApiV1::RecordBatchUpdate(app_token, table_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_update"
                )
            }
            BitableApiV1::RecordDelete(app_token, table_id, record_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/{record_id}"
                )
            }
            BitableApiV1::RecordBatchDelete(app_token, table_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_delete"
                )
            }
            BitableApiV1::RecordList(app_token, table_id) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records")
            }
            BitableApiV1::RecordSearch(app_token, table_id) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/search")
            }

            // 表单管理
            BitableApiV1::FormGet(app_token, table_id, form_id) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/forms/{form_id}")
            }
            BitableApiV1::FormPatch(app_token, table_id, form_id) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/forms/{form_id}")
            }
            BitableApiV1::FormUpgrade(app_token, table_id, form_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/forms/{form_id}/upgrade"
                )
            }
            BitableApiV1::FormFieldList(app_token, table_id, form_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/forms/{form_id}/fields"
                )
            }
            BitableApiV1::FormFieldPatch(app_token, table_id, form_id, field_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/forms/{form_id}/fields/{field_id}"
                )
            }

            // 权限管理
            BitableApiV1::RoleCreate(app_token) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/roles")
            }
            BitableApiV1::RoleUpdate(app_token, role_id) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}")
            }
            BitableApiV1::RoleDelete(app_token, role_id) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}")
            }
            BitableApiV1::RoleList(app_token) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/roles")
            }
            BitableApiV1::RoleMemberCreate(app_token, role_id) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members")
            }
            BitableApiV1::RoleMemberBatchCreate(app_token, role_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members/batch_create"
                )
            }
            BitableApiV1::RoleMemberDelete(app_token, role_id, member_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members/{member_id}"
                )
            }
            BitableApiV1::RoleMemberBatchDelete(app_token, role_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members/batch_delete"
                )
            }
            BitableApiV1::RoleMemberList(app_token, role_id) => {
                format!("/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members")
            }
        }
    }

    /// 返回配置了正确 HTTP 方法的 ApiRequest（委托到 CatalogEndpoint trait）。
    pub fn to_request<R>(&self) -> ApiRequest<R> {
        <Self as CatalogEndpoint>::to_request(self)
    }
}

impl CatalogEndpoint for BitableApiV1 {
    fn to_url(&self) -> String {
        // delegate to inherent for backward compat
        BitableApiV1::to_url(self)
    }

    fn method(&self) -> HttpMethod {
        match self {
            // App 管理
            BitableApiV1::AppCreate => HttpMethod::Post,
            BitableApiV1::AppCopy(_) => HttpMethod::Post,
            BitableApiV1::AppGet(_) => HttpMethod::Get,
            BitableApiV1::AppUpdate(_) => HttpMethod::Put,
            BitableApiV1::DashboardList(_) => HttpMethod::Get,
            BitableApiV1::DashboardCopy(_, _) => HttpMethod::Post,
            BitableApiV1::BlockWorkflowList(_) => HttpMethod::Get,
            BitableApiV1::WorkflowList(_) => HttpMethod::Get,
            BitableApiV1::WorkflowUpdate(_, _) => HttpMethod::Put,

            // 表格管理
            BitableApiV1::TableCreate(_) => HttpMethod::Post,
            BitableApiV1::TableBatchCreate(_) => HttpMethod::Post,
            BitableApiV1::TableUpdate(_, _) => HttpMethod::Put,
            BitableApiV1::TableDelete(_, _) => HttpMethod::Delete,
            BitableApiV1::TableBatchDelete(_) => HttpMethod::Post,
            BitableApiV1::TableGet(_, _) => HttpMethod::Get,
            BitableApiV1::TableList(_) => HttpMethod::Get,
            BitableApiV1::TablePatch(_, _) => HttpMethod::Patch,

            // 字段管理
            BitableApiV1::FieldCreate(_, _) => HttpMethod::Post,
            BitableApiV1::FieldGroupCreate(_, _) => HttpMethod::Post,
            BitableApiV1::FieldUpdate(_, _, _) => HttpMethod::Put,
            BitableApiV1::FieldDelete(_, _, _) => HttpMethod::Delete,
            BitableApiV1::FieldList(_, _) => HttpMethod::Get,

            // 视图管理
            BitableApiV1::ViewCreate(_, _) => HttpMethod::Post,
            BitableApiV1::ViewUpdate(_, _, _) => HttpMethod::Put,
            BitableApiV1::ViewDelete(_, _, _) => HttpMethod::Delete,
            BitableApiV1::ViewGet(_, _, _) => HttpMethod::Get,
            BitableApiV1::ViewList(_, _) => HttpMethod::Get,
            BitableApiV1::ViewPatch(_, _, _) => HttpMethod::Patch,

            // 记录管理
            BitableApiV1::RecordCreate(_, _) => HttpMethod::Post,
            BitableApiV1::RecordBatchCreate(_, _) => HttpMethod::Post,
            BitableApiV1::RecordGet(_, _, _) => HttpMethod::Get,
            BitableApiV1::RecordBatchGet(_, _) => HttpMethod::Post,
            BitableApiV1::RecordUpdate(_, _, _) => HttpMethod::Put,
            BitableApiV1::RecordBatchUpdate(_, _) => HttpMethod::Post,
            BitableApiV1::RecordDelete(_, _, _) => HttpMethod::Delete,
            BitableApiV1::RecordBatchDelete(_, _) => HttpMethod::Post,
            BitableApiV1::RecordList(_, _) => HttpMethod::Get,
            BitableApiV1::RecordSearch(_, _) => HttpMethod::Post,

            // 表单管理
            BitableApiV1::FormGet(_, _, _) => HttpMethod::Get,
            BitableApiV1::FormPatch(_, _, _) => HttpMethod::Patch,
            BitableApiV1::FormUpgrade(_, _, _) => HttpMethod::Post,
            BitableApiV1::FormFieldList(_, _, _) => HttpMethod::Get,
            BitableApiV1::FormFieldPatch(_, _, _, _) => HttpMethod::Patch,

            // 权限/角色管理
            BitableApiV1::RoleCreate(_) => HttpMethod::Post,
            BitableApiV1::RoleUpdate(_, _) => HttpMethod::Put,
            BitableApiV1::RoleDelete(_, _) => HttpMethod::Delete,
            BitableApiV1::RoleList(_) => HttpMethod::Get,
            BitableApiV1::RoleMemberCreate(_, _) => HttpMethod::Post,
            BitableApiV1::RoleMemberBatchCreate(_, _) => HttpMethod::Post,
            BitableApiV1::RoleMemberDelete(_, _, _) => HttpMethod::Delete,
            BitableApiV1::RoleMemberBatchDelete(_, _) => HttpMethod::Post,
            BitableApiV1::RoleMemberList(_, _) => HttpMethod::Get,
        }
    }

    /// Bitable 端点的稳定认证要求（#439 迁移）。
    /// 与 Base 一致，显式声明以便语义目录统一拥有认证要求。
    fn supported_access_token_types(&self) -> Option<Vec<AccessTokenType>> {
        Some(vec![AccessTokenType::User, AccessTokenType::Tenant])
    }

    // to_request 使用 trait 默认实现
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::api::{ApiRequest, HttpMethod};
    use openlark_core::constants::AccessTokenType;

    #[test]
    fn test_bitable_api_v1_app_create() {
        let endpoint = BitableApiV1::AppCreate;
        assert_eq!(endpoint.to_url(), "/open-apis/bitable/v1/apps");
    }

    #[test]
    fn test_bitable_api_v1_app_copy() {
        let endpoint = BitableApiV1::AppCopy("app_token_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/bitable/v1/apps/app_token_123/copy"
        );
    }

    #[test]
    fn test_bitable_api_v1_table_create() {
        let endpoint = BitableApiV1::TableCreate("app_token_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/bitable/v1/apps/app_token_123/tables"
        );
    }

    #[test]
    fn test_bitable_api_v1_record_create() {
        let endpoint =
            BitableApiV1::RecordCreate("app_token_123".to_string(), "table_id_456".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/bitable/v1/apps/app_token_123/tables/table_id_456/records"
        );
    }

    #[test]
    fn test_bitable_api_v1_field_create() {
        let endpoint =
            BitableApiV1::FieldCreate("app_token_123".to_string(), "table_id_456".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/bitable/v1/apps/app_token_123/tables/table_id_456/fields"
        );
    }

    #[test]
    fn test_bitable_api_v1_block_workflow_list() {
        let endpoint = BitableApiV1::BlockWorkflowList("app_token_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/bitable/v1/apps/app_token_123/block_workflows"
        );
    }

    #[test]
    fn test_bitable_api_v1_field_group_create() {
        let endpoint =
            BitableApiV1::FieldGroupCreate("app_token_123".to_string(), "table_id_456".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/bitable/v1/apps/app_token_123/tables/table_id_456/field_groups"
        );
    }

    #[test]
    fn test_bitable_api_v1_form_upgrade() {
        let endpoint = BitableApiV1::FormUpgrade(
            "app_token_123".to_string(),
            "table_id_456".to_string(),
            "form_id_789".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/bitable/v1/apps/app_token_123/tables/table_id_456/forms/form_id_789/upgrade"
        );
    }

    #[test]
    fn test_bitable_api_v1_view_create() {
        let endpoint =
            BitableApiV1::ViewCreate("app_token_123".to_string(), "table_id_456".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/bitable/v1/apps/app_token_123/tables/table_id_456/views"
        );
    }

    #[test]
    fn test_bitable_api_v1_form_get() {
        let endpoint = BitableApiV1::FormGet(
            "app_token_123".to_string(),
            "table_id_456".to_string(),
            "form_id_789".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/bitable/v1/apps/app_token_123/tables/table_id_456/forms/form_id_789"
        );
    }

    #[test]
    fn test_bitable_api_v1_role_member_create() {
        let endpoint =
            BitableApiV1::RoleMemberCreate("app_token_123".to_string(), "role_id_456".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/bitable/v1/apps/app_token_123/roles/role_id_456/members"
        );
    }

    #[test]
    fn test_bitable_api_v1_batch_operations() {
        let endpoint = BitableApiV1::TableBatchCreate("app_token_123".to_string());
        assert!(endpoint.to_url().contains("batch_create"));

        let endpoint = BitableApiV1::RecordBatchDelete(
            "app_token_123".to_string(),
            "table_id_456".to_string(),
        );
        assert!(endpoint.to_url().contains("batch_delete"));
    }

    // ========== #424 / #439: 端点目录语义测试（method + path + auth） ==========
    #[test]
    fn test_bitable_record_endpoints_semantics_424() {
        // Create / BatchCreate -> POST
        let ep = BitableApiV1::RecordCreate("app".into(), "tbl".into());
        assert_eq!(ep.method(), HttpMethod::Post);
        assert!(ep.to_url().contains("/records"));
        let req: ApiRequest<()> = ep.to_request();
        assert_eq!(req.method(), &HttpMethod::Post);

        let ep = BitableApiV1::RecordBatchCreate("app".into(), "tbl".into());
        assert_eq!(ep.method(), HttpMethod::Post);

        // Get / List -> GET
        let ep = BitableApiV1::RecordGet("app".into(), "tbl".into(), "rec".into());
        assert_eq!(ep.method(), HttpMethod::Get);
        let req: ApiRequest<()> = ep.to_request();
        assert_eq!(req.method(), &HttpMethod::Get);

        let ep = BitableApiV1::RecordList("app".into(), "tbl".into());
        assert_eq!(ep.method(), HttpMethod::Get);

        // Update -> PUT , BatchUpdate -> POST
        let ep = BitableApiV1::RecordUpdate("app".into(), "tbl".into(), "rec".into());
        assert_eq!(ep.method(), HttpMethod::Put);

        let ep = BitableApiV1::RecordBatchUpdate("app".into(), "tbl".into());
        assert_eq!(ep.method(), HttpMethod::Post);

        // Delete -> DELETE , BatchDelete -> POST
        let ep = BitableApiV1::RecordDelete("app".into(), "tbl".into(), "rec".into());
        assert_eq!(ep.method(), HttpMethod::Delete);
        let req: ApiRequest<()> = ep.to_request();
        assert_eq!(req.method(), &HttpMethod::Delete);

        let ep = BitableApiV1::RecordBatchDelete("app".into(), "tbl".into());
        assert_eq!(ep.method(), HttpMethod::Post);

        // Search -> POST (body query)
        let ep = BitableApiV1::RecordSearch("app".into(), "tbl".into());
        assert_eq!(ep.method(), HttpMethod::Post);
        assert!(ep.to_url().contains("/search"));

        // auth: 显式声明在 catalog 中（#439）
        let ep = BitableApiV1::RecordCreate("a".into(), "t".into());
        assert_eq!(
            ep.supported_access_token_types(),
            Some(vec![AccessTokenType::User, AccessTokenType::Tenant])
        );
    }
}
