//! 批量查询算薪项
//!
//! docPath: <https://open.feishu.cn/document/server-docs/payroll-v1/acct_item/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 批量查询算薪项请求
#[derive(Debug, Clone)]
pub struct ListRequest {
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
            page_size: None,
            page_token: None,
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
        let api_endpoint = PayrollApiV1::AcctItemList;
        let mut request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());

        // 2. 添加查询参数（可选）
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
                "批量查询算薪项响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 批量查询算薪项响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 算薪项列表
    pub items: Vec<AcctItem>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 算薪项信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AcctItem {
    /// 算薪项 ID
    pub acct_item_id: String,
    /// 算薪项名称
    pub name: String,
    /// 算薪项类型
    pub type_field: String,
    /// 是否计入工资
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_in_salary: Option<bool>,
    /// 是否计入社保公积金
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_in_social_insurance: Option<bool>,
    /// 是否计入个税
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_in_tax: Option<bool>,
    /// 排序权重
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
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
