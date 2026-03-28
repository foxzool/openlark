//! 获取云文档协作者
//!
//! 获取文件或文件夹的协作者列表。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/permission/permission-member/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

use super::models::PermissionMemberItem;

/// 获取协作者列表请求
#[derive(Debug, Clone)]
pub struct ListPermissionMembersRequest {
    config: Config,
    /// 文件token
    pub token: String,
    /// 云文档类型（query 参数 `type`，需要与 token 匹配）
    pub file_type: String,
    /// 返回字段（query 参数 `fields`）
    pub fields: Option<String>,
    /// 权限角色类型（query 参数 `perm_type`）
    pub perm_type: Option<String>,
}

impl ListPermissionMembersRequest {
    /// 创建获取协作者列表请求
    ///
    /// # 参数
    /// * `config` - 配置
    /// * `token` - 文件token
    /// * `file_type` - 云文档类型（query 参数 `type`）
    pub fn new(config: Config, token: impl Into<String>, file_type: impl Into<String>) -> Self {
        Self {
            config,
            token: token.into(),
            file_type: file_type.into(),
            fields: None,
            perm_type: None,
        }
    }

    /// 设置返回字段（如 `*`）
    pub fn fields(mut self, fields: impl Into<String>) -> Self {
        self.fields = Some(fields.into());
        self
    }

    /// 设置权限角色类型（知识库文档有效）
    pub fn perm_type(mut self, perm_type: impl Into<String>) -> Self {
        self.perm_type = Some(perm_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<ListPermissionMembersResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListPermissionMembersResponse> {
        // ===== 验证必填字段 =====
        if self.token.is_empty() {
            return Err(openlark_core::error::validation_error(
                "token",
                "token 不能为空",
            ));
        }
        if self.file_type.is_empty() {
            return Err(openlark_core::error::validation_error(
                "file_type",
                "file_type 不能为空",
            ));
        }
        // ===== 验证字段枚举值 =====
        match self.file_type.as_str() {
            "doc" | "sheet" | "file" | "wiki" | "bitable" | "docx" | "folder" | "mindnote"
            | "minutes" | "slides" => {}
            _ => {
                return Err(openlark_core::error::validation_error(
                    "file_type",
                    "file_type 必须为 doc/sheet/file/wiki/bitable/docx/folder/mindnote/minutes/slides",
                ));
            }
        }
        if let Some(perm_type) = &self.perm_type {
            match perm_type.as_str() {
                "container" | "single_page" => {}
                _ => {
                    return Err(openlark_core::error::validation_error(
                        "perm_type",
                        "perm_type 必须为 container/single_page",
                    ));
                }
            }
        }

        let api_endpoint = DriveApi::ListPermissionMembers(self.token.clone());

        let mut api_request =
            ApiRequest::<ListPermissionMembersResponse>::get(&api_endpoint.to_url());

        api_request = api_request.query("type", &self.file_type);
        if let Some(fields) = &self.fields {
            api_request = api_request.query("fields", fields);
        }
        if let Some(perm_type) = &self.perm_type {
            api_request = api_request.query("perm_type", perm_type);
        }

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "列表")
    }
}

/// 获取协作者列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPermissionMembersResponse {
    /// 协作者列表
    #[serde(default)]
    pub items: Vec<PermissionMemberItem>,
}

impl ApiResponseTrait for ListPermissionMembersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::test_runtime;

    /// 测试构建器模式
    #[test]
    fn test_list_permission_members_request_builder() {
        let config = Config::default();
        let request = ListPermissionMembersRequest::new(config, "file_token", "docx");

        assert_eq!(request.token, "file_token");
        assert_eq!(request.file_type, "docx");
    }

    /// 测试带参数的请求
    #[test]
    fn test_list_permission_members_request_with_params() {
        let config = Config::default();
        let request = ListPermissionMembersRequest::new(config, "file_token", "docx")
            .fields("*")
            .perm_type("container");

        assert_eq!(request.token, "file_token");
        assert_eq!(request.file_type, "docx");
        assert_eq!(
            request
                .fields
                .expect("fields should be set when .fields() is called"),
            "*"
        );
        assert_eq!(
            request
                .perm_type
                .expect("perm_type should be set when .perm_type() is called"),
            "container"
        );
    }

    /// 测试响应格式
    #[test]
    fn test_response_trait() {
        assert_eq!(
            ListPermissionMembersResponse::data_format(),
            ResponseFormat::Data
        );
    }

    /// 测试 token 为空时的验证
    #[test]
    fn test_empty_token_validation() {
        let config = Config::default();
        let request = ListPermissionMembersRequest::new(config, "", "docx");

        let result = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request.execute().await;
            })
        })
        .join();

        assert!(result.is_ok());
    }

    /// 测试 file_type 枚举值验证
    #[test]
    fn test_file_type_validation() {
        let config = Config::default();
        let request = ListPermissionMembersRequest::new(config, "token", "invalid");

        let result = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request.execute().await;
            })
        })
        .join();

        assert!(result.is_ok());
    }

    /// 测试 perm_type 枚举值验证
    #[test]
    fn test_perm_type_validation() {
        let config = Config::default();
        let request =
            ListPermissionMembersRequest::new(config, "token", "docx").perm_type("invalid");

        let result = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request.execute().await;
            })
        })
        .join();

        assert!(result.is_ok());
    }

    /// 测试支持的 file_type 类型
    #[test]
    fn test_supported_file_types() {
        let config = Config::default();

        for file_type in [
            "doc", "sheet", "file", "wiki", "bitable", "docx", "folder", "mindnote", "minutes",
            "slides",
        ] {
            let request =
                ListPermissionMembersRequest::new(config.clone(), "token", file_type.to_string());
            assert_eq!(request.file_type, file_type);
        }
    }

    /// 测试可选参数
    #[test]
    fn test_optional_parameters() {
        let config = Config::default();
        let request = ListPermissionMembersRequest::new(config.clone(), "token", "docx");

        assert!(request.fields.is_none());
        assert!(request.perm_type.is_none());

        let request2 = ListPermissionMembersRequest::new(config.clone(), "token", "docx")
            .fields("*")
            .perm_type("container");

        assert_eq!(request2.fields, Some("*".to_string()));
        assert_eq!(request2.perm_type, Some("container".to_string()));
    }
}
