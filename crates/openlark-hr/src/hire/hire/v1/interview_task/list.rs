//! 获取面试任务列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/interview_task/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_token: Option<String>,
}

/// 获取面试任务列表请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ListRequest {
    /// 配置信息
    config: Config,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
        }
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        use crate::common::api_endpoints::HireApiV1;

        if let Some(size) = self.page_size {
            if !(1..=100).contains(&size) {
                return Err(openlark_core::error::validation_error(
                    "分页大小超出范围",
                    "page_size 必须在 1-100 之间",
                ));
            }
        }

        let api_endpoint = HireApiV1::InterviewTaskList;
        let request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());
        let request_body = ListRequestBody {
            page_size: self.page_size,
            page_token: self.page_token,
        };
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {}", e),
            )
        })?);
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取面试任务列表响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 获取面试任务列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
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
