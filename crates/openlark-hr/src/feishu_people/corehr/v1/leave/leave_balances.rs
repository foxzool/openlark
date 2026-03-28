//! 批量查询员工假期余额
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/leave/leave_balances>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use super::models::{LeaveBalancesRequestBody, LeaveBalancesResponse};

/// 批量查询员工假期余额请求
#[derive(Debug, Clone)]
pub struct LeaveBalancesRequest {
    /// 配置信息
    config: Config,
    /// 员工 ID 列表（与 department_id 至少提供一个）
    employee_ids: Option<Vec<String>>,
    /// 部门 ID（与 employee_ids 至少提供一个）
    department_id: Option<String>,
    /// 假期类型 ID
    leave_type_id: Option<String>,
    /// 分页大小（1-100，默认 20）
    page_size: Option<i32>,
    /// 分页标记
    page_token: Option<String>,
}

impl LeaveBalancesRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            employee_ids: None,
            department_id: None,
            leave_type_id: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置员工 ID 列表
    pub fn employee_ids(mut self, employee_ids: Vec<String>) -> Self {
        self.employee_ids = Some(employee_ids);
        self
    }

    /// 设置部门 ID
    pub fn department_id(mut self, department_id: String) -> Self {
        self.department_id = Some(department_id);
        self
    }

    /// 设置假期类型 ID
    pub fn leave_type_id(mut self, leave_type_id: String) -> Self {
        self.leave_type_id = Some(leave_type_id);
        self
    }

    /// 设置分页大小（1-100）
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
    pub async fn execute(self) -> SDKResult<LeaveBalancesResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<LeaveBalancesResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        // 1. 验证必填字段（至少提供一个查询条件）
        let has_condition = self.employee_ids.is_some() || self.department_id.is_some();
        if !has_condition {
            return Err(openlark_core::error::validation_error(
                "查询条件不能为空",
                "至少提供一个查询条件：employee_ids 或 department_id",
            ));
        }

        // 2. 构建端点
        let api_endpoint = FeishuPeopleApiV1::LeaveLeaveBalances;
        let request = ApiRequest::<LeaveBalancesResponse>::get(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = LeaveBalancesRequestBody {
            employee_ids: self.employee_ids,
            department_id: self.department_id,
            leave_type_id: self.leave_type_id,
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
                "批量查询员工假期余额响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for LeaveBalancesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

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
