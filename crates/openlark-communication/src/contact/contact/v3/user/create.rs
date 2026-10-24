//! 创建用户
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/user/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::serialize_params,
    contact::contact::v3::user::models::{DepartmentIdType, User, UserIdType},
    endpoints::CONTACT_V3_USERS,
};

/// 用户排序信息（官方 user_order 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserOrder {
    /// 排序信息对应的部门 ID（须在 department_ids 内）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 用户在其直属部门内的排序，数值越大排序越靠前
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_order: Option<i64>,
    /// 用户所属的多个部门之间的排序，数值越大排序越靠前
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_order: Option<i64>,
    /// 是否为用户的唯一主部门
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_primary_dept: Option<bool>,
}

/// 用户自定义字段取值中引用的人员（官方 custom_attr_generic_user 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomAttrGenericUser {
    /// 引用人员的用户 ID，ID 类型与查询参数 user_id_type 的取值保持一致
    pub id: String,
    /// 用户类型，目前仅支持 1（用户）
    pub r#type: i32,
}

/// 用户自定义字段取值（官方 user_custom_attr_value 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserCustomAttrValue {
    /// TEXT/HREF 类型时的字段值或网页标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// HREF 类型时的默认 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// HREF 类型时的 PC 端 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pc_url: Option<String>,
    /// ENUMERATION/PICTURE_ENUM 类型时的选项 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_id: Option<String>,
    /// GENERIC_USER 类型时的引用人员
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generic_user: Option<CustomAttrGenericUser>,
}

/// 用户自定义字段（官方 user_custom_attr 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserCustomAttr {
    /// 自定义字段类型：TEXT/HREF/ENUMERATION/PICTURE_ENUM/GENERIC_USER
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 自定义字段 ID（设置了 type 时必填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 自定义字段取值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<UserCustomAttrValue>,
}

/// 创建用户请求体
///
/// 按官方文档建模全部字段。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserBody {
    /// 用户名
    pub name: String,
    /// 手机号
    pub mobile: String,
    /// 用户所属部门 ID 列表
    pub department_ids: Vec<String>,
    /// 员工类型
    pub employee_type: i32,
    /// 自定义 user_id（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 英文名（可选，最长 255 字符）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    /// 别名（可选，最长 255 字符）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// 邮箱（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 手机号码是否对其他员工可见（可选，默认 true）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_visible: Option<bool>,
    /// 性别：0-保密 1-男 2-女 3-其他（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<i32>,
    /// 头像的文件 Key（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_key: Option<String>,
    /// 用户的直接主管的用户 ID（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader_user_id: Option<String>,
    /// 工作城市（可选，最长 100 字符）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// 国家或地区 Code 缩写（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// 工位（可选，最长 255 字符）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_station: Option<String>,
    /// 入职时间，秒级时间戳（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_time: Option<i64>,
    /// 工号（可选，最长 255 字符）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_no: Option<String>,
    /// 用户排序信息列表（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<UserOrder>>,
    /// 自定义字段（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_attrs: Option<Vec<UserCustomAttr>>,
    /// 企业邮箱（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_email: Option<String>,
    /// 职务名称（可选，最长 255 字符）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    /// 数据驻留地（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<String>,
    /// 职级 ID（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    /// 序列 ID（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    /// 分配给用户的席位 ID 列表（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_ids: Option<Vec<String>>,
    /// 虚线上级的用户 ID 列表（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dotted_line_leader_user_ids: Option<Vec<String>>,
}

impl CreateUserBody {
    /// 创建最小必填字段的请求体。
    pub fn new(
        name: impl Into<String>,
        mobile: impl Into<String>,
        department_ids: Vec<String>,
        employee_type: i32,
    ) -> Self {
        Self {
            name: name.into(),
            mobile: mobile.into(),
            department_ids,
            employee_type,
            user_id: None,
            en_name: None,
            nickname: None,
            email: None,
            mobile_visible: None,
            gender: None,
            avatar_key: None,
            leader_user_id: None,
            city: None,
            country: None,
            work_station: None,
            join_time: None,
            employee_no: None,
            orders: None,
            custom_attrs: None,
            enterprise_email: None,
            job_title: None,
            geo: None,
            job_level_id: None,
            job_family_id: None,
            subscription_ids: None,
            dotted_line_leader_user_ids: None,
        }
    }
}

