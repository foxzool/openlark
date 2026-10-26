//! 更新岗位信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/position/patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::query::{CustomFieldData, I18nText};

/// 更新岗位信息请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchBody {
    /// 编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 名称（i18n 多语）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<I18nText>>,
    /// 描述（i18n 多语）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<I18nText>>,
    /// 序列 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family_ids: Option<Vec<String>>,
    /// 成本中心 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    /// 职务 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// 职级 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_level_ids: Option<Vec<String>>,
    /// 人员类型 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_type_ids: Option<Vec<String>>,
    /// 职等 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_grade_ids: Option<Vec<String>>,
    /// 工作地点 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_location_ids: Option<Vec<String>>,
    /// 工时制度 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_hours_type_id: Option<String>,
    /// 部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 直属上级岗位 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_leader_id: Option<String>,
    /// 虚线上级岗位 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dotted_line_leader_id: Option<String>,
    /// 是否关键岗位
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_key_position: Option<bool>,
    /// 版本生效日期（必填，格式：YYYY-MM-DD）
    pub effective_time: String,
    /// 自定义字段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldData>>,
}

/// 更新岗位信息请求
#[derive(Debug, Clone)]
pub struct PatchRequest {
    /// 配置信息
    config: Config,
    position_id: Option<String>,
    /// 请求体
    body: PatchBody,
    /// 根据 client_token 是否一致来判断是否为同一请求
    client_token: Option<String>,
    /// 此次调用中使用的部门 ID 类型
    department_id_type: Option<String>,
}

