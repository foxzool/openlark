//! 更新统计设置
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_stats_view/update>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 更新统计设置请求
#[derive(Debug, Clone)]
pub struct UpdateRequest {
    /// 统计设置 ID（path 参数 `user_stats_view_id`，必填）
    user_stats_view_id: String,
    /// 统计设置（必填）
    view: UserStatsView,
    /// 配置信息
    config: Config,
}

impl UpdateRequest {
    /// 创建请求
    ///
    /// - `user_stats_view_id`: path 参数（统计设置 ID）
    /// - `view`: 统计设置内容
    pub fn new(config: Config, user_stats_view_id: String, view: UserStatsView) -> Self {
        Self {
            user_stats_view_id,
            view,
            config,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.user_stats_view_id.trim(), "user_stats_view_id");

        // 2. 构建端点（user_stats_view_id 为 path 参数）
        let api_endpoint =
            AttendanceApiV1::UserStatsViewUpdate(self.user_stats_view_id.clone()).to_url();
        let request = ApiRequest::<UpdateResponse>::put(&api_endpoint);

        // 3. 构建请求体
        let request_body = UpdateRequestBody { view: self.view };
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
            "更新统计设置响应数据为空",
        )
        .await
    }
}

/// 更新统计设置请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRequestBody {
    /// 统计设置
    pub view: UserStatsView,
}

/// 统计设置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserStatsView {
    /// 视图 ID
    pub view_id: String,
    /// 视图类型（`daily` / `month`）
    pub stats_type: String,
    /// 操作者用户 ID
    pub user_id: String,
    /// 用户设置字段
    pub items: Vec<UserStatsViewItem>,
}

/// 用户设置字段项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserStatsViewItem {
    /// 标题编号
    pub code: String,
    /// 子标题
    pub child_items: Vec<ChildItem>,
}

/// 子标题项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChildItem {
    /// 子标题编号
    pub code: String,
    /// 开关字段（`0`=关闭，`1`=开启）
    pub value: String,
}

/// 更新统计设置响应
///
/// 官网 response `data.view` 为 object，schema 未完整给出，透传 `Value`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateResponse {
    /// 统计设置
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view: Option<serde_json::Value>,
}

impl ApiResponseTrait for UpdateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;
    use openlark_core::testing::prelude::TestConfigBuilder;

    #[test]
    fn test_update_request_builder_new() {
        let request = UpdateRequest::new(
            TestConfigBuilder::new().build(),
            "TmpZNU5qTTJORFF6T1RnNU5UTTNOakV6TWl0dGIyNTBhQT09".to_string(),
            UserStatsView {
                view_id: "TmpZNU5qTTJORFF6T1RnNU5UTTNOakV6TWl0dGIyNTBhQT09".to_string(),
                stats_type: "month".to_string(),
                user_id: "ec8ddg56".to_string(),
                items: vec![UserStatsViewItem {
                    code: "522".to_string(),
                    child_items: vec![ChildItem {
                        code: "50101".to_string(),
                        value: "0".to_string(),
                    }],
                }],
            },
        );
        let _ = request;
    }

    /// 端到端：Builder→execute→Transport→mock→assert 请求体字段对齐飞书官网 schema。
    #[tokio::test]
    async fn test_attendance_v1_user_stats_view_update_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/open-apis/attendance/v1/user_stats_views/view_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "view": { "view_id": "view_001" } }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = UpdateRequest::new(
            config,
            "view_001".to_string(),
            UserStatsView {
                view_id: "view_001".to_string(),
                stats_type: "month".to_string(),
                user_id: "ec8ddg56".to_string(),
                items: vec![UserStatsViewItem {
                    code: "522".to_string(),
                    child_items: vec![ChildItem {
                        code: "50101".to_string(),
                        value: "0".to_string(),
                    }],
                }],
            },
        )
        .execute()
        .await
        .expect("attendance_v1_user_stats_view_update 应成功");

        assert!(data.view.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/user_stats_views/view_001"
        );
        let body = String::from_utf8(received[0].body.clone()).unwrap();
        assert!(body.contains("\"view\""), "请求体缺 view: {body}");
        assert!(
            body.contains("\"stats_type\""),
            "请求体缺 view.stats_type: {body}"
        );
        assert!(
            body.contains("\"child_items\""),
            "请求体缺 view.items.child_items: {body}"
        );
        assert!(
            !body.contains("\"field_ids\""),
            "请求体不应含旧字段 field_ids: {body}"
        );
    }
}
