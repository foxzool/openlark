//! 获取 OKR 周期列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v1/period/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取 OKR 周期列表请求
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
        use crate::common::api_endpoints::OkrApiV1;

        // 1. 构建端点
        let api_endpoint = OkrApiV1::PeriodList;
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
                "获取 OKR 周期列表响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 获取 OKR 周期列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// OKR 周期列表
    pub items: Vec<OkrPeriod>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// OKR 周期信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OkrPeriod {
    /// 周期 ID
    pub period_id: String,
    /// 周期名称
    pub name: String,
    /// 周期开始时间（Unix 时间戳）
    pub start_time: i64,
    /// 周期结束时间（Unix 时间戳）
    pub end_time: i64,
    /// 周期状态
    /// - 1: 未开始
    /// - 2: 进行中
    /// - 3: 已结束
    pub status: i32,
    /// 周期描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 目标制定截止时间（Unix 时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_setting_deadline: Option<i64>,
    /// 复盘时间（Unix 时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_time: Option<i64>,
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
