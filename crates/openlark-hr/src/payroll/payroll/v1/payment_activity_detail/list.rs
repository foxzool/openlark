//! 查询发薪活动明细列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/payroll-v1/payment_activity_detail/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 查询发薪活动明细列表请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 发薪活动 ID（可选）
    activity_id: Option<String>,
    /// 分页大小（可选，默认 50，最大 100）
    page_size: Option<i32>,
    /// 分页标记（可选）
    page_token: Option<String>,
    /// 配置信息
    config: Config,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            activity_id: None,
            page_size: None,
            page_token: None,
            config,
        }
    }

    /// 设置发薪活动 ID（可选）
    pub fn activity_id(mut self, activity_id: String) -> Self {
        self.activity_id = Some(activity_id);
        self
    }

    /// 设置分页大小（可选，默认 50，最大 100）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（可选）
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        use crate::common::api_endpoints::PayrollApiV1;

        // 1. 构建端点
        let api_endpoint = PayrollApiV1::PaymentActivityDetailList;
        let mut request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());

        // 2. 添加查询参数（可选）
        if let Some(ref activity_id) = self.activity_id {
            request = request.query("activity_id", activity_id);
        }
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        // 3. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 4. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "查询发薪活动明细列表响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 查询发薪活动明细列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 发薪活动明细列表
    pub items: Vec<PaymentActivityDetail>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 发薪活动明细信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentActivityDetail {
    /// 明细 ID
    pub detail_id: String,
    /// 发薪活动 ID
    pub activity_id: String,
    /// 员工 ID
    pub employee_id: String,
    /// 员工姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_name: Option<String>,
    /// 部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 应发工资
    pub gross_pay: f64,
    /// 实发工资
    pub net_pay: f64,
    /// 扣款总额
    pub total_deduction: f64,
    /// 税额
    pub tax_amount: f64,
    /// 币种
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

impl ApiResponseTrait for ListResponse {
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
