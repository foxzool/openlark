//! 修改用户人脸识别信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_setting/modify>
//!
//! 官方请求体为嵌套结构 `{"user_setting": {...}}`，必填 query 参数 `employee_type`。

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 修改用户人脸识别信息请求
#[derive(Debug, Clone)]
pub struct ModifyRequest {
    /// 员工 ID 类型（必填，query 参数）
    employee_type: String,
    /// 用户设置（必填）
    user_setting: UserSetting,
    /// 配置信息
    config: Config,
}

impl ModifyRequest {
    /// 创建请求
    pub fn new(config: Config, employee_type: String, user_setting: UserSetting) -> Self {
        Self {
            employee_type,
            user_setting,
            config,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ModifyResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ModifyResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.employee_type, "employee_type 不能为空");
        validate_required!(self.user_setting.user_id, "user_id 不能为空");
        validate_required!(self.user_setting.face_key, "face_key 不能为空");

        // 2. 构建端点并附加必填 query 参数 employee_type
        let api_endpoint = AttendanceApiV1::UserSettingModify;
        let request = ApiRequest::<ModifyResponse>::post(api_endpoint.to_url())
            .query("employee_type", &self.employee_type);

        // 3. 构建请求体
        let request_body = ModifyRequestBody {
            user_setting: self.user_setting,
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
            "修改用户人脸识别信息响应数据为空",
        )
        .await
    }
}

/// 用户设置（请求体与响应体的 `user_setting` 同构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserSetting {
    /// 用户 ID，对应 employee_type
    pub user_id: String,
    /// 人脸照片文件 ID，获取方式：上传用户人脸识别照片
    pub face_key: String,
    /// 人脸照片更新时间，精确到秒的时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_key_update_time: Option<String>,
}

/// 修改用户人脸识别信息请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyRequestBody {
    /// 用户设置
    pub user_setting: UserSetting,
}

/// 修改用户人脸识别信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModifyResponse {
    /// 用户设置
    pub user_setting: UserSetting,
}

impl ApiResponseTrait for ModifyResponse {
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
    use serde_json::json;

    #[test]
    fn test_modify_request_body_nested_shape() {
        let body = ModifyRequestBody {
            user_setting: UserSetting {
                user_id: "abd754f7".to_string(),
                face_key: "face_key_1".to_string(),
                face_key_update_time: Some("1625681917".to_string()),
            },
        };
        assert_eq!(
            serde_json::to_value(&body).unwrap(),
            json!({
                "user_setting": {
                    "user_id": "abd754f7",
                    "face_key": "face_key_1",
                    "face_key_update_time": "1625681917"
                }
            })
        );

        // 可选字段缺省时不序列化
        let body_without_optional = ModifyRequestBody {
            user_setting: UserSetting {
                user_id: "abd754f7".to_string(),
                face_key: "face_key_1".to_string(),
                face_key_update_time: None,
            },
        };
        assert_eq!(
            serde_json::to_value(&body_without_optional).unwrap(),
            json!({
                "user_setting": {
                    "user_id": "abd754f7",
                    "face_key": "face_key_1"
                }
            })
        );
    }

    #[test]
    fn test_modify_request_builder_new() {
        let request = ModifyRequest::new(
            TestConfigBuilder::new().build(),
            "employee_id".to_string(),
            UserSetting {
                user_id: "abd754f7".to_string(),
                face_key: "face_key_1".to_string(),
                face_key_update_time: None,
            },
        );
        let _ = request;
    }

    #[tokio::test]
    async fn test_modify_validation_rejects_missing_required_fields() {
        let server = wiremock::MockServer::start().await;
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let valid_setting = UserSetting {
            user_id: "abd754f7".to_string(),
            face_key: "face_key_1".to_string(),
            face_key_update_time: None,
        };

        // 空 employee_type 被拒绝
        let empty_employee_type =
            ModifyRequest::new(config.clone(), "  ".to_string(), valid_setting.clone());
        assert!(empty_employee_type.execute().await.is_err());

        // 空 user_id 被拒绝
        let empty_user_id = ModifyRequest::new(
            config.clone(),
            "employee_id".to_string(),
            UserSetting {
                user_id: " ".to_string(),
                ..valid_setting.clone()
            },
        );
        assert!(empty_user_id.execute().await.is_err());

        // 空 face_key 被拒绝
        let empty_face_key = ModifyRequest::new(
            config,
            "employee_id".to_string(),
            UserSetting {
                face_key: " ".to_string(),
                ..valid_setting
            },
        );
        assert!(empty_face_key.execute().await.is_err());

        // 校验失败不应发出任何请求
        let received = server.received_requests().await.unwrap_or_default();
        assert!(received.is_empty());
    }
    /// 端到端：Builder→execute→Transport→mock→assert 嵌套请求体 + query 参数 + 响应解析。
    #[tokio::test]
    async fn test_attendance_v1_user_setting_modify_returns_data_on_success() {
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/attendance/v1/user_settings/modify"))
            .and(query_param("employee_type", "employee_id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "user_setting": {
                        "user_id": "abd754f7",
                        "face_key": "face_key_1",
                        "face_key_update_time": "1625681917"
                    }
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = ModifyRequest::new(
            config,
            "employee_id".to_string(),
            UserSetting {
                user_id: "abd754f7".to_string(),
                face_key: "face_key_1".to_string(),
                face_key_update_time: Some("1625681917".to_string()),
            },
        )
        .execute()
        .await
        .expect("attendance_v1_user_setting_modify 应成功");

        assert_eq!(data.user_setting.user_id, "abd754f7");
        assert_eq!(data.user_setting.face_key, "face_key_1");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/user_settings/modify"
        );
        assert!(
            received[0]
                .url
                .query()
                .unwrap_or("")
                .contains("employee_type=employee_id"),
            "缺必填 query 参数 employee_type: {:?}",
            received[0].url.query()
        );
        let body = String::from_utf8(received[0].body.clone()).unwrap();
        assert!(
            body.contains("\"user_setting\""),
            "请求体缺 user_setting 嵌套包装: {body}"
        );
        assert!(body.contains("\"face_key\""), "请求体缺 face_key: {body}");
        assert!(
            !body.contains("\"face_recognition_enabled\""),
            "请求体不应含臆造字段 face_recognition_enabled: {body}"
        );
    }
}
