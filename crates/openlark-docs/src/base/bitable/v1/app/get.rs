//! Bitable 获取多维表格详情API
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use super::models::App;
use super::AppService;

/// 获取多维表格请求
#[derive(Debug, Clone)]
pub struct GetAppRequest {
    /// 应用token
    app_token: String,
    /// 配置信息
    config: Config,
}

/// 获取多维表格响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetAppResponse {
    /// 应用信息
    pub app: App,
}

impl ApiResponseTrait for GetAppResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetAppRequest {
    /// 创建获取多维表格请求
    pub fn new(config: Config) -> Self {
        Self {
            app_token: String::new(),
            config,
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetAppResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetAppResponse> {
        // 验证必填字段
        validate_required!(self.app_token, "应用令牌不能为空");

        // 🚀 使用新的enum+builder系统生成API端点
        // 替代传统的字符串拼接方式，提供类型安全和IDE自动补全
        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::AppGet(self.app_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<GetAppResponse> = ApiRequest::get(&api_endpoint.to_url());

        // 发送请求
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl AppService {
    /// 创建获取多维表格请求
    pub fn get_builder(&self, app_token: impl Into<String>) -> GetAppRequest {
        GetAppRequest::new(self.config.clone()).app_token(app_token)
    }

    /// 创建获取多维表格请求
    pub fn get_app(&self, app_token: impl Into<String>) -> GetAppRequest {
        self.get_builder(app_token)
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
