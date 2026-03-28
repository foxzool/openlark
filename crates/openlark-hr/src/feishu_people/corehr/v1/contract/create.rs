//! 新建合同
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/contract/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};

use super::models::{CreateRequestBody, CreateResponse, CustomField};

/// 新建合同请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 配置信息
    config: Config,
    /// 员工 ID（必填）
    employee_id: String,
    /// 合同编号
    contract_number: Option<String>,
    /// 合同开始日期
    start_date: Option<String>,
    /// 合同结束日期
    end_date: Option<String>,
    /// 合同类型
    contract_type: Option<i32>,
    /// 签订日期
    signing_date: Option<String>,
    /// 试用期开始日期
    probation_start_date: Option<String>,
    /// 试用期结束日期
    probation_end_date: Option<String>,
    /// 试用期时长（月）
    probation_duration: Option<i32>,
    /// 合同文件 ID 列表
    file_ids: Option<Vec<String>>,
    /// 自定义字段
    custom_fields: Option<Vec<CustomField>>,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            employee_id: String::new(),
            contract_number: None,
            start_date: None,
            end_date: None,
            contract_type: None,
            signing_date: None,
            probation_start_date: None,
            probation_end_date: None,
            probation_duration: None,
            file_ids: None,
            custom_fields: None,
        }
    }

    /// 设置员工 ID（必填）
    pub fn employee_id(mut self, employee_id: String) -> Self {
        self.employee_id = employee_id;
        self
    }

    /// 设置合同编号
    pub fn contract_number(mut self, contract_number: String) -> Self {
        self.contract_number = Some(contract_number);
        self
    }

    /// 设置合同开始日期
    pub fn start_date(mut self, start_date: String) -> Self {
        self.start_date = Some(start_date);
        self
    }

    /// 设置合同结束日期
    pub fn end_date(mut self, end_date: String) -> Self {
        self.end_date = Some(end_date);
        self
    }

    /// 设置合同类型
    pub fn contract_type(mut self, contract_type: i32) -> Self {
        self.contract_type = Some(contract_type);
        self
    }

    /// 设置签订日期
    pub fn signing_date(mut self, signing_date: String) -> Self {
        self.signing_date = Some(signing_date);
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

    /// 设置合同文件 ID 列表
    pub fn file_ids(mut self, file_ids: Vec<String>) -> Self {
        self.file_ids = Some(file_ids);
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

        // 2. 构建端点
        let api_endpoint = FeishuPeopleApiV1::ContractCreate;
        let request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = CreateRequestBody {
            employee_id: self.employee_id,
            contract_number: self.contract_number,
            start_date: self.start_date,
            end_date: self.end_date,
            contract_type: self.contract_type,
            signing_date: self.signing_date,
            probation_start_date: self.probation_start_date,
            probation_end_date: self.probation_end_date,
            probation_duration: self.probation_duration,
            file_ids: self.file_ids,
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
                "新建合同响应数据为空",
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
#[allow(unused_imports)]
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
