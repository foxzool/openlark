//! 获取告警记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/alert/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::extract_response_data;

/// 获取告警记录请求

#[derive(Debug, Clone)]
pub struct ListAlertRequest {
    /// 配置信息
    config: Config,
    /// 查询参数
    query_params: Vec<(String, String)>,
}

/// 获取告警记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListAlertResponse {
    /// 告警列表
    pub alerts: Vec<AlertItem>,
    /// 是否有下一页
    pub has_more: Option<bool>,
}

/// 告警项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AlertItem {
    /// 告警 ID
    pub alert_id: String,
    /// 告警级别
    pub level: String,
    /// 告警内容
    pub content: String,
}

impl ApiResponseTrait for ListAlertResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListAlertRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query_params: Vec::new(),
        }
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/alert/list>
    pub async fn execute(self) -> SDKResult<ListAlertResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<ListAlertResponse> {
        let api_endpoint = VcApiV1::AlertList;
        let mut api_request: ApiRequest<ListAlertResponse> = ApiRequest::get(api_endpoint.to_url());

        for (key, value) in self.query_params {
            api_request = api_request.query(key, value);
        }

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取告警记录")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = ListAlertRequest::new(config.clone())
            .query_param("test".to_string(), "value".to_string());
        let _ = request;
    }
}
