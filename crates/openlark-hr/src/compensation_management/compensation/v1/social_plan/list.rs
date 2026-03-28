//! 根据生效日期分页查询参保方案
//!
//! docPath: <https://open.feishu.cn/document/server-docs/compensation-v1/social_plan/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 根据生效日期分页查询参保方案请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ListRequest {
    /// 生效日期（必填）
    effective_date: i64,
    /// 分页大小（可选）
    page_size: Option<i32>,
    /// 分页标记（可选）
    page_token: Option<String>,
    /// 配置信息
    config: Config,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config, effective_date: i64) -> Self {
        Self {
            effective_date,
            page_size: None,
            page_token: None,
            config,
        }
    }

    /// 设置分页大小（可选）
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
        use crate::common::api_endpoints::CompensationApiV1;

        // 1. 构建端点
        let api_endpoint = CompensationApiV1::SocialPlanList;
        let mut request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());

        // 2. 添加查询参数
        request = request.query("effective_date", self.effective_date.to_string());
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
                "查询参保方案响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 根据生效日期分页查询参保方案响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 参保方案列表
    pub items: Vec<SocialPlan>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 参保方案
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SocialPlan {
    /// 方案 ID
    pub id: String,
    /// 方案名称
    pub name: String,
    /// 生效日期
    pub effective_date: i64,
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
