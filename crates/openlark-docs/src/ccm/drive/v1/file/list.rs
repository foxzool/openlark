//! 获取文件夹中的文件清单
//!
//! 获取用户云空间中指定文件夹下的文件清单。清单类型包括文件、各种在线文档（文档、电子表格、多维表格、思维笔记）、文件夹和快捷方式。
//! 该接口支持分页，但是不会递归的获取子文件夹的清单。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 获取文件夹中的文件清单请求
#[derive(Debug)]
pub struct ListFilesRequest {
    config: Config,
    /// 文件夹 token，不填则获取根目录
    pub folder_token: Option<String>,
    /// 分页大小（1~200）。默认值为 100
    pub page_size: Option<i32>,
    /// 分页标记，第一页不填；后续页面使用上一页返回的 `next_page_token` 作为本次的 `page_token`
    pub page_token: Option<String>,
    /// 排序方式（可选）：EditedTime | CreatedTime
    pub order_by: Option<String>,
    /// 排序规则（可选）：ASC | DESC
    pub direction: Option<String>,
    /// 用户 ID 类型（可选）：open_id | union_id | user_id
    pub user_id_type: Option<String>,
}

/// 文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// 文件 token
    pub token: String,
    /// 文件名称
    pub name: String,
    /// 文件类型
    pub r#type: String,
    /// 父节点 token（文件夹 token）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_token: Option<String>,
    /// 访问链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 快捷方式信息（当 type=shortcut 时返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortcut_info: Option<ShortcutInfo>,
    /// 文件创建时间（秒级时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// 文件最近修改时间（秒级时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<String>,
    /// 文件所有者的 ID。ID 类型由查询参数中的 user_id_type 决定
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
}

/// 快捷方式信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutInfo {
    /// 快捷方式指向的原文件类型
    pub target_type: String,
    /// 快捷方式指向的原文件 token
    pub target_token: String,
}

/// 获取文件夹中的文件清单响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFilesResponse {
    /// 文件夹中的文件清单列表
    pub files: Vec<FileInfo>,
    /// 下一页分页标记。当 has_more 为 true 时会返回
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// 是否还有更多项
    pub has_more: bool,
}

impl ApiResponseTrait for ListFilesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListFilesRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            folder_token: None,
            page_size: None,
            page_token: None,
            order_by: None,
            direction: None,
            user_id_type: None,
        }
    }

    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.folder_token = Some(folder_token.into());
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    pub fn order_by(mut self, order_by: impl Into<String>) -> Self {
        self.order_by = Some(order_by.into());
        self
    }

    pub fn direction(mut self, direction: impl Into<String>) -> Self {
        self.direction = Some(direction.into());
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<ListFilesResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListFilesResponse> {
        // ========== 参数校验 ==========

        // 不填写或填空字符串表示根目录；根目录不支持分页
        let is_root = self
            .folder_token
            .as_deref()
            .map(|t| t.trim().is_empty())
            .unwrap_or(true);

        if is_root {
            if self.page_size.is_some() {
                return Err(openlark_core::error::validation_error(
                    "page_size",
                    "获取根目录清单时不支持分页，请勿传入 page_size",
                ));
            }
            if self.page_token.is_some() {
                return Err(openlark_core::error::validation_error(
                    "page_token",
                    "获取根目录清单时不支持分页，请勿传入 page_token",
                ));
            }
        }

        if let Some(page_size) = self.page_size {
            if !(1..=200).contains(&page_size) {
                return Err(openlark_core::error::validation_error(
                    "page_size",
                    "page_size 必须在 1~200 之间",
                ));
            }
        }

        if let Some(order_by) = &self.order_by {
            match order_by.as_str() {
                "EditedTime" | "CreatedTime" => {}
                _ => {
                    return Err(openlark_core::error::validation_error(
                        "order_by",
                        "order_by 仅支持 EditedTime/CreatedTime",
                    ));
                }
            }
        }

        if let Some(direction) = &self.direction {
            match direction.as_str() {
                "ASC" | "DESC" => {}
                _ => {
                    return Err(openlark_core::error::validation_error(
                        "direction",
                        "direction 仅支持 ASC/DESC",
                    ));
                }
            }
        }

        if let Some(user_id_type) = &self.user_id_type {
            match user_id_type.as_str() {
                "open_id" | "union_id" | "user_id" => {}
                _ => {
                    return Err(openlark_core::error::validation_error(
                        "user_id_type",
                        "user_id_type 仅支持 open_id/union_id/user_id",
                    ));
                }
            }
        }

        // ========== 构建 API 请求 ==========

        let api_endpoint = DriveApi::ListFiles;
        let mut request = ApiRequest::<ListFilesResponse>::get(&api_endpoint.to_url());

        if let Some(token) = &self.folder_token {
            request = request.query("folder_token", token);
        }
        if let Some(size) = self.page_size {
            request = request.query("page_size", size.to_string());
        }
        if let Some(token) = &self.page_token {
            request = request.query("page_token", token);
        }
        if let Some(order) = &self.order_by {
            request = request.query("order_by", order);
        }
        if let Some(direction) = &self.direction {
            request = request.query("direction", direction);
        }
        if let Some(user_id_type) = &self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        // ========== 发送请求并返回响应 ==========
        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取文件夹中的文件清单")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_files_request_builder() {
        let config = Config::default();
        let request = ListFilesRequest::new(config)
            .folder_token("folder_token")
            .page_size(50)
            .order_by("EditedTime")
            .direction("DESC")
            .user_id_type("open_id");

        assert_eq!(request.folder_token, Some("folder_token".to_string()));
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.order_by, Some("EditedTime".to_string()));
        assert_eq!(request.direction, Some("DESC".to_string()));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_list_files_request_root() {
        let config = Config::default();
        let request = ListFilesRequest::new(config);

        assert_eq!(request.folder_token, None);
        assert_eq!(request.page_token, None);
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(ListFilesResponse::data_format(), ResponseFormat::Data);
    }
}
