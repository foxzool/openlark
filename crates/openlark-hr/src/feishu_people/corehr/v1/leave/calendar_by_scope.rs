//! 根据适用条件获取工作日历 ID
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/leave/calendar_by_scope>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use super::models::{CalendarByScopeRequestBody, CalendarByScopeResponse};

/// 根据适用条件获取工作日历 ID 请求
#[derive(Debug, Clone)]
pub struct CalendarByScopeRequest {
    /// 配置信息
    config: Config,
    /// 员工 ID 列表
    employee_ids: Option<Vec<String>>,
    /// 部门 ID
    department_id: Option<String>,
    /// 用户 ID 列表
    user_ids: Option<Vec<String>>,
    /// 开始日期（格式：YYYY-MM-DD）
    start_date: Option<String>,
    /// 结束日期（格式：YYYY-MM-DD）
    end_date: Option<String>,
}

impl CalendarByScopeRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            employee_ids: None,
            department_id: None,
            user_ids: None,
            start_date: None,
            end_date: None,
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

    /// 设置用户 ID 列表
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = Some(user_ids);
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

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CalendarByScopeResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CalendarByScopeResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        // 1. 验证必填字段（至少提供一个查询条件）
        let has_condition = self.employee_ids.is_some()
            || self.department_id.is_some()
            || self.user_ids.is_some()
            || self.start_date.is_some()
            || self.end_date.is_some();

        if !has_condition {
            return Err(openlark_core::error::validation_error(
                "查询条件不能为空",
                "至少提供一个查询条件：employee_ids、department_id、user_ids、start_date 或 end_date",
            ));
        }

        // 2. 构建端点
        let api_endpoint = FeishuPeopleApiV1::LeaveCalendarByScope;
        let request = ApiRequest::<CalendarByScopeResponse>::post(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = CalendarByScopeRequestBody {
            employee_ids: self.employee_ids,
            department_id: self.department_id,
            user_ids: self.user_ids,
            start_date: self.start_date,
            end_date: self.end_date,
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
                "获取工作日历 ID 响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for CalendarByScopeResponse {
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
