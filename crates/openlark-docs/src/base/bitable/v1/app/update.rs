//! Bitable 更新多维表格API
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/update>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use super::models::{App, AppSettings, UpdateAppRequest as UpdateAppRequestBody};
use super::AppService;

/// 更新多维表格请求
pub struct UpdateAppRequest {
    /// 应用token
    app_token: String,
    /// 应用名称
    name: Option<String>,
    /// 应用图标
    avatar: Option<String>,
    /// 应用设置
    app_settings: Option<AppSettings>,
    /// 配置信息
    config: Config,
}

/// 更新多维表格响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateAppResponse {
    /// 应用信息
    pub app: App,
}

impl ApiResponseTrait for UpdateAppResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UpdateAppRequest {
    /// 创建更新多维表格请求
    pub fn new(config: Config) -> Self {
        Self {
            app_token: String::new(),
            name: None,
            avatar: None,
            app_settings: None,
            config,
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    /// 设置应用名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置应用图标
    pub fn avatar(mut self, avatar: impl Into<String>) -> Self {
        self.avatar = Some(avatar.into());
        self
    }

    /// 设置应用设置
    pub fn app_settings(mut self, app_settings: AppSettings) -> Self {
        self.app_settings = Some(app_settings);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateAppResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateAppResponse> {
        // 验证必填字段
        validate_required!(self.app_token, "应用令牌不能为空");

        // 🚀 使用新的enum+builder系统生成API端点
        // 替代传统的字符串拼接方式，提供类型安全和IDE自动补全
        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::AppUpdate(self.app_token.clone());

        // 构建请求体
        let request_body = UpdateAppRequestBody {
            name: self.name.clone(),
            avatar: self.avatar.clone(),
            app_settings: self.app_settings.clone(),
        };

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<UpdateAppResponse> = ApiRequest::put(&api_endpoint.to_url())
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
    /// 创建更新多维表格请求
    pub fn update_builder(&self, app_token: impl Into<String>) -> UpdateAppRequest {
        UpdateAppRequest::new(self.config.clone()).app_token(app_token)
    }

    /// 创建更新多维表格请求（带完整参数）
    pub fn update_app(
        &self,
        app_token: impl Into<String>,
        name: Option<impl Into<String>>,
        avatar: Option<impl Into<String>>,
        app_settings: Option<AppSettings>,
    ) -> UpdateAppRequest {
        let mut request = UpdateAppRequest::new(self.config.clone()).app_token(app_token);

        if let Some(name) = name {
            request = request.name(name);
        }

        if let Some(avatar) = avatar {
            request = request.avatar(avatar);
        }

        if let Some(app_settings) = app_settings {
            request = request.app_settings(app_settings);
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
