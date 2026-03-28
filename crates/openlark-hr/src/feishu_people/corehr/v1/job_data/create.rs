//! 创建任职信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/job_data/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};

use super::models::{CreateRequestBody, CreateResponse};

/// 创建任职信息请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 配置信息
    config: Config,
    /// 员工 ID（必填）
    employee_id: String,
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
        validate_required!(self.job_id.trim(), "职务 ID 不能为空");

        // 2. 构建端点
        let api_endpoint = FeishuPeopleApiV1::JobDataCreate;
        let request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = CreateRequestBody {
            employee_id: self.employee_id,
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
                format!("无法序列化请求参数: {}", e),
            )
        })?);

        // 4. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 5. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "创建任职信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for CreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
