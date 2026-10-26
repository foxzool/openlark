//! 创建岗位信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/position/create>

use crate::common::shared_models::I18nLangValue;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 官方 `custom_field_data` 结构（请求侧）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CustomFieldData {
    /// 自定义字段 apiname，即自定义字段的唯一标识（必填）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_api_name: Option<String>,
    /// 字段值，是 json 转义后的字符串（必填）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 创建岗位信息请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateBody {
    /// 名称（必填，i18n 多语，最多 2 条）
    pub names: Vec<I18nLangValue>,
    /// 职务 ID（必填），可通过查询单个职务接口获得
    pub job_id: String,
    /// 部门 ID（必填），ID 类型须与查询参数 department_id_type 一致
    pub department_id: String,
    /// 版本生效日期（必填，格式：YYYY-MM-DD）
    pub effective_time: String,
    /// 岗位编码（不能与其他记录的编码重复；开启自动编码时传入值不生效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 描述（i18n 多语，最多 2 条）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<I18nLangValue>>,
    /// 序列 ID 列表（最多 50 个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family_ids: Option<Vec<String>>,
    /// 成本中心 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    /// 职级 ID 列表（最多 50 个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_level_ids: Option<Vec<String>>,
    /// 人员类型 ID 列表（最多 50 个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_type_ids: Option<Vec<String>>,
    /// 职等 ID 列表（最多 50 个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_grade_ids: Option<Vec<String>>,
    /// 工作地点 ID 列表（最多 50 个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_location_ids: Option<Vec<String>>,
    /// 工时制度 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_hours_type_id: Option<String>,
    /// 直属上级岗位 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_leader_id: Option<String>,
    /// 虚线上级岗位 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dotted_line_leader_id: Option<String>,
    /// 是否关键岗位
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_key_position: Option<bool>,
    /// 自定义字段（最多 200 个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldData>>,
}

/// 创建岗位信息请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 配置信息
    config: Config,
    /// 请求体
    body: CreateBody,
    /// 根据 client_token 是否一致来判断是否为同一请求
    client_token: Option<String>,
    /// 此次调用中使用的部门 ID 类型
    department_id_type: Option<String>,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: CreateBody::default(),
            client_token: None,
            department_id_type: None,
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: CreateBody) -> Self {
        self.body = body;
        self
    }

    /// 设置名称（必填，i18n 多语）。
    pub fn names(mut self, names: Vec<I18nLangValue>) -> Self {
        self.body.names = names;
        self
    }

    /// 设置职务 ID（必填）。
    pub fn job_id(mut self, job_id: impl Into<String>) -> Self {
        self.body.job_id = job_id.into();
        self
    }

    /// 设置部门 ID（必填，ID 类型须与 department_id_type 一致）。
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.body.department_id = department_id.into();
        self
    }

    /// 设置版本生效日期（必填，格式：YYYY-MM-DD）。
    pub fn effective_time(mut self, effective_time: impl Into<String>) -> Self {
        self.body.effective_time = effective_time.into();
        self
    }

    /// 设置岗位编码。
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.body.code = Some(code.into());
        self
    }

    /// 设置描述（i18n 多语）。
    pub fn descriptions(mut self, descriptions: Vec<I18nLangValue>) -> Self {
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

    /// 设置自定义字段。
    pub fn custom_fields(mut self, custom_fields: Vec<CustomFieldData>) -> Self {
        self.body.custom_fields = Some(custom_fields);
        self
    }

    /// 设置幂等 client_token。
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
    pub async fn execute(self) -> SDKResult<CreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        validate_required!(self.body.names, "names 不能为空");
        validate_required!(self.body.job_id, "job_id 不能为空");
        validate_required!(self.body.department_id, "department_id 不能为空");
        validate_required!(self.body.effective_time, "effective_time 不能为空");

        let request = ApiRequest::<CreateResponse>::post("/open-apis/corehr/v2/positions")
            .query_opt("client_token", self.client_token)
            .query_opt("department_id_type", self.department_id_type)
            .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 创建岗位信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 岗位 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
}

impl ApiResponseTrait for CreateResponse {
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

    /// 端到端：POST /open-apis/corehr/v2/positions
    /// （官方请求体必填 names/job_id/department_id/effective_time）
    #[tokio::test]
    async fn test_create_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/positions"))
            .and(query_param(
                "department_id_type",
                "people_corehr_department_id",
            ))
            .and(body_json(json!({
                "code": "A01234",
                "names": [
                    { "lang": "zh-CN", "value": "高级产品经理" }
                ],
                "descriptions": [
                    { "lang": "zh-CN", "value": "岗位的描述" }
                ],
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
                "custom_fields": [
                    { "custom_api_name": "name", "value": "\"231\"" }
                ]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "position_id": "12345678" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = CreateRequest::new(config)
            .code("A01234")
            .names(vec![I18nLangValue {
                lang: Some("zh-CN".to_string()),
                value: Some("高级产品经理".to_string()),
            }])
            .descriptions(vec![I18nLangValue {
                lang: Some("zh-CN".to_string()),
                value: Some("岗位的描述".to_string()),
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
                value: Some("\"231\"".to_string()),
            }])
            .department_id_type("people_corehr_department_id")
            .execute()
            .await
            .expect("请求应成功");

        assert_eq!(resp.position_id.as_deref(), Some("12345678"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
