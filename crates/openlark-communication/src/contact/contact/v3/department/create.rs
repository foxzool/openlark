//! 创建部门
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/department/create>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, error, http::Transport, validate_required,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::serialize_params,
    contact::contact::v3::{
        department::models::{DepartmentI18nName, DepartmentResponse},
        user::models::{DepartmentIdType, UserIdType},
    },
    endpoints::CONTACT_V3_DEPARTMENTS,
};

/// 部门负责人信息（官方 departmentLeader 结构，字段必填）
///
/// JSON key 为 `leaderType` / `leaderID`（官方文档示例如此，非 snake_case）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DepartmentLeader {
    /// 负责人类型：1-主负责人，2-副负责人
    #[serde(rename = "leaderType")]
    pub leader_type: i32,
    /// 负责人的用户 ID，ID 类型与查询参数 user_id_type 的取值保持一致
    #[serde(rename = "leaderID")]
    pub leader_id: String,
}

/// 创建部门请求体
///
/// 按官方文档建模全部字段。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDepartmentBody {
    /// 部门名称
    pub name: String,
    /// 父部门 ID
    pub parent_department_id: String,
    /// 自定义部门 ID（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 部门名称的国际化配置（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<DepartmentI18nName>,
    /// 部门主管的用户 ID（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader_user_id: Option<String>,
    /// 部门的排序，String 类型的非负整数（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    /// 部门绑定的单位自定义 ID 列表（可选，当前只支持绑定一个单位）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_ids: Option<Vec<String>>,
    /// 是否创建部门群（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_group_chat: Option<bool>,
    /// 部门负责人信息（可选；配置时必须指定一名主负责人）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaders: Option<Vec<DepartmentLeader>>,
    /// 部门群的人员类型限制（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_chat_employee_types: Option<Vec<i32>>,
    /// 部门 HRBP 的用户 ID 列表（可选，最多 500 个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_hrbps: Option<Vec<String>>,
}

impl CreateDepartmentBody {
    /// 创建新的部门请求体。
    pub fn new(name: impl Into<String>, parent_department_id: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            parent_department_id: parent_department_id.into(),
            department_id: None,
            i18n_name: None,
            leader_user_id: None,
            order: None,
            unit_ids: None,
            create_group_chat: None,
            leaders: None,
            group_chat_employee_types: None,
            department_hrbps: None,
        }
    }
}

/// 创建部门请求
///
/// 用于在通讯录中创建新部门。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `user_id_type`: 用户 ID 类型（可选）
/// - `department_id_type`: 部门 ID 类型（可选）
/// - `client_token`: 幂等 token（可选）
///
/// # 请求体字段
///
/// - `name`: 部门名称，必填
/// - `parent_department_id`: 父部门 ID，必填
/// - `department_id`: 自定义部门 ID（可选）
/// - `i18n_name`: 部门名称的国际化配置（可选）
/// - `leader_user_id`: 部门主管的用户 ID（可选）
/// - `order`: 部门的排序（可选）
/// - `unit_ids`: 部门绑定的单位自定义 ID 列表（可选）
/// - `create_group_chat`: 是否创建部门群（可选）
/// - `leaders`: 部门负责人信息（可选）
/// - `group_chat_employee_types`: 部门群的人员类型限制（可选）
/// - `department_hrbps`: 部门 HRBP 的用户 ID 列表（可选）
///
/// # 示例
///
/// ```rust,ignore
/// let body = CreateDepartmentBody::new("研发部", "parent_dept_1");
/// let request = CreateDepartmentRequest::new(config)
///     .user_id_type(UserIdType::OpenId)
///     .client_token("uuid");
/// ```
pub struct CreateDepartmentRequest {
    config: Config,
    user_id_type: Option<UserIdType>,
    department_id_type: Option<DepartmentIdType>,
    client_token: Option<String>,
}

