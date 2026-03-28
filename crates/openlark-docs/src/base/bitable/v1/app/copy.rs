//! Bitable 复制多维表格API
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/copy>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use super::models::{App, CopyAppRequest as CopyAppRequestBody};
use super::AppService;

/// 复制多维表格请求
pub struct CopyAppRequest {
    /// 应用token
    app_token: String,
    /// 新应用名称
    name: Option<String>,
    /// 目标文件夹token
    folder_token: Option<String>,
    /// 是否复制内容（true: 不复制内容，false: 复制内容）
    without_content: Option<bool>,
    /// 时区
    time_zone: Option<String>,
    /// 配置信息
    config: Config,
}

/// 复制多维表格响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CopyAppResponse {
    /// 应用信息
    pub app: App,
}

impl ApiResponseTrait for CopyAppResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CopyAppRequest {
    /// 创建复制多维表格请求
    pub fn new(config: Config) -> Self {
        Self {
            app_token: String::new(),
            name: None,
            folder_token: None,
            without_content: None,
            time_zone: None,
            config,
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    /// 设置新应用名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置目标文件夹token
    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.folder_token = Some(folder_token.into());
        self
    }

    /// 设置是否复制内容（true: 不复制内容，false: 复制内容）
    pub fn without_content(mut self, without_content: bool) -> Self {
        self.without_content = Some(without_content);
        self
    }

    /// 设置时区
    pub fn time_zone(mut self, time_zone: impl Into<String>) -> Self {
        self.time_zone = Some(time_zone.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CopyAppResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<CopyAppResponse> {
        // 验证必填字段
        validate_required!(self.app_token, "应用令牌不能为空");

        // 🚀 使用新的enum+builder系统生成API端点
        // 替代传统的字符串拼接方式，提供类型安全和IDE自动补全
        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::AppCopy(self.app_token.clone());

        // 构建请求体
        let request_body = CopyAppRequestBody {
            name: self.name.clone(),
            folder_token: self.folder_token.clone(),
            without_content: self.without_content,
            time_zone: self.time_zone.clone(),
        };

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<CopyAppResponse> = ApiRequest::post(&api_endpoint.to_url())
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
    /// 创建复制多维表格请求
    pub fn copy_builder(&self, app_token: impl Into<String>) -> CopyAppRequest {
        CopyAppRequest::new(self.config.clone()).app_token(app_token)
    }

    /// 创建复制多维表格请求（带完整参数）
    pub fn copy_app(
        &self,
        app_token: impl Into<String>,
        name: Option<impl Into<String>>,
        folder_token: Option<impl Into<String>>,
        without_content: Option<bool>,
        time_zone: Option<impl Into<String>>,
    ) -> CopyAppRequest {
        let mut request = CopyAppRequest::new(self.config.clone()).app_token(app_token);

        if let Some(name) = name {
            request = request.name(name);
        }

        if let Some(folder_token) = folder_token {
            request = request.folder_token(folder_token);
        }

        if let Some(without_content) = without_content {
            request = request.without_content(without_content);
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
