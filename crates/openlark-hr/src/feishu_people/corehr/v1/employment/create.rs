//! 创建雇佣信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/employment/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};

use super::models::{CreateRequestBody, CreateResponse, CustomField, EnumValue};

/// 创建雇佣信息请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 配置信息
    config: Config,
    /// 员工 ID（必填）
    employee_id: String,
    /// 入职日期（格式：YYYY-MM-DD HH:mm:ss，必填）
    effective_time: String,
    /// 个人信息 ID（必填，由创建个人信息接口生成）
    person_id: String,
    /// 是否是主雇佣信息（必填）
    primary_employment: bool,
    /// 雇佣状态（必填，枚举值如 hired）
    employment_status: Option<EnumValue>,
    /// 部门 ID
    department_id: Option<String>,
    /// 职位 ID
    position_id: Option<String>,
    /// 入职日期
    start_date: Option<String>,
    /// 离职日期
    end_date: Option<String>,
    /// 雇佣状态
    status: Option<i32>,
    /// 雇佣类型
    employment_type: Option<i32>,
    /// 试用期开始日期
    probation_start_date: Option<String>,
    /// 试用期结束日期
    probation_end_date: Option<String>,
    /// 试用期时长（月）
    probation_duration: Option<i32>,
    /// 工作地点
    work_location: Option<String>,
    /// 自定义字段
    custom_fields: Option<Vec<CustomField>>,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            employee_id: String::new(),
            effective_time: String::new(),
            person_id: String::new(),
            primary_employment: true,
            employment_status: None,
            department_id: None,
            position_id: None,
            start_date: None,
            end_date: None,
            status: None,
            employment_type: None,
            probation_start_date: None,
            probation_end_date: None,
            probation_duration: None,
            work_location: None,
            custom_fields: None,
        }
    }

    /// 设置员工 ID（必填）
    pub fn employee_id(mut self, employee_id: String) -> Self {
        self.employee_id = employee_id;
        self
    }

    /// 设置入职日期（必填，格式：YYYY-MM-DD HH:mm:ss）
    pub fn effective_time(mut self, effective_time: String) -> Self {
        self.effective_time = effective_time;
        self
    }

    /// 设置个人信息 ID（必填，由创建个人信息接口生成）
    pub fn person_id(mut self, person_id: String) -> Self {
        self.person_id = person_id;
        self
    }

    /// 设置是否是主雇佣信息（必填）
    pub fn primary_employment(mut self, primary_employment: bool) -> Self {
        self.primary_employment = primary_employment;
        self
    }

    /// 设置雇佣状态（必填，枚举值如 hired）
    pub fn employment_status(mut self, enum_name: String) -> Self {
        self.employment_status = Some(EnumValue {
            enum_name: Some(enum_name),
        });
        self
    }

    /// 设置部门 ID
    pub fn department_id(mut self, department_id: String) -> Self {
        self.department_id = Some(department_id);
        self
    }

    /// 设置职位 ID
    pub fn position_id(mut self, position_id: String) -> Self {
        self.position_id = Some(position_id);
        self
    }

    /// 设置入职日期
    pub fn start_date(mut self, start_date: String) -> Self {
        self.start_date = Some(start_date);
        self
    }

    /// 设置离职日期
    pub fn end_date(mut self, end_date: String) -> Self {
        self.end_date = Some(end_date);
        self
    }

    /// 设置雇佣状态
    pub fn status(mut self, status: i32) -> Self {
        self.status = Some(status);
        self
    }

    /// 设置雇佣类型
    pub fn employment_type(mut self, employment_type: i32) -> Self {
        self.employment_type = Some(employment_type);
        self
    }

    /// 设置试用期开始日期
    pub fn probation_start_date(mut self, probation_start_date: String) -> Self {
        self.probation_start_date = Some(probation_start_date);
        self
    }

    /// 设置试用期结束日期
    pub fn probation_end_date(mut self, probation_end_date: String) -> Self {
        self.probation_end_date = Some(probation_end_date);
        self
    }

    /// 设置试用期时长（月）
    pub fn probation_duration(mut self, probation_duration: i32) -> Self {
        self.probation_duration = Some(probation_duration);
        self
    }

    /// 设置工作地点
    pub fn work_location(mut self, work_location: String) -> Self {
        self.work_location = Some(work_location);
        self
    }

    /// 设置自定义字段
    pub fn custom_fields(mut self, custom_fields: Vec<CustomField>) -> Self {
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
        validate_required!(self.effective_time.trim(), "入职日期不能为空");
        validate_required!(self.person_id.trim(), "个人信息 ID 不能为空");
        let employment_status = self.employment_status.ok_or_else(|| {
            openlark_core::error::validation_error(
                "employment_status 不能为空",
                "创建雇佣信息必须提供雇佣状态（如 hired）",
            )
        })?;

        // 2. 构建端点
        let api_endpoint = FeishuPeopleApiV1::EmploymentCreate;
        let request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = CreateRequestBody {
            employee_id: self.employee_id,
            effective_time: self.effective_time,
            person_id: self.person_id,
            primary_employment: self.primary_employment,
            employment_status,
            department_id: self.department_id,
            position_id: self.position_id,
            start_date: self.start_date,
            end_date: self.end_date,
            status: self.status,
            employment_type: self.employment_type,
            probation_start_date: self.probation_start_date,
            probation_end_date: self.probation_end_date,
            probation_duration: self.probation_duration,
            work_location: self.work_location,
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
            "创建雇佣信息响应数据为空",
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
    use openlark_core::testing::prelude::TestConfigBuilder;

    #[test]
    fn test_create_request_builder_new() {
        let request =
            CreateRequest::new(TestConfigBuilder::new().build()).employee_id("test".to_string());
        let _ = request;
    }

    #[test]
    fn test_create_request_validation_fails_on_default_request() {
        let request = CreateRequest::new(TestConfigBuilder::new().build());
        let rt = tokio::runtime::Runtime::new().expect("创建 tokio runtime 失败");
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }

    /// 端到端：POST /open-apis/corehr/v1/employments
    #[tokio::test]
    async fn test_create_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_partial_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v1/employments"))
            .and(body_partial_json(json!({
                "effective_time": "2020-01-01 00:00:00",
                "person_id": "6919733936050406926",
                "primary_employment": true,
                "employment_status": { "enum_name": "hired" }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "employment_id": "" }
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
            .effective_time("2020-01-01 00:00:00".to_string())
            .person_id("6919733936050406926".to_string())
            .primary_employment(true)
            .employment_status("hired".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
