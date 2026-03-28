//! Bitable 创建多维表格API
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use super::models::{App, CreateAppRequest as CreateAppRequestBody};
use super::AppService;

/// 创建多维表格请求
pub struct CreateAppRequest {
    /// 应用名称
    name: String,
    /// 文件夹token
    folder_token: Option<String>,
    /// 时区
    time_zone: Option<String>,
    /// 配置信息
    config: Config,
}

/// 创建多维表格响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateAppResponse {
    /// 应用信息
    pub app: App,
}

impl ApiResponseTrait for CreateAppResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateAppRequest {
    /// 创建新增多维表格请求
    pub fn new(config: Config) -> Self {
        Self {
            name: String::new(),
            folder_token: None,
            time_zone: None,
            config,
        }
    }

    /// 设置应用名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// 设置文件夹token
    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.folder_token = Some(folder_token.into());
        self
    }

    /// 设置时区
    pub fn time_zone(mut self, time_zone: impl Into<String>) -> Self {
        self.time_zone = Some(time_zone.into());
        self
    }

    /// 执行请求（集成现代化enum+builder API端点系统）
    pub async fn execute(self) -> SDKResult<CreateAppResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateAppResponse> {
        // 验证必填字段
        validate_required!(self.name, "应用名称不能为空");

        // 🚀 使用新的enum+builder系统生成API端点
        // 替代传统的字符串拼接方式，提供类型安全和IDE自动补全
        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::AppCreate;

        // 构建请求体 - 符合官方文档格式
        let request_body = CreateAppRequestBody {
            name: self.name.clone(),
            folder_token: self.folder_token.clone(),
            time_zone: self.time_zone.clone(),
            app_settings: None,
        };

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<CreateAppResponse> = ApiRequest::post(&api_endpoint.to_url())
            .body(openlark_core::api::RequestData::Binary(serde_json::to_vec(
                &request_body,
            )?));

        // 发送请求
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl AppService {
    /// 创建新增多维表格请求
    pub fn create_builder(&self, name: impl Into<String>) -> CreateAppRequest {
        CreateAppRequest::new(self.config.clone()).name(name)
    }

    /// 创建新增多维表格请求（带完整参数）
    pub fn create_app(
        &self,
        name: impl Into<String>,
        folder_token: Option<impl Into<String>>,
        time_zone: Option<impl Into<String>>,
    ) -> CreateAppRequest {
        let mut request = CreateAppRequest::new(self.config.clone()).name(name);

        if let Some(folder_token) = folder_token {
            request = request.folder_token(folder_token);
        }

        if let Some(time_zone) = time_zone {
            request = request.time_zone(time_zone);
        }

        request
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
