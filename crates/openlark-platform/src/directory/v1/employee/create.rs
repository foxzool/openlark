//! 创建员工
//!
//! 文档: <https://open.feishu.cn/document/directory-v1/employee/create>
//! docPath: <https://open.feishu.cn/document/directory-v1/employee/create>

use std::collections::HashMap;

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 创建员工 Builder
#[derive(Debug, Clone)]
pub struct EmployeeCreateRequestBuilder {
    config: Config,
    /// 创建员工对象
    employee: CreateEmployee,
    /// 接口拓展选项
    options: Option<CreateEmployeeOptions>,
    /// 用户 ID 类型（查询参数，可选：open_id/union_id/employee_id）
    employee_id_type: Option<String>,
    /// 部门 ID 类型（查询参数，可选：department_id/open_department_id）
    department_id_type: Option<String>,
}

impl EmployeeCreateRequestBuilder {
    /// 创建新的 Builder
    ///
    /// `name` 作为员工姓名的默认值（employee.name.name.default_value，最多 64 字），
    /// `mobile` 为员工手机号（非中国大陆手机号须以 + 开头）。
    pub fn new(config: Config, name: impl Into<String>, mobile: impl Into<String>) -> Self {
        Self {
            config,
            employee: CreateEmployee {
                name: Some(UpsertName {
                    name: Some(I18nText {
                        default_value: name.into(),
                        i18n_value: None,
                    }),
                    another_name: None,
                }),
                mobile: Some(mobile.into()),
                custom_employee_id: None,
                avatar_key: None,
                email: None,
                enterprise_email: None,
                gender: None,
                employee_order_in_departments: None,
                leader_id: None,
                dotted_line_leader_ids: None,
                work_country_or_region: None,
                work_place_id: None,
                work_station: None,
                job_number: None,
                extension_number: None,
                join_date: None,
                employment_type: None,
                job_title_id: None,
            },
            options: None,
            employee_id_type: None,
            department_id_type: None,
        }
    }

    /// 设置员工姓名的国际化值（key 为 zh_cn/ja_jp/en_us）
    pub fn name_i18n(mut self, i18n_value: HashMap<String, String>) -> Self {
        if let Some(name) = self.employee.name.as_mut().and_then(|n| n.name.as_mut()) {
            name.i18n_value = Some(i18n_value);
        }
        self
    }

    /// 设置别名（最多 64 字）
    pub fn another_name(mut self, another_name: impl Into<String>) -> Self {
        if let Some(name) = self.employee.name.as_mut() {
            name.another_name = Some(another_name.into());
        }
        self
    }

    /// 设置自定义员工 ID（1~64 字符，不能包含空格）
    pub fn custom_employee_id(mut self, custom_employee_id: impl Into<String>) -> Self {
        self.employee.custom_employee_id = Some(custom_employee_id.into());
        self
    }

    /// 设置员工头像 key（上传图片时类型需选择「用于设置头像」）
    pub fn avatar_key(mut self, avatar_key: impl Into<String>) -> Self {
        self.employee.avatar_key = Some(avatar_key.into());
        self
    }

