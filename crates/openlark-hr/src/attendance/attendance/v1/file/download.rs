//! 下载用户人脸识别照片
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/file/download>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 下载用户人脸识别照片请求
#[derive(Debug, Clone)]
pub struct DownloadRequest {
    /// 照片 ID（必填）
    photo_id: String,
    /// 配置信息
    config: Config,
}

impl DownloadRequest {
    /// 创建请求
    pub fn new(config: Config, photo_id: String) -> Self {
        Self { photo_id, config }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DownloadResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DownloadResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.photo_id.trim(), "photo_id");

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::FileDownload;
        let mut request = ApiRequest::<DownloadResponse>::get(api_endpoint.to_url());

        // 3. 添加查询参数
        request = request.query("photo_id", &self.photo_id);

        // 4. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 5. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "下载用户人脸识别照片响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 下载用户人脸识别照片响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DownloadResponse {
    /// 照片 ID
    pub photo_id: String,
    /// 用户 ID
    pub user_id: String,
    /// 照片数据（Base64 编码）
    pub photo_data: String,
    /// 照片格式
    pub content_type: String,
}

impl ApiResponseTrait for DownloadResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::TestConfigBuilder;

    #[test]
    fn test_download_request_builder_new() {
        let request = DownloadRequest::new(TestConfigBuilder::new().build(), "test".to_string());
        let _ = request;
    }
}
