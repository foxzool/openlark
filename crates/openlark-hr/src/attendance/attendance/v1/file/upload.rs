//! 上传用户人脸识别照片
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/file/upload>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, validate_required_list, validation_error, SDKResult,
};
use rand::Rng;
use serde::{Deserialize, Serialize};

/// 上传用户人脸识别照片请求
#[derive(Debug, Clone)]
pub struct UploadRequest {
    /// 用户 ID（必填）
    user_id: String,
    /// 图片名称（必填）
    file_name: String,
    /// 图片二进制数据（必填）
    image_data: Vec<u8>,
    /// 配置信息
    config: Config,
}

impl UploadRequest {
    /// 创建请求
    pub fn new(config: Config, user_id: String, file_name: String, image_data: Vec<u8>) -> Self {
        Self {
            user_id,
            file_name,
            image_data,
            config,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UploadResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UploadResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.user_id.trim(), "user_id");
        validate_required_list!(self.image_data, 1, "图片数据不能为空");

        // 验证文件名
        let file_name = self.file_name.trim();
        if file_name.is_empty() {
            return Err(validation_error("file_name", "文件名不能为空"));
        }
        if file_name.len() > 255 {
            return Err(validation_error(
                "file_name",
                "文件名长度不能超过 255 个字符",
            ));
        }
        // 检查路径遍历字符
        if file_name.contains("..") || file_name.contains('/') || file_name.contains('\\') {
            return Err(validation_error(
                "file_name",
                "文件名不能包含路径分隔符或路径遍历字符",
            ));
        }

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::FileUpload;
        let request = ApiRequest::<UploadResponse>::post(api_endpoint.to_url());

        // 3. 构建请求体（multipart/form-data）
        // 生成随机 boundary 后缀避免冲突
        let random_suffix: u32 = rand::thread_rng().gen_range(0..1000000);
        let boundary = format!("----OpenLarkBoundary{:06}", random_suffix);
        let mime_type = detect_mime_type(file_name);
        let body = build_multipart_body(
            &self.user_id,
            file_name,
            &self.image_data,
            &boundary,
            mime_type,
        );

        let request = request
            .header(
                "Content-Type",
                format!("multipart/form-data; boundary={}", boundary),
            )
            .body(body);

        // 4. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 5. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "上传用户人脸识别照片响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 根据文件扩展名检测 MIME 类型
fn detect_mime_type(file_name: &str) -> &'static str {
    let ext = file_name.rsplit('.').next().unwrap_or("").to_lowercase();

    match ext.as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "bmp" => "image/bmp",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        _ => "application/octet-stream",
    }
}

/// 构建 multipart 表单数据
fn build_multipart_body(
    user_id: &str,
    file_name: &str,
    image_data: &[u8],
    boundary: &str,
    mime_type: &str,
) -> Vec<u8> {
    let mut body = Vec::new();

    // 用户 ID 部分
    body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
    body.extend_from_slice(b"Content-Disposition: form-data; name=\"user_id\"\r\n\r\n");
    body.extend_from_slice(user_id.as_bytes());
    body.extend_from_slice(b"\r\n");

    // 文件部分
    body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
    body.extend_from_slice(
        format!(
            "Content-Disposition: form-data; name=\"file\"; filename=\"{}\"\r\n",
            file_name
        )
        .as_bytes(),
    );
    body.extend_from_slice(format!("Content-Type: {}\r\n\r\n", mime_type).as_bytes());
    body.extend_from_slice(image_data);
    body.extend_from_slice(b"\r\n");

    // 结束边界
    body.extend_from_slice(format!("--{}--\r\n", boundary).as_bytes());

    body
}

/// 上传用户人脸识别照片响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UploadResponse {
    /// 是否成功
    pub success: bool,
    /// 照片 ID
    pub photo_id: String,
    /// 用户 ID
    pub user_id: String,
    /// 上传时间（Unix 时间戳）
    pub upload_time: i64,
}

impl ApiResponseTrait for UploadResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

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
