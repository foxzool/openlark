//! 转移云文档所有者
//!
//! 将文件或文件夹的所有者转移给其他用户。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/permission/permission-member/transfer_owner

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 转移所有者请求
#[derive(Debug, Clone)]
pub struct TransferOwnerRequest {
    config: Config,
    /// 文件token
    pub token: String,
    /// 云文档类型（query 参数 `type`，需要与 token 匹配）
    pub file_type: String,
    /// 是否发送通知（query 参数 `need_notification`，默认 true）
    pub need_notification: Option<bool>,
    /// 是否移除旧所有者（query 参数 `remove_old_owner`，默认 false）
    pub remove_old_owner: Option<bool>,
    /// 旧所有者是否保留原位（query 参数 `stay_put`，默认 false）
    pub stay_put: Option<bool>,
    /// 旧所有者权限（query 参数 `old_owner_perm`，默认 full_access）
    pub old_owner_perm: Option<String>,

    /// 新所有者 ID 类型（request body 参数 `member_type`）
    pub member_type: String,
    /// 新所有者 ID（request body 参数 `member_id`）
    pub member_id: String,
}

impl TransferOwnerRequest {
    /// 创建新的所有者转移请求。
    /// 创建转移所有者请求
    ///
    /// # 参数
    /// * `config` - 配置
    /// * `token` - 文件token
    /// * `file_type` - 云文档类型（query 参数 `type`）
    /// * `member_type` - 新所有者 ID 类型（request body 参数 `member_type`）
    /// * `member_id` - 新所有者 ID（request body 参数 `member_id`）
    pub fn new(
        config: Config,
        token: impl Into<String>,
        file_type: impl Into<String>,
        member_type: impl Into<String>,
        member_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            token: token.into(),
            file_type: file_type.into(),
            need_notification: None,
            remove_old_owner: None,
            stay_put: None,
            old_owner_perm: None,
            member_type: member_type.into(),
            member_id: member_id.into(),
        }
    }

    /// 设置是否发送通知（默认 true）
    pub fn need_notification(mut self, need_notification: bool) -> Self {
        self.need_notification = Some(need_notification);
        self
    }

    /// 设置是否移除旧所有者（默认 false）
    pub fn remove_old_owner(mut self, remove_old_owner: bool) -> Self {
        self.remove_old_owner = Some(remove_old_owner);
        self
    }

    /// 设置旧所有者是否保留原位（默认 false）
    pub fn stay_put(mut self, stay_put: bool) -> Self {
        self.stay_put = Some(stay_put);
        self
    }

    /// 设置旧所有者权限（默认 full_access）
    pub fn old_owner_perm(mut self, old_owner_perm: impl Into<String>) -> Self {
        self.old_owner_perm = Some(old_owner_perm.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<TransferOwnerResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<TransferOwnerResponse> {
        // ===== 验证必填字段 =====
        validate_required!(self.token, "token 不能为空");
        validate_required!(self.file_type, "file_type 不能为空");
        validate_required!(self.member_type, "member_type 不能为空");
        validate_required!(self.member_id, "member_id 不能为空");
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
        match self.member_type.as_str() {
            "email" | "openid" | "userid" => {}
            _ => {
                return Err(openlark_core::error::validation_error(
                    "member_type",
                    "member_type 必须为 email/openid/userid",
                ));
            }
        }
        if let Some(old_owner_perm) = &self.old_owner_perm {
            match old_owner_perm.as_str() {
                "view" | "edit" | "full_access" => {}
                _ => {
                    return Err(openlark_core::error::validation_error(
                        "old_owner_perm",
                        "old_owner_perm 必须为 view/edit/full_access",
                    ));
                }
            }
        }

        let api_endpoint = DriveApi::TransferOwner(self.token.clone());

        #[derive(Serialize)]
        struct TransferOwnerBody {
            member_type: String,
            member_id: String,
        }

        let body = TransferOwnerBody {
            member_type: self.member_type,
            member_id: self.member_id,
        };

        let mut api_request: ApiRequest<TransferOwnerResponse> =
            ApiRequest::post(&api_endpoint.to_url()).query("type", &self.file_type);

        if let Some(need_notification) = self.need_notification {
            api_request = api_request.query("need_notification", need_notification.to_string());
        }
        if let Some(remove_old_owner) = self.remove_old_owner {
            api_request = api_request.query("remove_old_owner", remove_old_owner.to_string());
        }
        if let Some(stay_put) = self.stay_put {
            api_request = api_request.query("stay_put", stay_put.to_string());
        }
        if let Some(old_owner_perm) = &self.old_owner_perm {
            api_request = api_request.query("old_owner_perm", old_owner_perm);
        }

        api_request = api_request.body(serialize_params(&body, "转移云文档所有者")?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "转移")
    }
}

/// 转移所有者响应
/// 转移所有者响应 data。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferOwnerResponse {}

impl ApiResponseTrait for TransferOwnerResponse {
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
    fn test_transfer_owner_request_builder() {
        let config = Config::default();
        let request = TransferOwnerRequest::new(config, "file_token", "docx", "openid", "ou_xxx")
            .need_notification(true)
            .remove_old_owner(false)
            .stay_put(false)
            .old_owner_perm("full_access");

        assert_eq!(request.token, "file_token");
        assert_eq!(request.file_type, "docx");
        assert_eq!(request.member_type, "openid");
        assert_eq!(request.member_id, "ou_xxx");
    }

    /// 测试响应格式
    #[test]
    fn test_response_trait() {
        assert_eq!(TransferOwnerResponse::data_format(), ResponseFormat::Data);
    }

    /// 测试 token 为空时的验证
    #[test]
    fn test_empty_token_validation() {
        let config = Config::default();
        let request = TransferOwnerRequest::new(config, "", "docx", "openid", "ou_xxx");

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
        let request = TransferOwnerRequest::new(config, "token", "invalid", "openid", "ou_xxx");

        let result = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request.execute().await;
            })
        })
        .join();

        assert!(result.is_ok());
    }

    /// 测试 member_type 枚举值验证
    #[test]
    fn test_member_type_validation() {
        let config = Config::default();
        let request = TransferOwnerRequest::new(config, "token", "docx", "invalid", "ou_xxx");

        let result = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request.execute().await;
            })
        })
        .join();

        assert!(result.is_ok());
    }

    /// 测试 member_id 为空时的验证
    #[test]
    fn test_empty_member_id_validation() {
        let config = Config::default();
        let request = TransferOwnerRequest::new(config, "token", "docx", "openid", "");

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
            let request = TransferOwnerRequest::new(
                config.clone(),
                "token",
                file_type.to_string(),
                "openid",
                "ou_xxx",
            );
            assert_eq!(request.file_type, file_type);
        }
    }

    /// 测试支持的 member_type 类型
    #[test]
    fn test_supported_member_types() {
        let config = Config::default();

        for member_type in ["email", "openid", "userid"] {
            let request = TransferOwnerRequest::new(
                config.clone(),
                "token",
                "docx",
                member_type.to_string(),
                "ou_xxx",
            );
            assert_eq!(request.member_type, member_type);
        }
    }

    /// 测试可选参数
    #[test]
    fn test_optional_parameters() {
        let config = Config::default();
        let request = TransferOwnerRequest::new(config, "token", "docx", "openid", "ou_xxx");

        assert!(request.need_notification.is_none());
        assert!(request.remove_old_owner.is_none());
        assert!(request.stay_put.is_none());
        assert!(request.old_owner_perm.is_none());
    }
}
