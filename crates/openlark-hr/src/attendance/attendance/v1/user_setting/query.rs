//! 批量查询用户人脸识别信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_setting/query>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 批量查询用户人脸识别信息请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 用户 ID 列表（必填）
    user_ids: Vec<String>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config, user_ids: Vec<String>) -> Self {
        Self { user_ids, config }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.user_ids, "user_ids");

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::UserSettingQuery;
        let request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url());

        // 3. 构建请求体
        let request_body = QueryRequestBody {
            user_ids: self.user_ids,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "构建请求体失败",
                format!("序列化请求体失败: {}", e),
            )
        })?;
        let request = request.body(request_body_json);

        // 4. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 5. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "批量查询用户人脸识别信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 批量查询用户人脸识别信息请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 用户 ID 列表
    pub user_ids: Vec<String>,
}

/// 批量查询用户人脸识别信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 用户人脸识别信息列表
    pub items: Vec<UserSetting>,
}

/// 用户人脸识别信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserSetting {
    /// 用户 ID
    pub user_id: String,
    /// 是否开启人脸识别
    pub face_recognition_enabled: bool,
    /// 人脸照片 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_id: Option<String>,
    /// 照片上传时间（Unix 时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_upload_time: Option<i64>,
}

impl ApiResponseTrait for QueryResponse {
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
    fn test_query_request_builder_new() {
        let request = QueryRequest::new(TestConfigBuilder::new().build(), vec!["test".to_string()]);
        let _ = request;
    }
}
