//! 更新人员组成员
//!
//! docPath: <https://open.feishu.cn/document/server-docs/performance-v2/user_group_user_rel/write>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 更新人员组成员请求
///
/// 官方更新模式为覆盖式更新：更新操作会清除人员组以前的成员。
#[derive(Debug, Clone)]
pub struct WriteRequest {
    /// 人员组 ID（必填，可从人员组管理页面获取，0~128 字符）
    group_id: String,
    /// 人员组可见性配置（必填）
    /// - 0: 无限制
    /// - 1: 后台管理员不可见（默认）
    scope_visible_setting: i32,
    /// 人员 ID 列表（必填，全量成员，最多 10000 个，ID 类型与 user_id_type 一致）
    user_ids: Vec<String>,
    /// 幂等标记（必填 query 参数 client_token，同一请求多次调用将被拦截，0~64 字符）
    client_token: String,
    /// 用户 ID 类型（可选 query 参数，默认 open_id）
    user_id_type: Option<String>,
    /// 配置信息
    config: Config,
}

impl WriteRequest {
    /// 创建请求
    pub fn new(
        config: Config,
        group_id: String,
        scope_visible_setting: i32,
        client_token: String,
    ) -> Self {
        Self {
            group_id,
            scope_visible_setting,
            user_ids: Vec::new(),
            client_token,
            user_id_type: None,
            config,
        }
    }

    /// 追加一个人员 ID（覆盖式全量更新，须传入人员组的全部成员）
    pub fn add_user(mut self, user_id: String) -> Self {
        self.user_ids.push(user_id);
        self
    }

    /// 设置用户 ID 类型（可选 query 参数，open_id/union_id/user_id）
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<WriteResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<WriteResponse> {
        use crate::common::api_endpoints::PerformanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.group_id.trim(), "group_id");
        if !(0..=1).contains(&self.scope_visible_setting) {
            return Err(openlark_core::error::validation_error(
                "scope_visible_setting 无效",
                "scope_visible_setting 必须为 0（无限制）或 1（后台管理员不可见）",
            ));
        }
        validate_required_list!(self.user_ids, 10000, "user_ids 不能为空且不能超过 10000 个");
        validate_required!(self.client_token.trim(), "client_token");

        // 2. 构建端点
        let api_endpoint = PerformanceApiV1::UserGroupUserRelWrite;
        let request = ApiRequest::<WriteResponse>::post(api_endpoint.to_url())
            .query("client_token", self.client_token)
            .query_opt("user_id_type", self.user_id_type);

        // 3. 构建请求体
        let request_body = WriteRequestBody {
            group_id: self.group_id,
            scope_visible_setting: self.scope_visible_setting,
            user_ids: self.user_ids,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?;
        let request = request.body(request_body_json);

        // 4. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "更新人员组成员响应数据为空",
        )
        .await
    }
}

/// 写入请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteRequestBody {
    /// 人员组 ID
    pub group_id: String,
    /// 人员组可见性配置（0 无限制/1 后台管理员不可见）
    pub scope_visible_setting: i32,
    /// 人员 ID 列表（覆盖式全量）
    pub user_ids: Vec<String>,
}

/// 更新人员组成员响应（data.write_user_group_scope_data）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WriteResponse {
    /// 更新结果（write_user_group_scope_data）
    pub data: WriteUserGroupScopeData,
}

/// 人员组成员更新结果（write_user_group_scope_data）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WriteUserGroupScopeData {
    /// 更新成功的员工 ID 列表，与入参 user_id_type 类型一致
    #[serde(default)]
    pub success_user_ids: Vec<String>,
    /// 更新失败的员工信息
    #[serde(default)]
    pub fail_user_datas: Vec<WriteUserGroupScopeFailUserData>,
}

/// 更新失败的员工信息（write_user_group_scope_fail_user_data）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WriteUserGroupScopeFailUserData {
    /// 员工 ID，与入参 user_id_type 类型一致
    pub user_id: String,
    /// 失败原因枚举（1：员工未找到）
    pub fail_code: i32,
}

impl ApiResponseTrait for WriteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_performance_v2_user_group_user_rel_write_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应信封：data.data.write_user_group_scope_data
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"data": {"success_user_ids": ["ou_a9dc8d009fd5395c22c7e040e0130692"], "fail_user_datas": [{"user_id": "ou_ff77dba046431fc53ea21a0095df82f4", "fail_code": 1}]}}"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/performance/v2/user_group_user_rels/write"))
            .and(query_param("client_token", "123456"))
            .and(body_json(json!({
                "group_id": "ABCDEFG",
                "scope_visible_setting": 1,
                "user_ids": ["ou_fab1210511b6d2f990ba3b4e2e0dbeef"]
            })))
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

        let data = WriteRequest::new(config, "ABCDEFG".to_string(), 1, "123456".to_string())
            .add_user("ou_fab1210511b6d2f990ba3b4e2e0dbeef".to_string())
            .execute()
            .await
            .expect("performance_v2_user_group_user_rel_write 应成功");

        assert_eq!(
            data.data.success_user_ids,
            vec!["ou_a9dc8d009fd5395c22c7e040e0130692"]
        );
        assert_eq!(data.data.fail_user_datas.len(), 1);
        assert_eq!(data.data.fail_user_datas[0].fail_code, 1);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/performance/v2/user_group_user_rels/write"
        );
    }
}
