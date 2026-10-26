//! 更新员工信息
//!
//! 文档: <https://open.feishu.cn/document/directory-v1/employee/patch>
//! docPath: <https://open.feishu.cn/document/directory-v1/employee/patch>

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

/// 更新员工信息 Builder
#[derive(Debug, Clone)]
pub struct EmployeePatchRequestBuilder {
    config: Config,
    /// 员工 ID
    employee_id: String,
    /// 更新员工对象
    employee: UpdateEmployee,
    /// 用户 ID 类型（查询参数，可选：open_id/union_id/employee_id）
    employee_id_type: Option<String>,
    /// 部门 ID 类型（查询参数，可选：department_id/open_department_id）
    department_id_type: Option<String>,
}

impl EmployeePatchRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, employee_id: impl Into<String>) -> Self {
        Self {
            config,
            employee_id: employee_id.into(),
            employee: UpdateEmployee::default(),
            employee_id_type: None,
            department_id_type: None,
        }
    }

    /// 设置员工姓名的默认值（employee.name.name.default_value，1~64 字符）
    pub fn name(mut self, name: impl Into<String>) -> Self {
        let upsert = self.employee.name.get_or_insert_with(UpsertName::default);
        upsert.name = Some(I18nText {
            default_value: name.into(),
            i18n_value: None,
        });
        self
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
        self.employee
            .name
            .get_or_insert_with(UpsertName::default)
            .another_name = Some(another_name.into());
        self
    }

    /// 设置手机号（非中国大陆手机号须以 + 开头）
    pub fn mobile(mut self, mobile: impl Into<String>) -> Self {
        self.employee.mobile = Some(mobile.into());
        self
    }

    /// 设置自定义员工 ID（1~64 字符，不能包含空格）
    pub fn custom_employee_id(mut self, custom_employee_id: impl Into<String>) -> Self {
        self.employee.custom_employee_id = Some(custom_employee_id.into());
        self
    }

    /// 设置员工头像 key
    pub fn avatar_key(mut self, avatar_key: impl Into<String>) -> Self {
        self.employee.avatar_key = Some(avatar_key.into());
        self
    }

    /// 设置背景图 key（图片上传类型需选择「用于发送消息」）
    pub fn background_image_key(mut self, key: impl Into<String>) -> Self {
        self.employee.background_image_key = Some(key.into());
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

    /// 设置员工个性签名
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.employee.description = Some(description.into());
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

    /// 设置虚线上级 ID 列表（0~10 个）
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

    /// 设置工号（企业内不可重复）
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

    /// 设置员工类型：0 未知 / 1 全职 / 2 实习 / 3 外包 / 4 劳务 / 5 顾问
    pub fn employment_type(mut self, employment_type: i32) -> Self {
        self.employee.employment_type = Some(employment_type);
        self
    }

    /// 设置职务 ID
    pub fn job_title_id(mut self, job_title_id: impl Into<String>) -> Self {
        self.employee.job_title_id = Some(job_title_id.into());
        self
    }

    /// 设置职级 ID
    pub fn job_level_id(mut self, job_level_id: impl Into<String>) -> Self {
        self.employee.job_level_id = Some(job_level_id.into());
        self
    }

    /// 设置序列 ID
    pub fn job_family_id(mut self, job_family_id: impl Into<String>) -> Self {
        self.employee.job_family_id = Some(job_family_id.into());
        self
    }

    /// 设置离职日期（固定格式 yyyy-mm-dd）
    pub fn resign_date(mut self, resign_date: impl Into<String>) -> Self {
        self.employee.resign_date = Some(resign_date.into());
        self
    }

    /// 设置离职原因（官方枚举 "1"~"25"，"0" 表示置空）
    pub fn resign_reason(mut self, resign_reason: impl Into<String>) -> Self {
        self.employee.resign_reason = Some(resign_reason.into());
        self
    }

    /// 设置离职备注（0~255 字符）
    pub fn resign_remark(mut self, resign_remark: impl Into<String>) -> Self {
        self.employee.resign_remark = Some(resign_remark.into());
        self
    }

    /// 设置离职类型：1 主动 / 2 被动 / 3 其他（"0" 表示置空）
    pub fn resign_type(mut self, resign_type: impl Into<String>) -> Self {
        self.employee.resign_type = Some(resign_type.into());
        self
    }

    /// 设置是否冻结员工账号（true 冻结 / false 恢复）
    pub fn is_frozen(mut self, is_frozen: bool) -> Self {
        self.employee.is_frozen = Some(is_frozen);
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
    pub async fn execute(self) -> SDKResult<EmployeePatchResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<EmployeePatchResponse> {
        validate_required!(self.employee_id, "employee_id 不能为空");

        let url = format!("/open-apis/directory/v1/employees/{}", self.employee_id);

        let body = EmployeePatchBody {
            employee: self.employee,
        };

        let req: ApiRequest<EmployeePatchResponse> = ApiRequest::patch(&url)
            .body(serde_json::to_value(&body)?)
            .query_opt("employee_id_type", self.employee_id_type)
            .query_opt("department_id_type", self.department_id_type);
        Transport::request_typed(req, &self.config, Some(option), "更新员工信息").await
    }
}

/// 更新员工信息请求体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeePatchBody {
    /// 更新员工对象
    pub employee: UpdateEmployee,
}

