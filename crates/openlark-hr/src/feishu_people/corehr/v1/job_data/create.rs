//! 创建任职信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/job_data/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};

use super::models::{CreateRequestBody, CreateResponse, EnumValue};

/// 创建任职信息请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 配置信息
    config: Config,
    /// 员工 ID（必填）
    employee_id: String,
    /// 人员类型 ID（必填）
    employee_type_id: String,
    /// 部门 ID（必填）
    department_id: String,
    /// 是否为主任职（必填，该字段已废弃，固定为 true）
    primary_job_data: bool,
    /// 雇佣 ID（必填）
    employment_id: String,
    /// 生效时间（必填，格式：YYYY-MM-DD HH:mm:ss）
    effective_time: String,
    /// 业务类型/任职原因（必填，仅支持 onboarding）
    assignment_start_reason: Option<EnumValue>,
    /// 职务 ID（必填）
    job_id: String,
    /// 职位 ID
    position_id: Option<String>,
    /// 开始日期（格式：YYYY-MM-DD）
    start_date: Option<String>,
    /// 结束日期（格式：YYYY-MM-DD）
    end_date: Option<String>,
    /// 状态
    /// - 1: 在职
    /// - 2: 离职
    status: Option<i32>,
    /// 自定义字段
    custom_fields: Option<Vec<super::models::CustomField>>,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            employee_id: String::new(),
            employee_type_id: String::new(),
            department_id: String::new(),
            primary_job_data: true,
            employment_id: String::new(),
            effective_time: String::new(),
            assignment_start_reason: None,
            job_id: String::new(),
            position_id: None,
            start_date: None,
            end_date: None,
            status: None,
            custom_fields: None,
        }
    }

    /// 设置员工 ID（必填）
    pub fn employee_id(mut self, employee_id: String) -> Self {
        self.employee_id = employee_id;
        self
    }

    /// 设置人员类型 ID（必填，可通过批量查询人员类型接口获得）
    pub fn employee_type_id(mut self, employee_type_id: String) -> Self {
        self.employee_type_id = employee_type_id;
        self
    }

    /// 设置部门 ID（必填，可通过批量查询部门接口获得）
    pub fn department_id(mut self, department_id: String) -> Self {
        self.department_id = department_id;
        self
    }

    /// 设置是否为主任职（必填，该字段已废弃，固定为 true）
    pub fn primary_job_data(mut self, primary_job_data: bool) -> Self {
        self.primary_job_data = primary_job_data;
        self
    }

    /// 设置雇佣 ID（必填，可通过搜索员工信息接口获得）
    pub fn employment_id(mut self, employment_id: String) -> Self {
        self.employment_id = employment_id;
        self
    }

    /// 设置生效时间（必填，格式：YYYY-MM-DD HH:mm:ss）
    pub fn effective_time(mut self, effective_time: String) -> Self {
        self.effective_time = effective_time;
        self
    }

    /// 设置业务类型/任职原因（必填，仅支持 onboarding）
    pub fn assignment_start_reason(mut self, enum_name: String) -> Self {
        self.assignment_start_reason = Some(EnumValue {
            enum_name: Some(enum_name),
        });
        self
    }

    /// 设置职务 ID（必填）
    pub fn job_id(mut self, job_id: String) -> Self {
        self.job_id = job_id;
        self
    }

    /// 设置职位 ID
    pub fn position_id(mut self, position_id: String) -> Self {
        self.position_id = Some(position_id);
        self
    }

    /// 设置开始日期（格式：YYYY-MM-DD）
    pub fn start_date(mut self, start_date: String) -> Self {
        self.start_date = Some(start_date);
        self
    }

    /// 设置结束日期（格式：YYYY-MM-DD）
    pub fn end_date(mut self, end_date: String) -> Self {
        self.end_date = Some(end_date);
        self
    }

    /// 设置状态
    /// - 1: 在职
    /// - 2: 离职
    pub fn status(mut self, status: i32) -> Self {
        self.status = Some(status);
        self
    }

    /// 设置自定义字段
    pub fn custom_fields(mut self, custom_fields: Vec<super::models::CustomField>) -> Self {
        self.custom_fields = Some(custom_fields);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        // 1. 验证必填字段
        validate_required!(self.employee_id.trim(), "员工 ID 不能为空");
        validate_required!(self.employee_type_id.trim(), "人员类型 ID 不能为空");
        validate_required!(self.department_id.trim(), "部门 ID 不能为空");
        validate_required!(self.employment_id.trim(), "雇佣 ID 不能为空");
        validate_required!(self.effective_time.trim(), "生效时间不能为空");
        validate_required!(self.job_id.trim(), "职务 ID 不能为空");
        let assignment_start_reason = self.assignment_start_reason.ok_or_else(|| {
            openlark_core::error::validation_error(
                "assignment_start_reason 不能为空",
                "创建任职信息必须提供业务类型（仅支持 onboarding）",
            )
        })?;

        // 2. 构建端点
        let api_endpoint = FeishuPeopleApiV1::JobDataCreate;
        let request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = CreateRequestBody {
            employee_id: self.employee_id,
            employee_type_id: self.employee_type_id,
            department_id: self.department_id,
            primary_job_data: self.primary_job_data,
            employment_id: self.employment_id,
            effective_time: self.effective_time,
            assignment_start_reason,
            job_id: self.job_id,
            position_id: self.position_id,
            start_date: self.start_date,
            end_date: self.end_date,
            status: self.status,
            custom_fields: self.custom_fields,
        };
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);

        // 4. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "创建任职信息响应数据为空",
        )
        .await
    }
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
    use wiremock::matchers::{body_partial_json, method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v1/job_datas
    #[tokio::test]
    async fn test_create_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v1/job_datas"))
            .and(body_partial_json(json!({
                "employee_type_id": "6890452208593372679",
                "department_id": "6890452208593372679",
                "primary_job_data": true,
                "employment_id": "6893014062142064135",
                "effective_time": "2020-05-01 00:00:00",
                "assignment_start_reason": { "enum_name": "onboarding" }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "job_data_id": "" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        CreateRequest::new(config)
            .employee_id("test001".to_string())
            .employee_type_id("6890452208593372679".to_string())
            .department_id("6890452208593372679".to_string())
            .employment_id("6893014062142064135".to_string())
            .effective_time("2020-05-01 00:00:00".to_string())
            .assignment_start_reason("onboarding".to_string())
            .job_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
