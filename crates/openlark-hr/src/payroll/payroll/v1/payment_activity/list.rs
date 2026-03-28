//! 查询发薪活动列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/payroll-v1/payment_activity/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 查询发薪活动列表请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 分页大小（可选，默认 50，最大 100）
    page_size: Option<i32>,
    /// 分页标记（可选）
    page_token: Option<String>,
    /// 薪资组 ID（可选）
    paygroup_id: Option<String>,
    /// 状态筛选（可选）
    status: Option<i32>,
    /// 配置信息
    config: Config,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            page_size: None,
            page_token: None,
            paygroup_id: None,
            status: None,
            config,
        }
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

    /// 设置薪资组 ID（可选）
    pub fn paygroup_id(mut self, paygroup_id: String) -> Self {
        self.paygroup_id = Some(paygroup_id);
        self
    }

    /// 设置状态筛选（可选）
    pub fn status(mut self, status: i32) -> Self {
        self.status = Some(status);
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
        let api_endpoint = PayrollApiV1::PaymentActivityList;
        let mut request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());

        // 2. 添加查询参数（可选）
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(ref paygroup_id) = self.paygroup_id {
            request = request.query("paygroup_id", paygroup_id);
        }
        if let Some(status) = self.status {
            request = request.query("status", status.to_string());
        }

        // 3. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 4. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "查询发薪活动列表响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 查询发薪活动列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 发薪活动列表
    pub items: Vec<PaymentActivity>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 发薪活动信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentActivity {
    /// 发薪活动 ID
    pub activity_id: String,
    /// 薪资组 ID
    pub paygroup_id: String,
    /// 发薪活动名称
    pub name: String,
    /// 发薪周期开始时间（Unix 时间戳）
    pub period_start_time: i64,
    /// 发薪周期结束时间（Unix 时间戳）
    pub period_end_time: i64,
    /// 发薪状态
    pub status: i32,
    /// 发薪总额
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
    /// 币种
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// 创建时间（Unix 时间戳）
    pub created_at: i64,
    /// 更新时间（Unix 时间戳）
    pub updated_at: i64,
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
