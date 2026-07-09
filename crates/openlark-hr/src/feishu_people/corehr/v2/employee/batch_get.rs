//! 批量查询员工信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/employee/batch_get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};

use serde::{Deserialize, Serialize};

/// 批量查询员工信息请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchGetRequest {
    /// 员工 ID 列表
    pub employee_ids: Vec<String>,
}

impl BatchGetRequest {
    /// 创建请求
    pub fn new(employee_ids: Vec<String>) -> Self {
        Self { employee_ids }
    }

    /// 添加员工 ID
    pub fn add_employee_id(mut self, employee_id: String) -> Self {
        self.employee_ids.push(employee_id);
        self
    }
}

/// 批量查询员工信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetResponse {
    /// 员工列表
    pub data: Option<BatchGetResponseData>,
}

/// `BatchGetResponseData`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetResponseData {
    /// 员工列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<EmployeeItem>>,
}

/// `EmployeeItem`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmployeeItem {
    /// 员工 ID
    pub id: String,
    /// 姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 电话
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 职位 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    /// 工号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_no: Option<String>,
    /// 入职日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hire_date: Option<String>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl ApiResponseTrait for BatchGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量查询员工信息请求构建器
#[derive(Debug, Clone)]
pub struct BatchGetRequestBuilder {
    config: Config,
    request: BatchGetRequest,
}

impl BatchGetRequestBuilder {
    /// 创建请求构建器
    pub fn new(config: Config, employee_ids: Vec<String>) -> Self {
        Self {
            config,
            request: BatchGetRequest::new(employee_ids),
        }
    }

    /// 添加员工 ID
    pub fn add_employee_id(mut self, employee_id: String) -> Self {
        self.request = self.request.add_employee_id(employee_id);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchGetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchGetResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        // 构建端点
        let api_endpoint = FeishuPeopleApiV2::EmployeeBatchGet;
        let request = ApiRequest::<BatchGetResponse>::post(api_endpoint.to_url());

        // 序列化请求体
        let request = request.body(serde_json::to_value(&self.request).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);

        // 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "批量查询员工信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/employees/batch_get
    #[tokio::test]
    async fn test_batch_get_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/employees/batch_get"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {  }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        BatchGetRequestBuilder::new(config, vec!["test001".to_string()])
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
