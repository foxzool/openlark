//! 创建文件快捷方式
//!
//! 创建文件快捷方式，用于访问云空间的文件。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/create_shortcut

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 创建文件快捷方式响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileShortcutResponse {
    /// 快捷方式
    pub succ_shortcut_node: ShortcutNode,
}

/// 快捷方式文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutNode {
    /// 文件的 token
    pub token: String,
    /// 文件名
    pub name: String,
    /// 文件类型，可选值参照请求体的 refer_type
    #[serde(rename = "type")]
    pub r#type: String,
    /// 父文件夹的 token
    pub parent_token: String,
    /// 访问链接
    pub url: String,
    /// 快捷方式的源文件信息
    pub shortcut_info: CreateShortcutInfo,
    /// 文件创建时间
    pub created_time: String,
    /// 文件最近修改时间
    pub modified_time: String,
    /// 文件所有者
    pub owner_id: String,
}

/// 快捷方式的源文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateShortcutInfo {
    /// 快捷方式对应的源文件类型，可选值参照请求体的 refer_type
    pub target_type: String,
    /// 快捷方式指向的源文件 token
    pub target_token: String,
}

impl ApiResponseTrait for CreateFileShortcutResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建文件快捷方式请求
#[derive(Debug, Clone, Serialize)]
pub struct CreateFileShortcutRequest {
    #[serde(skip)]
    config: Config,
    /// 用户 ID 类型（默认 open_id）
    #[serde(skip)]
    pub user_id_type: Option<String>,
    /// 目标父文件夹的 token
    pub parent_token: String,
    /// 源文件的信息
    pub refer_entity: ReferEntity,
}

/// 源文件的信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferEntity {
    /// 源文件的 token
    pub refer_token: String,
    /// 源文件的类型
    pub refer_type: String,
}

impl CreateFileShortcutRequest {
    /// 创建新的请求实例
    pub fn new(
        config: Config,
        parent_token: impl Into<String>,
        refer_token: impl Into<String>,
        refer_type: impl Into<String>,
    ) -> Self {
        Self {
            config,
            user_id_type: None,
            parent_token: parent_token.into(),
            refer_entity: ReferEntity {
                refer_token: refer_token.into(),
                refer_type: refer_type.into(),
            },
        }
    }

    /// 设置 `user_id_type`。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行创建文件快捷方式操作
    pub async fn execute(self) -> SDKResult<CreateFileShortcutResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateFileShortcutResponse> {
        validate_required!(self.parent_token, "parent_token 不能为空");
        if self.refer_entity.refer_token.is_empty() {
            return Err(openlark_core::error::validation_error(
                "refer_entity.refer_token",
                "refer_token 不能为空",
            ));
        }
        if self.refer_entity.refer_type.is_empty() {
            return Err(openlark_core::error::validation_error(
                "refer_entity.refer_type",
                "refer_type 不能为空",
            ));
        }
        match self.refer_entity.refer_type.as_str() {
            "file" | "docx" | "bitable" | "doc" | "sheet" | "mindnote" | "slides" => {}
            _ => {
                return Err(openlark_core::error::validation_error(
                    "refer_entity.refer_type",
                    "refer_type 仅支持 file/docx/bitable/doc/sheet/mindnote/slides",
                ));
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

        let api_endpoint = DriveApi::CreateShortcut;
        let mut request = ApiRequest::<CreateFileShortcutResponse>::post(&api_endpoint.to_url());

        if let Some(user_id_type) = &self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        request = request.body(serialize_params(&self, "创建文件快捷方式")?);

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建文件快捷方式")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_file_shortcut_request() {
        let config = Config::default();
        let request = CreateFileShortcutRequest::new(config, "fld_xxx", "dox_xxx", "docx")
            .user_id_type("open_id");

        assert_eq!(request.parent_token, "fld_xxx");
        assert_eq!(request.refer_entity.refer_token, "dox_xxx");
        assert_eq!(request.refer_entity.refer_type, "docx");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_response_trait_implementation() {
        assert_eq!(
            CreateFileShortcutResponse::data_format(),
            ResponseFormat::Data
        );
    }
}

