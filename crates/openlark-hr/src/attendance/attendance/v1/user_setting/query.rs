//! 批量查询用户人脸识别信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_setting/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 批量查询用户人脸识别信息请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 员工 ID 类型（必填，query 参数，可选值：employee_id、employee_no）
    employee_type: String,
    /// 用户 ID 列表（必填，不超过 100 个）
    user_ids: Vec<String>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    ///
    /// - `employee_type`: 员工 ID 类型（query 参数）
    /// - `user_ids`: 用户 ID 列表（不超过 100 个）
    pub fn new(config: Config, employee_type: String, user_ids: Vec<String>) -> Self {
        Self {
            employee_type,
            user_ids,
            config,
        }
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
        validate_required!(self.employee_type, "employee_type 不能为空");
        validate_required_list!(self.user_ids, 100, "user_ids 不能为空且不能超过 100 个");

        // 2. 构建端点并附加必填 query 参数 employee_type
        let api_endpoint = AttendanceApiV1::UserSettingQuery;
        let request = ApiRequest::<QueryResponse>::get(api_endpoint.to_url())
            .query("employee_type", &self.employee_type);

        // 3. 构建请求体
        let request_body = QueryRequestBody {
            user_ids: self.user_ids,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "构建请求体失败",
                format!("序列化请求体失败: {e}"),
            )
        })?;
        let request = request.body(request_body_json);

        // 4. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "批量查询用户人脸识别信息响应数据为空",
        )
        .await
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
    /// 用户设置信息列表（user_setting[]）
    #[serde(default)]
    pub user_settings: Vec<UserSetting>,
}

/// 用户人脸识别信息（官方 `user_setting`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserSetting {
    /// 用户 ID，对应 employee_type
    pub user_id: String,
    /// 人脸照片文件 ID，可用于下载用户人脸识别照片
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub face_key: Option<String>,
    /// 人脸照片更新时间，精确到秒的时间戳
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub face_key_update_time: Option<String>,
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
    use openlark_core::config::Config;
    use openlark_core::testing::prelude::TestConfigBuilder;

    #[test]
    fn test_query_request_builder_new() {
        let request = QueryRequest::new(
            TestConfigBuilder::new().build(),
            "employee_id".to_string(),
            vec!["test".to_string()],
        );
        let _ = request;
    }
    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_attendance_v1_user_setting_query_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状：data.user_settings[]（user_id/face_key/face_key_update_time）
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"user_settings": [{"user_id": "abd754f7", "face_key": "xxxxxb306842b1c189bc5212eefxxxxx", "face_key_update_time": "1625681917"}]}"#,
        )
        .unwrap();
        Mock::given(method("GET"))
            .and(path("/open-apis/attendance/v1/user_settings/query"))
            .and(query_param("employee_type", "employee_id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": data_body
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = QueryRequest::new(
            config,
            "employee_id".to_string(),
            vec!["id_001".to_string()],
        )
        .execute()
        .await
        .expect("attendance_v1_user_setting_query 应成功");

        assert_eq!(data.user_settings.len(), 1);
        assert_eq!(data.user_settings[0].user_id, "abd754f7");
        assert_eq!(
            data.user_settings[0].face_key.as_deref(),
            Some("xxxxxb306842b1c189bc5212eefxxxxx")
        );
        assert_eq!(
            data.user_settings[0].face_key_update_time.as_deref(),
            Some("1625681917")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/user_settings/query"
        );
    }
}