/// 创建/更新/查询用户类接口通用响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    /// 用户详情。
    pub user: User,
}

impl ApiResponseTrait for UserResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建用户请求
///
/// 用于在通讯录中创建新用户。
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
/// - `name`: 用户名，必填
/// - `mobile`: 手机号，必填
/// - `department_ids`: 用户所属部门 ID 列表，必填
/// - `employee_type`: 员工类型，必填
/// - 其余 21 个可选字段（user_id/en_name/nickname/email/mobile_visible/gender/avatar_key/
///   leader_user_id/city/country/work_station/join_time/employee_no/orders/custom_attrs/
///   enterprise_email/job_title/geo/job_level_id/job_family_id/subscription_ids/
///   dotted_line_leader_user_ids）见 `CreateUserBody` 字段注释
///
/// # 示例
///
/// ```rust,ignore
/// let body = CreateUserBody::new(
///     "张三",
///     "13800138000",
///     vec!["dept_1".to_string()],
///     1,
/// );
/// let request = CreateUserRequest::new(config)
///     .user_id_type(UserIdType::OpenId)
///     .client_token("uuid");
/// ```
pub struct CreateUserRequest {
    config: Config,
    user_id_type: Option<UserIdType>,
    department_id_type: Option<DepartmentIdType>,
    client_token: Option<String>,
}

