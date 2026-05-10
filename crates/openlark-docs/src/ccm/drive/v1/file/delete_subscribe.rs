//! 取消云文档事件订阅
//!
//! 该接口**仅支持文档拥有者**取消订阅自己文档的通知事件，可订阅的文档类型为**旧版文档**、**新版文档**、**电子表格**和**多维表格**。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/event/delete_subscribe

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 取消云文档事件订阅请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSubscribeRequest {
    #[serde(skip)]
    config: Config,
    /// 文件token
    pub file_token: String,
    /// 文件类型
    pub file_type: String,
    /// 事件类型（仅当 file_type 为 folder 时需要传入 file.created_in_folder_v1）
    pub event_type: Option<String>,
}

impl DeleteSubscribeRequest {
    /// 创建取消云文档事件订阅请求
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

    /// 设置事件类型
    pub fn event_type(mut self, event_type: impl Into<String>) -> Self {
        self.event_type = Some(event_type.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<DeleteSubscribeResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteSubscribeResponse> {
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
        let api_endpoint = DriveApi::DeleteFileSubscribe(self.file_token.clone());
        let mut request = ApiRequest::<DeleteSubscribeResponse>::delete(&api_endpoint.to_url());

        request = request.query("file_type", &self.file_type);
        if let Some(event_type) = &self.event_type {
            request = request.query("event_type", event_type);
        }

        // ===== 发送请求 =====
        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "订阅文件")
    }
}

/// 取消云文档事件订阅响应
///
/// 文档响应体 `data` 为空对象 `{}`。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteSubscribeResponse {}

impl ApiResponseTrait for DeleteSubscribeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试构建器模式
    #[test]
    fn test_delete_subscribe_request_builder() {
        let config = Config::default();
        let request = DeleteSubscribeRequest::new(config, "file_token", "docx");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.file_type, "docx");
    }

    /// 测试响应trait实现
    #[test]
    fn test_response_trait() {
        assert_eq!(DeleteSubscribeResponse::data_format(), ResponseFormat::Data);
    }

    /// 测试设置event_type
    #[test]
    fn test_delete_subscribe_with_event_type() {
        let config = Config::default();
        let request = DeleteSubscribeRequest::new(config, "folder_token", "folder")
            .event_type("file.created_in_folder_v1");

        assert_eq!(
            request.event_type,
            Some("file.created_in_folder_v1".to_string())
        );
    }

    /// 测试响应数据结构
    #[test]
    fn test_delete_subscribe_response() {
        let response = DeleteSubscribeResponse::default();

        // DeleteSubscribeResponse 是空结构体，测试默认值
        let _ = response;
    }

    /// 测试bitable类型取消订阅
    #[test]
    fn test_delete_subscribe_bitable_type() {
        let config = Config::default();
        let request = DeleteSubscribeRequest::new(config, "bitable_token", "bitable");

        assert_eq!(request.file_type, "bitable");
    }

    /// 测试空event_type（非folder类型）
    #[test]
    fn test_delete_subscribe_without_event_type() {
        let config = Config::default();
        let request = DeleteSubscribeRequest::new(config, "docx_token", "docx");

        assert!(request.event_type.is_none());
    }
}

