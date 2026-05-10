//! 订阅云文档事件
//!
//! 订阅文件的更新通知。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/event/subscribe

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 订阅文件请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeFileRequest {
    #[serde(skip)]
    config: Config,
    /// 文件token
    pub file_token: String,
    /// 文件类型
    pub file_type: String,
    /// 订阅类型
    pub event_type: Option<String>,
}

impl SubscribeFileRequest {
    /// 创建订阅文件请求
    ///
    /// # 参数
    /// * `config` - 配置
    /// * `file_token` - 文件token
    /// * `file_type` - 文件类型（doc/docx/sheet/bitable/file/folder/slides）
    pub fn new(
        config: Config,
        file_token: impl Into<String>,
        file_type: impl Into<String>,
    ) -> Self {
        Self {
            config,
            file_token: file_token.into(),
            file_type: file_type.into(),
            event_type: None,
        }
    }

    /// 设置订阅类型
    pub fn event_type(mut self, event_type: impl Into<String>) -> Self {
        self.event_type = Some(event_type.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<SubscribeFileResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SubscribeFileResponse> {
        // ===== 参数校验 =====
        validate_required!(self.file_token, "file_token 不能为空");
        validate_required!(self.file_type, "file_type 不能为空");
        match self.file_type.as_str() {
            "doc" | "docx" | "sheet" | "bitable" | "file" | "folder" | "slides" => {}
            _ => {
                return Err(openlark_core::error::validation_error(
                    "file_type",
                    "file_type 仅支持 doc/docx/sheet/bitable/file/folder/slides",
                ));
            }
        }

        // 文档约束：仅当 file_type 为 folder 时，允许传 event_type，且必须为 file.created_in_folder_v1
        if self.file_type == "folder" {
            match self.event_type.as_deref() {
                Some("file.created_in_folder_v1") => {}
                Some(_) => {
                    return Err(openlark_core::error::validation_error(
                        "event_type",
                        "当 file_type=folder 时，event_type 必须为 file.created_in_folder_v1",
                    ));
                }
                None => {
                    return Err(openlark_core::error::validation_error(
                        "event_type",
                        "当 file_type=folder 时，event_type 不能为空（必须为 file.created_in_folder_v1）",
                    ));
                }
            }
        } else if self.event_type.is_some() {
            return Err(openlark_core::error::validation_error(
                "event_type",
                "当 file_type 不为 folder 时，请勿传入 event_type",
            ));
        }

        // ===== 构建请求 =====
        let api_endpoint = DriveApi::SubscribeFile(self.file_token.clone());
        let mut request = ApiRequest::<SubscribeFileResponse>::post(&api_endpoint.to_url());

        request = request.query("file_type", &self.file_type);
        if let Some(et) = &self.event_type {
            request = request.query("event_type", et);
        }

        // ===== 发送请求 =====
        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "订阅文件")
    }
}

/// 订阅文件响应
///
/// 文档响应体 `data` 为空对象 `{}`。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscribeFileResponse {}

impl ApiResponseTrait for SubscribeFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试构建器模式
    #[test]
    fn test_subscribe_file_request_builder() {
        let config = Config::default();
        let request = SubscribeFileRequest::new(config, "file_token", "docx");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.file_type, "docx");
    }

    /// 测试响应数据结构
    #[test]
    fn test_subscribe_data_structure() {
        let _subscribe_data = SubscribeFileResponse {};
    }

    /// 测试响应trait实现
    #[test]
    fn test_response_trait() {
        assert_eq!(SubscribeFileResponse::data_format(), ResponseFormat::Data);
    }

    /// 测试设置event_type
    #[test]
    fn test_subscribe_with_event_type() {
        let config = Config::default();
        let request = SubscribeFileRequest::new(config, "folder_token", "folder")
            .event_type("file.created_in_folder_v1");

        assert_eq!(
            request.event_type,
            Some("file.created_in_folder_v1".to_string())
        );
    }

    /// 测试folder类型订阅
    #[test]
    fn test_subscribe_folder_type() {
        let config = Config::default();
        let request = SubscribeFileRequest::new(config, "folder_token", "folder");

        assert_eq!(request.file_type, "folder");
    }

    /// 测试sheet类型订阅
    #[test]
    fn test_subscribe_sheet_type() {
        let config = Config::default();
        let request = SubscribeFileRequest::new(config, "sheet_token", "sheet");

        assert_eq!(request.file_type, "sheet");
        assert!(request.event_type.is_none());
    }
}