impl CreateUserRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/user/create>
    pub async fn execute(self, body: CreateUserBody) -> SDKResult<UserResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: CreateUserBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UserResponse> {
        // === 必填字段验证 ===
        validate_required!(body.name, "name 不能为空");
        validate_required!(body.mobile, "mobile 不能为空");
        if body.department_ids.is_empty() {
            return Err(error::validation_error(
                "department_ids 不能为空".to_string(),
                "department_ids 至少需要包含 1 个部门 ID".to_string(),
            ));
        }

        // url: POST:/open-apis/contact/v3/users
        let mut req: ApiRequest<UserResponse> =
            ApiRequest::post(CONTACT_V3_USERS).body(serialize_params(&body, "创建用户")?);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        if let Some(department_id_type) = self.department_id_type {
            req = req.query("department_id_type", department_id_type.as_str());
        }
        if let Some(client_token) = self.client_token {
            req = req.query("client_token", client_token);
        }

        Transport::request_typed(req, &self.config, Some(option), "创建用户").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user_request_builder() {
        let config = Config::default();
        let request = CreateUserRequest::new(config);
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_create_user_request_with_user_id_type() {
        let config = Config::default();
        let request = CreateUserRequest::new(config).user_id_type(UserIdType::OpenId);
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_create_user_body_builder() {
        let body = CreateUserBody::new("张三", "13800138000", vec!["dept_1".to_string()], 1);
        assert_eq!(body.name, "张三");
        assert_eq!(body.mobile, "13800138000");
        assert_eq!(body.department_ids.len(), 1);
        assert_eq!(body.employee_type, 1);
    }

    #[test]
    fn test_create_user_body_with_custom_user_id() {
        let body = CreateUserBody::new("李四", "13900139000", vec!["dept_2".to_string()], 2);
        let body_with_id = CreateUserBody {
            user_id: Some("custom_user_id".to_string()),
            ..body
        };
        assert_eq!(body_with_id.user_id, Some("custom_user_id".to_string()));
    }

    /// 全量可选字段序列化：orders/custom_attrs 按官方嵌套结构输出，
    /// 可选字段为 None 时不输出。
    #[test]
    fn test_create_user_body_full_serialization() {
        let body = CreateUserBody {
            user_id: Some("3e3cf96b".to_string()),
            en_name: Some("San Zhang".to_string()),
            nickname: Some("Alex Zhang".to_string()),
            email: Some("zhangsan@gmail.com".to_string()),
            mobile_visible: Some(false),
            gender: Some(1),
            avatar_key: Some("2500c7a9".to_string()),
            leader_user_id: Some("ou_leader".to_string()),
            city: Some("杭州".to_string()),
            country: Some("CN".to_string()),
            work_station: Some("北楼-H34".to_string()),
            join_time: Some(2147483647),
            employee_no: Some("1".to_string()),
            orders: Some(vec![UserOrder {
                department_id: Some("od-4e6ac4d14bcd5071a37a39de902c7141".to_string()),
                user_order: Some(100),
                department_order: Some(100),
                is_primary_dept: Some(true),
            }]),
            custom_attrs: Some(vec![UserCustomAttr {
                r#type: Some("TEXT".to_string()),
                id: Some("DemoId".to_string()),
                value: Some(UserCustomAttrValue {
                    text: Some("DemoText".to_string()),
                    url: None,
                    pc_url: None,
                    option_id: None,
                    generic_user: None,
                }),
            }]),
            enterprise_email: Some("demo@mail.com".to_string()),
            job_title: Some("xxxxx".to_string()),
            geo: Some("cn".to_string()),
            job_level_id: Some("mga5oa8ayjlp9rb".to_string()),
            job_family_id: Some("mga5oa8ayjlp9rb".to_string()),
            subscription_ids: Some(vec!["23213213213123123".to_string()]),
            dotted_line_leader_user_ids: Some(vec!["ou_dotted".to_string()]),
            ..CreateUserBody::new("张三", "13011111111", vec!["od-dept".to_string()], 1)
        };

        let value = serde_json::to_value(&body).expect("序列化失败");
        assert_eq!(value["name"], "张三");
        assert_eq!(value["mobile"], "13011111111");
        assert_eq!(value["department_ids"][0], "od-dept");
        assert_eq!(value["employee_type"], 1);
        assert_eq!(value["en_name"], "San Zhang");
        assert_eq!(value["mobile_visible"], false);
        assert_eq!(value["gender"], 1);
        assert_eq!(value["join_time"], 2147483647);
        assert_eq!(
            value["orders"][0]["department_id"],
            "od-4e6ac4d14bcd5071a37a39de902c7141"
        );
        assert_eq!(value["orders"][0]["user_order"], 100);
        assert_eq!(value["orders"][0]["is_primary_dept"], true);
        assert_eq!(value["custom_attrs"][0]["type"], "TEXT");
        assert_eq!(value["custom_attrs"][0]["id"], "DemoId");
        assert_eq!(value["custom_attrs"][0]["value"]["text"], "DemoText");
        assert_eq!(value["subscription_ids"][0], "23213213213123123");
        assert_eq!(value["dotted_line_leader_user_ids"][0], "ou_dotted");
        // value 内未设置的字段不输出
        assert!(value["custom_attrs"][0]["value"].get("url").is_none());

        // roundtrip：反序列化保留嵌套结构
        let decoded: CreateUserBody = serde_json::from_value(value).expect("反序列化失败");
        assert_eq!(
            decoded.orders.as_deref(),
            Some(
                &[UserOrder {
                    department_id: Some("od-4e6ac4d14bcd5071a37a39de902c7141".to_string()),
                    user_order: Some(100),
                    department_order: Some(100),
                    is_primary_dept: Some(true),
                }][..]
            )
        );
        assert_eq!(
            decoded.custom_attrs.as_deref(),
            Some(
                &[UserCustomAttr {
                    r#type: Some("TEXT".to_string()),
                    id: Some("DemoId".to_string()),
                    value: Some(UserCustomAttrValue {
                        text: Some("DemoText".to_string()),
                        url: None,
                        pc_url: None,
                        option_id: None,
                        generic_user: None,
                    }),
                }][..]
            )
        );
    }

    /// 最小请求体：只含四个必填字段。
    #[test]
    fn test_create_user_body_minimal_serialization() {
        let body = CreateUserBody::new("张三", "13011111111", vec!["od-dept".to_string()], 1);
        let value = serde_json::to_value(&body).expect("序列化失败");
        let obj = value.as_object().expect("应为 JSON 对象");
        assert_eq!(
            obj.len(),
            4,
            "最小请求体只应包含 name/mobile/department_ids/employee_type"
        );
    }

    #[test]
    fn test_create_user_request_with_all_options() {
        let config = Config::default();
        let request = CreateUserRequest::new(config)
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