/// 更新员工对象（官方 update_employee，字段均为可选增量更新）
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEmployee {
    /// 姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<UpsertName>,
    /// 手机号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// 自定义员工 ID（1~64 字符，不能包含空格）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_employee_id: Option<String>,
    /// 员工头像 key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_key: Option<String>,
    /// 背景图 key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_image_key: Option<String>,
    /// 工作邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 企业邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_email: Option<String>,
    /// 性别：0 未知 / 1 男 / 2 女 / 3 其他
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<i32>,
    /// 员工个性签名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 员工在所属部门内的排序信息（0~10 个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_order_in_departments: Option<Vec<UserDepartmentSortInfo>>,
    /// 直属上级 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader_id: Option<String>,
    /// 虚线上级 ID 列表（0~10 个）
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
    /// 工号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_number: Option<String>,
    /// 分机号（最多 99 字）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_number: Option<String>,
    /// 入职日期（固定格式 yyyy-mm-dd）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_date: Option<String>,
    /// 员工类型：0 未知 / 1 全职 / 2 实习 / 3 外包 / 4 劳务 / 5 顾问
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_type: Option<i32>,
    /// 职务 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_title_id: Option<String>,
    /// 职级 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    /// 序列 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    /// 离职日期（固定格式 yyyy-mm-dd）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resign_date: Option<String>,
    /// 离职原因（官方枚举 "1"~"25"，"0" 表示置空）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resign_reason: Option<String>,
    /// 离职备注（0~255 字符）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resign_remark: Option<String>,
    /// 离职类型：1 主动 / 2 被动 / 3 其他（"0" 表示置空）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resign_type: Option<String>,
    /// 是否冻结员工账号（true 冻结 / false 恢复）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_frozen: Option<bool>,
}

/// 姓名（官方 upsert_name）
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpsertName {
    /// 员工的姓名
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
    /// 部门 ID，与 department_id_type 类型保持一致
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

/// 更新员工信息响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeePatchResponse {
    /// 员工 ID
    #[serde(rename = "employee_id")]
    pub employee_id: String,
    /// 更新后的名称
    #[serde(rename = "name")]
    pub name: String,
    /// 更新时间
    #[serde(rename = "updated_at")]
    pub updated_at: i64,
}

impl ApiResponseTrait for EmployeePatchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to EmployeePatchRequestBuilder, will be removed in v1.0 (#271)")]
pub type EmployeePatchBuilder = EmployeePatchRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PATCH 官方 `{"employee": {...}}` 请求体 → 强类型 EmployeePatchResponse。
    #[tokio::test]
    async fn test_patch_employee_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/directory/v1/employees/emp_001"))
            .and(body_json(json!({
                "employee": {
                    "name": { "name": { "default_value": "alice_updated" } },
                    "email": "alice@example.com"
                }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "employee_id": "emp_001",
                    "name": "alice_updated",
                    "updated_at": 1700000000
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

        let resp = EmployeePatchRequestBuilder::new(config, "emp_001")
            .name("alice_updated")
            .email("alice@example.com")
            .execute()
            .await
            .expect("更新员工信息应成功");
        assert_eq!(resp.employee_id, "emp_001");
        assert_eq!(resp.name, "alice_updated");
        assert_eq!(resp.updated_at, 1700000000);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/directory/v1/employees/emp_001"
        );
        assert_eq!(received[0].method, "PATCH");
    }
}