    /// 设置工作邮箱
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.employee.email = Some(email.into());
        self
    }

    /// 设置企业邮箱
    pub fn enterprise_email(mut self, enterprise_email: impl Into<String>) -> Self {
        self.employee.enterprise_email = Some(enterprise_email.into());
        self
    }

    /// 设置性别：0 未知 / 1 男 / 2 女 / 3 其他
    pub fn gender(mut self, gender: i32) -> Self {
        self.employee.gender = Some(gender);
        self
    }

    /// 设置员工在所属部门内的排序信息（0~10 个）
    pub fn employee_order_in_departments(mut self, orders: Vec<UserDepartmentSortInfo>) -> Self {
        self.employee.employee_order_in_departments = Some(orders);
        self
    }

    /// 添加员工所属部门排序信息（指定员工所在部门与排序权重）
    pub fn department_order(mut self, order: UserDepartmentSortInfo) -> Self {
        self.employee
            .employee_order_in_departments
            .get_or_insert_with(Vec::new)
            .push(order);
        self
    }

    /// 设置直属上级 ID（不可成环，且需为在职员工）
    pub fn leader_id(mut self, leader_id: impl Into<String>) -> Self {
        self.employee.leader_id = Some(leader_id.into());
        self
    }

    /// 设置虚线上级 ID 列表（0~20 个）
    pub fn dotted_line_leader_ids(
        mut self,
        ids: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.employee.dotted_line_leader_ids = Some(ids.into_iter().map(Into::into).collect());
        self
    }

    /// 设置工作地国家/地区码
    pub fn work_country_or_region(mut self, code: impl Into<String>) -> Self {
        self.employee.work_country_or_region = Some(code.into());
        self
    }

    /// 设置工作地点 ID
    pub fn work_place_id(mut self, work_place_id: impl Into<String>) -> Self {
        self.employee.work_place_id = Some(work_place_id.into());
        self
    }

    /// 设置工位
    pub fn work_station(mut self, default_value: impl Into<String>) -> Self {
        self.employee.work_station = Some(I18nText {
            default_value: default_value.into(),
            i18n_value: None,
        });
        self
    }

    /// 设置工号（企业内不可重复，0~255 字符）
    pub fn job_number(mut self, job_number: impl Into<String>) -> Self {
        self.employee.job_number = Some(job_number.into());
        self
    }

    /// 设置分机号（最多 99 字，企业内不可重复）
    pub fn extension_number(mut self, extension_number: impl Into<String>) -> Self {
        self.employee.extension_number = Some(extension_number.into());
        self
    }

    /// 设置入职日期（固定格式 yyyy-mm-dd）
    pub fn join_date(mut self, join_date: impl Into<String>) -> Self {
        self.employee.join_date = Some(join_date.into());
        self
    }

    /// 设置员工类型：1 全职 / 2 实习 / 3 外包 / 4 劳务 / 5 顾问
    pub fn employment_type(mut self, employment_type: i32) -> Self {
        self.employee.employment_type = Some(employment_type);
        self
    }

    /// 设置职务 ID
    pub fn job_title_id(mut self, job_title_id: impl Into<String>) -> Self {
        self.employee.job_title_id = Some(job_title_id.into());
        self
    }

    /// 设置员工数据驻留地（仅 Multi-Geo 企业可选，需传小写字母）
    pub fn geo_name(mut self, geo_name: impl Into<String>) -> Self {
        self.options
            .get_or_insert_with(CreateEmployeeOptions::default)
            .geo_name = Some(geo_name.into());
        self
    }

    /// 设置分配给员工的席位 ID 列表（0~20 个）
    pub fn subscription_ids(mut self, ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.options
            .get_or_insert_with(CreateEmployeeOptions::default)
            .subscription_ids = Some(ids.into_iter().map(Into::into).collect());
        self
    }

    /// 设置用户 ID 类型（查询参数）
    pub fn employee_id_type(mut self, employee_id_type: impl Into<String>) -> Self {
        self.employee_id_type = Some(employee_id_type.into());
        self
    }

    /// 设置部门 ID 类型（查询参数）
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.department_id_type = Some(department_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<EmployeeCreateResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<EmployeeCreateResponse> {
        let name = self
            .employee
            .name
            .as_ref()
            .and_then(|n| n.name.as_ref())
            .map(|t| t.default_value.clone())
            .unwrap_or_default();
        validate_required!(name, "employee.name.name.default_value 不能为空");

        let url = "/open-apis/directory/v1/employees";

        let body = EmployeeCreateBody {
            employee: self.employee,
            options: self.options,
        };

        let req: ApiRequest<EmployeeCreateResponse> = ApiRequest::post(url)
            .body(serde_json::to_value(&body)?)
            .query_opt("employee_id_type", self.employee_id_type)
            .query_opt("department_id_type", self.department_id_type);
        Transport::request_typed(req, &self.config, Some(option), "创建员工").await
    }
}

/// 创建员工请求体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeCreateBody {
    /// 创建员工对象
    pub employee: CreateEmployee,
    /// 接口拓展选项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<CreateEmployeeOptions>,
}

/// 创建员工对象（官方 create_employee）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateEmployee {
    /// 姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<UpsertName>,
    /// 手机号（非中国大陆手机号须以 + 开头）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// 自定义员工 ID（1~64 字符，不能包含空格）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_employee_id: Option<String>,
    /// 员工头像 key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_key: Option<String>,
    /// 工作邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 企业邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_email: Option<String>,
    /// 性别：0 未知 / 1 男 / 2 女 / 3 其他
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<i32>,
    /// 员工在所属部门内的排序信息（0~10 个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_order_in_departments: Option<Vec<UserDepartmentSortInfo>>,
    /// 直属上级 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader_id: Option<String>,
    /// 虚线上级 ID 列表（0~20 个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dotted_line_leader_ids: Option<Vec<String>>,
    /// 工作地国家/地区码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_country_or_region: Option<String>,
    /// 工作地点 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_place_id: Option<String>,
    /// 工位
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_station: Option<I18nText>,
    /// 工号（0~255 字符）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_number: Option<String>,
    /// 分机号（最多 99 字）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_number: Option<String>,
    /// 入职日期（固定格式 yyyy-mm-dd）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_date: Option<String>,
    /// 员工类型：1 全职 / 2 实习 / 3 外包 / 4 劳务 / 5 顾问
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_type: Option<i32>,
    /// 职务 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_title_id: Option<String>,
}