impl PatchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            position_id: None,
            body: PatchBody::default(),
            client_token: None,
            department_id_type: None,
        }
    }

    /// 设置 `position_id`。
    pub fn position_id(mut self, position_id: String) -> Self {
        self.position_id = Some(position_id);
        self
    }

    /// 设置请求体
    pub fn body(mut self, body: PatchBody) -> Self {
        self.body = body;
        self
    }

    /// 设置编码。
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.body.code = Some(code.into());
        self
    }

    /// 设置名称（i18n 多语）。
    pub fn names(mut self, names: Vec<I18nText>) -> Self {
        self.body.names = Some(names);
        self
    }

    /// 设置描述（i18n 多语）。
    pub fn descriptions(mut self, descriptions: Vec<I18nText>) -> Self {
        self.body.descriptions = Some(descriptions);
        self
    }

    /// 设置序列 ID 列表。
    pub fn job_family_ids(mut self, job_family_ids: Vec<String>) -> Self {
        self.body.job_family_ids = Some(job_family_ids);
        self
    }

    /// 设置成本中心 ID。
    pub fn cost_center_id(mut self, cost_center_id: impl Into<String>) -> Self {
        self.body.cost_center_id = Some(cost_center_id.into());
        self
    }

    /// 设置职务 ID。
    pub fn job_id(mut self, job_id: impl Into<String>) -> Self {
        self.body.job_id = Some(job_id.into());
        self
    }

    /// 设置职级 ID 列表。
    pub fn job_level_ids(mut self, job_level_ids: Vec<String>) -> Self {
        self.body.job_level_ids = Some(job_level_ids);
        self
    }

    /// 设置人员类型 ID 列表。
    pub fn employee_type_ids(mut self, employee_type_ids: Vec<String>) -> Self {
        self.body.employee_type_ids = Some(employee_type_ids);
        self
    }

    /// 设置职等 ID 列表。
    pub fn job_grade_ids(mut self, job_grade_ids: Vec<String>) -> Self {
        self.body.job_grade_ids = Some(job_grade_ids);
        self
    }

    /// 设置工作地点 ID 列表。
    pub fn work_location_ids(mut self, work_location_ids: Vec<String>) -> Self {
        self.body.work_location_ids = Some(work_location_ids);
        self
    }

    /// 设置工时制度 ID。
    pub fn working_hours_type_id(mut self, working_hours_type_id: impl Into<String>) -> Self {
        self.body.working_hours_type_id = Some(working_hours_type_id.into());
        self
    }

    /// 设置部门 ID。
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.body.department_id = Some(department_id.into());
        self
    }

    /// 设置直属上级岗位 ID。
    pub fn direct_leader_id(mut self, direct_leader_id: impl Into<String>) -> Self {
        self.body.direct_leader_id = Some(direct_leader_id.into());
        self
    }

    /// 设置虚线上级岗位 ID。
    pub fn dotted_line_leader_id(mut self, dotted_line_leader_id: impl Into<String>) -> Self {
        self.body.dotted_line_leader_id = Some(dotted_line_leader_id.into());
        self
    }

    /// 设置是否关键岗位。
    pub fn is_key_position(mut self, is_key_position: bool) -> Self {
        self.body.is_key_position = Some(is_key_position);
        self
    }

    /// 设置版本生效日期（必填，格式：YYYY-MM-DD）。
    pub fn effective_time(mut self, effective_time: impl Into<String>) -> Self {
        self.body.effective_time = effective_time.into();
        self
    }

    /// 设置自定义字段列表。
    pub fn custom_fields(mut self, custom_fields: Vec<CustomFieldData>) -> Self {
        self.body.custom_fields = Some(custom_fields);
        self
    }

    /// 设置 client_token（判断是否为同一请求）。
    pub fn client_token(mut self, client_token: impl Into<String>) -> Self {
        self.client_token = Some(client_token.into());
        self
    }

    /// 设置部门 ID 类型（open_department_id/department_id/people_corehr_department_id）。
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.department_id_type = Some(department_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<PatchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PatchResponse> {
        let position_id = self.position_id.unwrap_or_default();
        validate_required!(position_id.trim(), "position_id 不能为空");
        validate_required!(self.body.effective_time, "effective_time 不能为空");

        let request = ApiRequest::<PatchResponse>::patch(format!(
            "/open-apis/corehr/v2/positions/{position_id}"
        ))
        .query_opt("client_token", self.client_token)
        .query_opt("department_id_type", self.department_id_type)
        .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 更新岗位信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchResponse {
    /// 响应数据
    pub data: Value,
}

impl ApiResponseTrait for PatchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PATCH /open-apis/corehr/v2/positions/test001
    #[tokio::test]
    async fn test_patch_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/corehr/v2/positions/test001"))
            .and(query_param("department_id_type", "open_department_id"))
            .and(body_json(json!({
                "code": "A01234",
                "names": [{ "lang": "zh-CN", "value": "张三" }],
                "descriptions": [{ "lang": "zh-CN", "value": "张三" }],
                "job_family_ids": ["4719519211875096301"],
                "cost_center_id": "4719519211875096301",
                "job_id": "4719519211875096301",
                "job_level_ids": ["4719519211875096301"],
                "employee_type_ids": ["4719519211875096301"],
                "job_grade_ids": ["4719519211875096301"],
                "work_location_ids": ["4719519211875096301"],
                "working_hours_type_id": "4719519211875096301",
                "department_id": "4719519211875096301",
                "direct_leader_id": "4719519211875096301",
                "dotted_line_leader_id": "4719519211875096301",
                "is_key_position": true,
                "effective_time": "2020-05-01",
                "custom_fields": [{ "custom_api_name": "name", "value": "\"231\"" }]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "data": {} }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        PatchRequest::new(config)
            .position_id("test001".to_string())
            .code("A01234")
            .names(vec![I18nText {
                lang: Some("zh-CN".to_string()),
                value: Some("张三".to_string()),
            }])
            .descriptions(vec![I18nText {
                lang: Some("zh-CN".to_string()),
                value: Some("张三".to_string()),
            }])
            .job_family_ids(vec!["4719519211875096301".to_string()])
            .cost_center_id("4719519211875096301")
            .job_id("4719519211875096301")
            .job_level_ids(vec!["4719519211875096301".to_string()])
            .employee_type_ids(vec!["4719519211875096301".to_string()])
            .job_grade_ids(vec!["4719519211875096301".to_string()])
            .work_location_ids(vec!["4719519211875096301".to_string()])
            .working_hours_type_id("4719519211875096301")
            .department_id("4719519211875096301")
            .direct_leader_id("4719519211875096301")
            .dotted_line_leader_id("4719519211875096301")
            .is_key_position(true)
            .effective_time("2020-05-01")
            .custom_fields(vec![CustomFieldData {
                custom_api_name: Some("name".to_string()),
                name: None,
                r#type: None,
                value: Some("\"231\"".to_string()),
            }])
            .department_id_type("open_department_id")
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
