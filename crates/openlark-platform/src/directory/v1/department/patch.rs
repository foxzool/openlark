//! 更新部门
//!
//! 文档: <https://open.feishu.cn/document/directory-v1/department/patch>
//! docPath: <https://open.feishu.cn/document/directory-v1/department/patch>

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

/// 更新部门 Builder
#[derive(Debug, Clone)]
pub struct DepartmentPatchRequestBuilder {
    config: Config,
    /// 部门 ID
    department_id: String,
    /// 更新部门对象
    department: UpdateDepartment,
    /// 用户 ID 类型（查询参数，可选：open_id/union_id/employee_id）
    employee_id_type: Option<String>,
    /// 部门 ID 类型（查询参数，可选：department_id/open_department_id）
    department_id_type: Option<String>,
}

impl DepartmentPatchRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, department_id: impl Into<String>) -> Self {
        Self {
            config,
            department_id: department_id.into(),
            department: UpdateDepartment {
                name: None,
                custom_department_id: None,
                parent_department_id: None,
                leaders: None,
                order_weight: None,
                enabled_status: None,
            },
            employee_id_type: None,
            department_id_type: None,
        }
    }

    /// 设置部门名称的默认值（i18n_text.default_value，1~100 字符）
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.department.name = Some(I18nText {
            default_value: name.into(),
            i18n_value: None,
        });
        self
    }

    /// 设置部门名称的国际化值（key 为 zh_cn/ja_jp/en_us）
    pub fn name_i18n(mut self, i18n_value: HashMap<String, String>) -> Self {
        if let Some(name) = self.department.name.as_mut() {
            name.i18n_value = Some(i18n_value);
        }
        self
    }

    /// 设置自定义部门 ID（正则 `^[a-zA-Z0-9][a-zA-Z0-9_\-@.]{0,63}$`，不能以 od- 开头）
    pub fn custom_department_id(mut self, custom_department_id: impl Into<String>) -> Self {
        self.department.custom_department_id = Some(custom_department_id.into());
        self
    }

    /// 设置父部门 ID（与 department_id_type 类型保持一致，根部门为 "0"）
    pub fn parent_department_id(mut self, parent_department_id: impl Into<String>) -> Self {
        self.department.parent_department_id = Some(parent_department_id.into());
        self
    }

    /// 设置部门负责人列表（最多 20 个）
    pub fn leaders(mut self, leaders: Vec<DepartmentLeader>) -> Self {
        self.department.leaders = Some(leaders);
        self
    }

    /// 添加部门负责人（负责人类型：1 主 / 2 副）
    pub fn leader(mut self, leader_type: i32, leader_id: impl Into<String>) -> Self {
        self.department
            .leaders
            .get_or_insert_with(Vec::new)
            .push(DepartmentLeader {
                leader_type,
                leader_id: leader_id.into(),
            });
        self
    }

    /// 设置在上级部门下的排序权重（返回结果按 order_weight 升序排列）
    pub fn order_weight(mut self, order_weight: impl Into<String>) -> Self {
        self.department.order_weight = Some(order_weight.into());
        self
    }

    /// 设置是否启用
    pub fn enabled_status(mut self, enabled_status: bool) -> Self {
        self.department.enabled_status = Some(enabled_status);
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
    pub async fn execute(self) -> SDKResult<DepartmentPatchResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DepartmentPatchResponse> {
        validate_required!(self.department_id, "department_id 不能为空");

        let url = format!("/open-apis/directory/v1/departments/{}", self.department_id);

        let body = DepartmentPatchBody {
            department: self.department,
        };

        let req: ApiRequest<DepartmentPatchResponse> = ApiRequest::patch(&url)
            .body(serde_json::to_value(&body)?)
            .query_opt("employee_id_type", self.employee_id_type)
            .query_opt("department_id_type", self.department_id_type);
        Transport::request_typed(req, &self.config, Some(option), "更新部门").await
    }
}

/// 更新部门请求体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepartmentPatchBody {
    /// 更新部门对象
    pub department: UpdateDepartment,
}

/// 更新部门对象（官方 update_department）
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDepartment {
    /// 部门名称（default_value 1~100 字符）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 自定义部门 ID（正则 `^[a-zA-Z0-9][a-zA-Z0-9_\-@.]{0,63}$`，不能以 od- 开头）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_department_id: Option<String>,
    /// 父部门 ID（与 department_id_type 类型保持一致，根部门为 "0"）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    /// 部门负责人（0~20 个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaders: Option<Vec<DepartmentLeader>>,
    /// 在上级部门下的排序权重
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_weight: Option<String>,
    /// 是否启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_status: Option<bool>,
}

/// i18n 文本（官方 i18n_text）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct I18nText {
    /// 默认值（1~100 字符）
    pub default_value: String,
    /// 国际化值，key 为 zh_cn/ja_jp/en_us
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_value: Option<HashMap<String, String>>,
}

/// 部门负责人（官方 department_leader）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepartmentLeader {
    /// 部门负责人类型：1 主 / 2 副
    pub leader_type: i32,
    /// 部门负责人 ID，与 employee_id_type 类型保持一致
    pub leader_id: String,
}

/// 更新部门响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepartmentPatchResponse {
    /// 部门 ID
    #[serde(rename = "department_id")]
    pub department_id: String,
    /// 更新后的名称
    #[serde(rename = "name")]
    pub name: String,
    /// 更新时间
    #[serde(rename = "updated_at")]
    pub updated_at: i64,
}

impl ApiResponseTrait for DepartmentPatchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to DepartmentPatchRequestBuilder, will be removed in v1.0 (#271)")]
pub type DepartmentPatchBuilder = DepartmentPatchRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PATCH 官方 `{"department": {...}}` 请求体 → 强类型 DepartmentPatchResponse。
    #[tokio::test]
    async fn test_patch_department_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/directory/v1/departments/dept_001"))
            .and(body_json(json!({
                "department": {
                    "name": { "default_value": "Engineering" },
                    "parent_department_id": "dept_000",
                    "leaders": [
                        { "leader_type": 1, "leader_id": "u_001" }
                    ]
                }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "department_id": "dept_001",
                    "name": "Engineering",
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

        let resp = DepartmentPatchRequestBuilder::new(config, "dept_001")
            .name("Engineering")
            .parent_department_id("dept_000")
            .leader(1, "u_001")
            .execute()
            .await
            .expect("更新部门应成功");
        assert_eq!(resp.department_id, "dept_001");
        assert_eq!(resp.name, "Engineering");
        assert_eq!(resp.updated_at, 1700000000);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/directory/v1/departments/dept_001"
        );
        assert_eq!(received[0].method, "PATCH");
    }
}
