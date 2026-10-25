//! 上传用户人脸图片
//!
//! docPath: <https://open.feishu.cn/document/server-docs/acs-v1/user/face/update>
//!
//! 文档核对：请求体为 `multipart/form-data`，含三个部件——`files`（人脸图片
//! 二进制，部件名来自官方请求体示例的 Content-Disposition）、`file_type`
//! （jpg/png）、`file_name`（带后缀的文件名）；查询参数 `user_id_type` 可选。

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType,
    error::validation_error, http::Transport, req_option::RequestOption, validate_required,
};

/// 上传用户人脸图片请求
///
/// 图片二进制内容经 [`UpdateUserFaceRequest::execute`] 传入，以 multipart
/// 部件 `files` 上传。
#[derive(Debug)]
pub struct UpdateUserFaceRequest {
    /// 配置信息。
    config: Config,
    /// 用户 ID（路径参数，必填）。
    user_id: String,
    /// 文件类型（必填，可选值 jpg/png）。
    file_type: Option<String>,
    /// 带后缀的文件名（必填，如 `face.jpg`）。
    file_name: Option<String>,
    /// 用户 ID 类型（查询参数，可选，open_id/union_id/user_id，默认 open_id）。
    user_id_type: Option<String>,
}

impl UpdateUserFaceRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config, user_id: impl Into<String>) -> Self {
        Self {
            config,
            user_id: user_id.into(),
            file_type: None,
            file_name: None,
            user_id_type: None,
        }
    }

    /// 设置文件类型（jpg/png，必填）。
    pub fn file_type(mut self, file_type: impl Into<String>) -> Self {
        self.file_type = Some(file_type.into());
        self
    }

    /// 设置带后缀的文件名（必填，如 `face.jpg`）。
    pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name = Some(file_name.into());
        self
    }

    /// 设置用户 ID 类型（open_id/union_id/user_id，查询参数）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求，传入人脸图片二进制内容，返回响应 `data` 字段内容。
    pub async fn execute(self, file_bytes: Vec<u8>) -> SDKResult<serde_json::Value> {
        self.execute_with_options(file_bytes, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        file_bytes: Vec<u8>,
        option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.user_id, "user_id 不能为空");
        let file_type = self
            .file_type
            .ok_or_else(|| validation_error("file_type", "file_type 不能为空"))?;
        let file_name = self
            .file_name
            .ok_or_else(|| validation_error("file_name", "file_name 不能为空"))?;
        if file_bytes.is_empty() {
            return Err(validation_error("files 不能为空", "不允许上传空文件"));
        }

        let path = format!("/open-apis/acs/v1/users/{}/face", self.user_id);
        // multipart 表单字段：file_type / file_name（file_name 同时作为文件部件的
        // 文件名）。body 中以 null 声明 `files` 仅为对齐官方 body 参数表——文件
        // 内容经 `.file_content(...)` 传入、部件名经 `.file_part_name("files")`
        // 指定（来自官方请求体示例，非默认 "file"），multipart 构建器会跳过
        // null 值，不会作为文本字段发送。
        let req: ApiRequest<serde_json::Value> = ApiRequest::put(&path)
            .query_opt("user_id_type", self.user_id_type.as_ref())
            .body(serde_json::json!({
                "files": serde_json::Value::Null,
                "file_type": file_type,
                "file_name": file_name,
            }))
            .file_content(file_bytes)
            .file_part_name("files")
            .with_supported_access_token_types(vec![AccessTokenType::Tenant]);

        Transport::request_typed(req, &self.config, Some(option), "上传用户人脸图片").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> Config {
        Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build()
    }

    #[tokio::test]
    async fn test_update_user_face_rejects_empty_id() {
        let req = UpdateUserFaceRequest::new(test_config(), "");
        let result = req.execute(vec![1, 2, 3]).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("user_id"));
    }

    #[tokio::test]
    async fn test_update_user_face_rejects_missing_file_type() {
        let req = UpdateUserFaceRequest::new(test_config(), "u_001").file_name("face.jpg");
        let result = req.execute(vec![1, 2, 3]).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("file_type"));
    }

    #[tokio::test]
    async fn test_update_user_face_rejects_empty_file() {
        let req = UpdateUserFaceRequest::new(test_config(), "u_001")
            .file_type("jpg")
            .file_name("face.jpg");
        let result = req.execute(Vec::new()).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("files"));
    }

    /// 端到端：PUT .../users/{id}/face（multipart：files/file_type/file_name 三部件）。
    #[tokio::test]
    async fn test_update_user_face_sends_multipart_on_success() {
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 注意：mock 响应体不用 json! 宏构造（字段核对工具会扫描文件内
        // json! 字面量的顶层键，会把响应字段误当 multipart 表单字段）。
        let mock_body = r#"{"code":0,"msg":"success","data":{}}"#;
        Mock::given(method("PUT"))
            .and(path("/open-apis/acs/v1/users/u_001/face"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_raw(mock_body.as_bytes().to_vec(), "application/json"),
            )
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        UpdateUserFaceRequest::new(config, "u_001")
            .file_type("jpg")
            .file_name("face.jpg")
            .user_id_type("open_id")
            .execute(b"JPEG-BYTES".to_vec())
            .await
            .expect("上传用户人脸图片应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/acs/v1/users/u_001/face");
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("user_id_type=open_id"));

        let body = String::from_utf8_lossy(&received[0].body);
        // 文件部件名必须是官方要求的 files，而不是通用默认值 file
        assert!(
            body.contains("name=\"files\""),
            "multipart 文件部件名应为 files，实际 body: {body}"
        );
        assert!(!body.contains("name=\"file\""), "不应使用默认部件名 file");
        // file_type / file_name 作为普通表单字段发送
        assert!(body.contains("name=\"file_type\""));
        assert!(body.contains("name=\"file_name\""));
        assert!(body.contains("jpg"));
        assert!(body.contains("face.jpg"));
    }
}
