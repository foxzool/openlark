//! 更新雇佣信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/employment/patch>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};

use super::models::{CustomField, PatchRequestBody, PatchResponse};

/// 更新雇佣信息请求
#[derive(Debug, Clone)]
pub struct PatchRequest {
    /// 配置信息
    config: Config,
    /// 雇佣 ID（必填）
    employment_id: String,
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

impl PatchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            employment_id: String::new(),
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

    /// 设置雇佣 ID（必填）
    pub fn employment_id(mut self, employment_id: String) -> Self {
        self.employment_id = employment_id;
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
    pub async fn execute(self) -> SDKResult<PatchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PatchResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        // 1. 验证必填字段
        validate_required!(self.employment_id.trim(), "雇佣 ID 不能为空");

        // 2. 构建端点
        let api_endpoint = FeishuPeopleApiV1::EmploymentPatch(self.employment_id.clone());
        let request = ApiRequest::<PatchResponse>::patch(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = PatchRequestBody {
            employment_id: self.employment_id,
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
                format!("无法序列化请求参数: {}", e),
            )
        })?);

        // 4. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 5. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "更新雇佣信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for PatchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::TestConfigBuilder;

    #[test]
    fn test_patch_request_builder_new() {
        let request =
            PatchRequest::new(TestConfigBuilder::new().build()).employment_id("test".to_string());
        let _ = request;
    }

    #[test]
    fn test_patch_request_validation_fails_on_default_request() {
        let request = PatchRequest::new(TestConfigBuilder::new().build());
        let rt = tokio::runtime::Runtime::new().expect("创建 tokio runtime 失败");
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }
}