impl CreateDepartmentRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id_type: None,
            department_id_type: None,
            client_token: None,
        }
    }

    /// 用户 ID 类型（查询参数，可选）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 部门 ID 类型（查询参数，可选）
    pub fn department_id_type(mut self, department_id_type: DepartmentIdType) -> Self {
        self.department_id_type = Some(department_id_type);
        self
    }

    /// 幂等 token（查询参数，可选）
    pub fn client_token(mut self, client_token: impl Into<String>) -> Self {
        self.client_token = Some(client_token.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/department/create>
    pub async fn execute(self, body: CreateDepartmentBody) -> SDKResult<DepartmentResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: CreateDepartmentBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DepartmentResponse> {
        // === 必填字段验证 ===
        validate_required!(body.name, "name 不能为空");
        validate_required!(body.parent_department_id, "parent_department_id 不能为空");
        if body.name.contains('/') {
            return Err(error::validation_error(
                "name 不可包含 /".to_string(),
                "部门名称不可包含斜杠（/）".to_string(),
            ));
        }

        // url: POST:/open-apis/contact/v3/departments
        let mut req: ApiRequest<DepartmentResponse> =
            ApiRequest::post(CONTACT_V3_DEPARTMENTS).body(serialize_params(&body, "创建部门")?);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        if let Some(department_id_type) = self.department_id_type {
            req = req.query("department_id_type", department_id_type.as_str());
        }
        if let Some(client_token) = self.client_token {
            req = req.query("client_token", client_token);
        }

        Transport::request_typed(req, &self.config, Some(option), "创建部门").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_create_department_request_builder() {
        let config = Config::default();
        let request = CreateDepartmentRequest::new(config);
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_create_department_request_with_user_id_type() {
        let config = Config::default();
        let request = CreateDepartmentRequest::new(config).user_id_type(UserIdType::OpenId);
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_create_department_body_builder() {
        let body = CreateDepartmentBody::new("研发部", "parent_dept_1");
        assert_eq!(body.name, "研发部");
        assert_eq!(body.parent_department_id, "parent_dept_1");
    }

    #[test]
    fn test_create_department_body_with_custom_department_id() {
        let body = CreateDepartmentBody::new("产品部", "parent_dept_2");
        let body_with_id = CreateDepartmentBody {
            department_id: Some("custom_dept_id".to_string()),
            ..body
        };
        assert_eq!(
            body_with_id.department_id,
            Some("custom_dept_id".to_string())
        );
    }

    /// 全量可选字段序列化：嵌套 leaders 使用官方 key `leaderType`/`leaderID`，
    /// 可选字段为 None 时不输出。
    #[test]
    fn test_create_department_body_full_serialization() {
        let body = CreateDepartmentBody {
            i18n_name: Some(DepartmentI18nName {
                zh_cn: Some("技术部".to_string()),
                en_us: Some("Tech".to_string()),
                ja_jp: None,
                extra: HashMap::new(),
            }),
            leader_user_id: Some("ou_leader".to_string()),
            order: Some("100".to_string()),
            unit_ids: Some(vec!["custom_unit_id".to_string()]),
            create_group_chat: Some(true),
            leaders: Some(vec![DepartmentLeader {
                leader_type: 1,
                leader_id: "ou_leader".to_string(),
            }]),
            group_chat_employee_types: Some(vec![1, 2]),
            department_hrbps: Some(vec!["ou_hrbp".to_string()]),
            ..CreateDepartmentBody::new("研发部", "0")
        };

        let value = serde_json::to_value(&body).expect("序列化失败");
        assert_eq!(value["name"], "研发部");
        assert_eq!(value["parent_department_id"], "0");
        assert_eq!(value["i18n_name"]["zh_cn"], "技术部");
        assert_eq!(value["leader_user_id"], "ou_leader");
        assert_eq!(value["order"], "100");
        assert_eq!(value["unit_ids"][0], "custom_unit_id");
        assert_eq!(value["create_group_chat"], true);
        assert_eq!(value["leaders"][0]["leaderType"], 1);
        assert_eq!(value["leaders"][0]["leaderID"], "ou_leader");
        assert_eq!(value["group_chat_employee_types"][1], 2);
        assert_eq!(value["department_hrbps"][0], "ou_hrbp");
        // department_id 为 None 时不输出
        assert!(value.get("department_id").is_none());

        // roundtrip：反序列化保留全部字段
        let decoded: CreateDepartmentBody = serde_json::from_value(value).expect("反序列化失败");
        assert_eq!(
            decoded.leaders.as_deref(),
            Some(
                &[DepartmentLeader {
                    leader_type: 1,
                    leader_id: "ou_leader".to_string(),
                }][..]
            )
        );
        assert_eq!(decoded.create_group_chat, Some(true));
    }

    /// 最小请求体：只含两个必填字段。
    #[test]
    fn test_create_department_body_minimal_serialization() {
        let body = CreateDepartmentBody::new("研发部", "0");
        let value = serde_json::to_value(&body).expect("序列化失败");
        let obj = value.as_object().expect("应为 JSON 对象");
        assert_eq!(obj.len(), 2, "最小请求体只应包含 name/parent_department_id");
        assert_eq!(value["name"], "研发部");
        assert_eq!(value["parent_department_id"], "0");
    }

    #[test]
    fn test_create_department_request_with_all_options() {
        let config = Config::default();
        let request = CreateDepartmentRequest::new(config)
            .user_id_type(UserIdType::UnionId)
            .department_id_type(DepartmentIdType::DepartmentId)
            .client_token("uuid123");
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));
        assert_eq!(
            request.department_id_type,
            Some(DepartmentIdType::DepartmentId)
        );
        assert_eq!(request.client_token, Some("uuid123".to_string()));
    }
}
