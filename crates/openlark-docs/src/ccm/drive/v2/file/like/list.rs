use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};

/// 获取云文档的点赞者列表
///
/// 获取指定云文档的点赞者列表。
/// docPath: /document/ukTMukTMukTM/uIzNzUjLyczM14iM3MTN/drive-v2/file-like/list
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 获取点赞者列表请求
#[derive(Debug, Clone)]
pub struct ListFileLikesRequest {
    config: Config,
    /// 云文档 token
    pub file_token: String,
    /// 云文档类型（doc/docx/file）
    pub file_type: String,
    /// 分页大小（1~50，默认 10）
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 用户 ID 类型
    pub user_id_type: Option<String>,
}

impl ListFileLikesRequest {
    /// 创建新的点赞者列表请求。
    pub fn new(
        config: Config,
        file_token: impl Into<String>,
        file_type: impl Into<String>,
    ) -> Self {
        Self {
            config,
            file_token: file_token.into(),
            file_type: file_type.into(),
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }

    /// 设置分页大小。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<ListFileLikesResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListFileLikesResponse> {
        validate_required!(self.file_token, "file_token 不能为空");
        match self.file_type.as_str() {
            "doc" | "docx" | "file" => {}
            _ => {
                return Err(openlark_core::error::validation_error(
                    "file_type",
                    "file_type 仅支持 doc/docx/file",
                ));
            }
        }
        if let Some(page_size) = self.page_size
            && !(1..=50).contains(&page_size)
        {
            return Err(openlark_core::error::validation_error(
                "page_size",
                "page_size 必须在 1~50 之间",
            ));
        }

        let api_endpoint = DriveApi::ListFileLikes(self.file_token);
        let request = ApiRequest::<ListFileLikesResponse>::get(&api_endpoint.to_url())
            .query("file_type", self.file_type)
            .query_opt("page_size", self.page_size.map(|v| v.to_string()))
            .query_opt("page_token", self.page_token)
            .query_opt("user_id_type", self.user_id_type);

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "列表")
    }
}

/// 点赞者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileLike {
    /// 用户 ID（与 user_id_type 一致）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户最后点赞时间（秒级时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_liked_time: Option<String>,
    /// 用户名字（用户信息被脱敏时不返回）
    pub user_name: Option<String>,
    /// 用户英文名字（用户信息被脱敏时不返回）
    pub user_en_name: Option<String>,
    /// 用户头像（用户信息被脱敏时不返回）
    pub user_avatar_url: Option<String>,
    /// 用户信息是否脱敏
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_is_desensitized: Option<bool>,
}

/// 获取点赞者列表响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFileLikesResponse {
    /// 云文档的点赞者列表
    #[serde(default)]
    pub items: Vec<FileLike>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项
    pub has_more: bool,
}

impl ApiResponseTrait for ListFileLikesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_file_likes_request_builder() {
        let config = Config::default();
        let request = ListFileLikesRequest::new(config, "file_token", "docx")
            .page_size(20)
            .page_token("next_page_token")
            .user_id_type("open_id");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.file_type, "docx");
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token, Some("next_page_token".to_string()));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_like_info_structure() {
        let like = FileLike {
            user_id: Some("ou_xxx".to_string()),
            last_liked_time: Some("1690857821".to_string()),
            user_name: Some("张三".to_string()),
            user_en_name: Some("San Zhang".to_string()),
            user_avatar_url: Some("https://foo.icon.com/xxxx".to_string()),
            user_is_desensitized: Some(false),
        };

        assert_eq!(like.user_id.as_deref(), Some("ou_xxx"));
        assert_eq!(like.last_liked_time.as_deref(), Some("1690857821"));
        assert_eq!(like.user_name.as_deref(), Some("张三"));
        assert_eq!(like.user_is_desensitized, Some(false));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(ListFileLikesResponse::data_format(), ResponseFormat::Data);
    }
}