/// 姓名（官方 upsert_name）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpsertName {
    /// 员工的姓名，最多 64 字
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 别名，最多 64 字
    #[serde(skip_serializing_if = "Option::is_none")]
    pub another_name: Option<String>,
}

/// i18n 文本（官方 i18n_text）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct I18nText {
    /// 默认值
    pub default_value: String,
    /// 国际化值，key 为 zh_cn/ja_jp/en_us
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_value: Option<HashMap<String, String>>,
}

/// 员工在部门内的排序信息（官方 upsert_user_department_sort_info）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserDepartmentSortInfo {
    /// 指定员工所在的部门，与 department_id_type 类型保持一致
    pub department_id: String,
    /// 员工在部门内的排序权重
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_weight_in_deparment: Option<String>,
    /// 该部门在员工所属多个部门间的排序权重
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_weight_among_deparments: Option<String>,
    /// 是否为用户的主部门（用户只能有一个主部门，且排序权重应最大）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_main_department: Option<bool>,
}

/// 创建员工拓展选项（官方 create_employee_options）
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEmployeeOptions {
    /// 员工的数据驻留地（仅 Multi-Geo 企业可选，传小写字母）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_name: Option<String>,
    /// 分配给员工的席位 ID 列表（0~20 个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_ids: Option<Vec<String>>,
}

/// 创建员工响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeCreateResponse {
    /// 员工 ID
    #[serde(rename = "employee_id")]
    pub employee_id: String,
    /// 员工名称
    #[serde(rename = "name")]
    pub name: String,
    /// 手机号
    #[serde(rename = "mobile")]
    pub mobile: String,
    /// 创建时间
    #[serde(rename = "created_at")]
    pub created_at: i64,
}

impl ApiResponseTrait for EmployeeCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to EmployeeCreateRequestBuilder, will be removed in v1.0 (#271)")]
pub type EmployeeCreateBuilder = EmployeeCreateRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST 官方 `{"employee": {...}}` 请求体 → 强类型 EmployeeCreateResponse。
    #[tokio::test]
    async fn test_create_employee_sends_official_body() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/directory/v1/employees"))
            .and(body_json(json!({
                "employee": {
                    "name": {
                        "name": { "default_value": "alice" }
                    },
                    "mobile": "13011111111",
                    "email": "alice@example.com",
                    "employee_order_in_departments": [
                        {
                            "department_id": "dept_001",
                            "order_weight_in_deparment": "100",
                            "is_main_department": true
                        }
                    ],
                    "join_date": "2024-06-01",
                    "employment_type": 1
                }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "employee_id": "emp_001",
                    "name": "alice",
                    "mobile": "13011111111",
                    "created_at": 1700000000
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = EmployeeCreateRequestBuilder::new(config, "alice", "13011111111")
            .email("alice@example.com")
            .department_order(UserDepartmentSortInfo {
                department_id: "dept_001".to_string(),
                order_weight_in_deparment: Some("100".to_string()),
                order_weight_among_deparments: None,
                is_main_department: Some(true),
            })
            .join_date("2024-06-01")
            .employment_type(1)
            .execute()
            .await
            .expect("创建员工应成功");
        assert_eq!(resp.employee_id, "emp_001");
        assert_eq!(resp.name, "alice");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/directory/v1/employees");
        assert_eq!(received[0].method, "POST");
    }

    /// 请求体序列化：options 未设置时不输出对应 key。
    #[test]
    fn test_create_employee_body_omits_absent_fields() {
        let body = EmployeeCreateBody {
            employee: CreateEmployee {
                name: Some(UpsertName {
                    name: Some(I18nText {
                        default_value: "alice".to_string(),
                        i18n_value: None,
                    }),
                    another_name: None,
                }),
                mobile: Some("13011111111".to_string()),
                custom_employee_id: None,
                avatar_key: None,
                email: None,
                enterprise_email: None,
                gender: None,
                employee_order_in_departments: None,
                leader_id: None,
                dotted_line_leader_ids: None,
                work_country_or_region: None,
                work_place_id: None,
                work_station: None,
                job_number: None,
                extension_number: None,
                join_date: None,
                employment_type: None,
                job_title_id: None,
            },
            options: None,
        };
        assert_eq!(
            serde_json::to_value(&body).unwrap(),
            json!({
                "employee": {
                    "name": { "name": { "default_value": "alice" } },
                    "mobile": "13011111111"
                }
            })
        );
    }
}
