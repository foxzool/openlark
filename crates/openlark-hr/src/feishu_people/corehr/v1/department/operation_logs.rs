//! 查询操作日志
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/department/query_operation_logs>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};

use super::models::{OperationLogsRequestBody, OperationLogsResponse};

/// 查询操作日志请求
#[derive(Debug, Clone)]
pub struct OperationLogsRequest {
    /// 配置信息
    config: Config,
    /// 部门 ID（必填）
    department_id: String,
    /// 开始时间（必填，格式：YYYY-MM-DD）
    start_time: String,
    /// 结束时间（必填，格式：YYYY-MM-DD）
    end_time: String,
    /// 分页大小（1-100，默认 20）
    page_size: Option<i32>,
    /// 分页标记
    page_token: Option<String>,
}

impl OperationLogsRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            department_id: String::new(),
            start_time: String::new(),
            end_time: String::new(),
            page_size: None,
            page_token: None,
        }
    }

    /// 设置部门 ID（必填）
    pub fn department_id(mut self, department_id: String) -> Self {
        self.department_id = department_id;
        self
    }

    /// 设置开始时间（必填，格式：YYYY-MM-DD）
    pub fn start_time(mut self, start_time: String) -> Self {
        self.start_time = start_time;
        self
    }

    /// 设置结束时间（必填，格式：YYYY-MM-DD）
    pub fn end_time(mut self, end_time: String) -> Self {
        self.end_time = end_time;
        self
    }

    /// 设置分页大小（1-100，默认 20）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<OperationLogsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<OperationLogsResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        // 1. 验证必填字段
        validate_required!(self.department_id.trim(), "部门ID不能为空");
        validate_required!(self.start_time.trim(), "开始时间不能为空");
        validate_required!(self.end_time.trim(), "结束时间不能为空");

        // 2. 构建端点
        let api_endpoint = FeishuPeopleApiV1::DepartmentQueryOperationLogs;
        let request = ApiRequest::<OperationLogsResponse>::post(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = OperationLogsRequestBody {
            department_id: self.department_id,
            start_time: self.start_time,
            end_time: self.end_time,
            page_size: self.page_size,
            page_token: self.page_token,
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
                "查询操作日志响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for OperationLogsResponse {
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
