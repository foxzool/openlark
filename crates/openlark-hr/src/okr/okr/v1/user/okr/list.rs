//! 获取用户的 OKR 列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v1/user.okr/list>

use crate::okr::okr::v1::okr::batch_get::OkrBatch;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 获取用户的 OKR 列表请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 目标用户 ID（必填，路径参数）
    user_id: String,
    /// 请求列表的偏移（必填，对应响应体的 okr_list 字段，>= 0）
    offset: i32,
    /// 列表长度（必填，0-10）
    limit: i32,
    /// 用户 ID 类型（可选，默认 open_id）
    user_id_type: Option<String>,
    /// 请求 OKR 的语言版本（可选，zh_cn/en_us，默认 zh_cn）
    lang: Option<String>,
    /// 周期 ID 列表（可选，最多 10 个）
    period_ids: Vec<String>,
    /// 配置信息
    config: Config,
}

impl ListRequest {
    /// 创建请求
    ///
    /// - `user_id`: 目标用户 ID
    /// - `offset`: 请求列表的偏移（>= 0）
    /// - `limit`: 列表长度（0-10）
    pub fn new(config: Config, user_id: String, offset: i32, limit: i32) -> Self {
        Self {
            user_id,
            offset,
            limit,
            user_id_type: None,
            lang: None,
            period_ids: Vec::new(),
            config,
        }
    }

    /// 设置用户 ID 类型（可选，open_id/union_id/user_id/people_admin_id）
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 设置语言版本（可选，zh_cn/en_us）
    pub fn lang(mut self, lang: String) -> Self {
        self.lang = Some(lang);
        self
    }

    /// 设置周期 ID 列表（可选，最多 10 个）
    pub fn period_ids(mut self, period_ids: Vec<String>) -> Self {
        self.period_ids = period_ids;
        self
    }

    fn validate(&self) -> SDKResult<()> {
        validate_required!(self.user_id.trim(), "user_id");
        if self.offset < 0 {
            return Err(openlark_core::error::validation_error(
                "offset 无效",
                "offset 必须为不小于 0 的整数",
            ));
        }
        if !(0..=10).contains(&self.limit) {
            return Err(openlark_core::error::validation_error(
                "limit 无效",
                "limit 取值范围为 0-10",
            ));
        }
        if !self.period_ids.is_empty() && self.period_ids.len() > 10 {
            return Err(openlark_core::error::validation_error(
                "period_ids 无效",
                "period_ids 不能为空且不能超过 10 个",
            ));
        }
        Ok(())
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        self.validate()?;

        let mut request = ApiRequest::<ListResponse>::get(format!(
            "/open-apis/okr/v1/users/{}/okrs",
            self.user_id
        ));

        // 官方 offset/limit 类型为 string，此处以整数建模后按字符串传递
        request = request
            .query("offset", self.offset.to_string())
            .query("limit", self.limit.to_string());

        if let Some(ref user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }
        if let Some(ref lang) = self.lang {
            request = request.query("lang", lang);
        }
        // 官方类型为 string[]，与 okr/batch_get 的 okr_ids 相同以逗号拼接传递
        if !self.period_ids.is_empty() {
            request = request.query("period_ids", self.period_ids.join(","));
        }

        // 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取用户的 OKR 列表响应数据为空",
        )
        .await
    }
}

/// 获取用户的 OKR 列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// OKR 列表（okr_batch[]，复用 okr/batch_get 的建模）
    #[serde(default)]
    pub okr_list: Vec<OkrBatch>,
    /// OKR 周期总数
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

impl ApiResponseTrait for ListResponse {
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
    async fn test_list_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状：data.okr_list[]（okr_batch）+ total
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"okr_list": [{"id": "7072252816005349396", "name": "2022 年 3 月", "period_id": "7067724095781142548", "permission": 1, "confirm_status": 4, "objective_list": []}], "total": 14}"#,
        )
        .unwrap();
        Mock::given(method("GET"))
            .and(path("/open-apis/okr/v1/users/user_001/okrs"))
            .and(query_param("offset", "0"))
            .and(query_param("limit", "10"))
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

        let data = ListRequest::new(config, "user_001".to_string(), 0, 10)
            .execute()
            .await
            .expect("list 应成功");

        assert_eq!(data.okr_list.len(), 1);
        assert_eq!(data.okr_list[0].id, "7072252816005349396");
        assert_eq!(data.okr_list[0].confirm_status, Some(4));
        assert_eq!(data.total, Some(14));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/okr/v1/users/user_001/okrs"
        );
        // 必填查询参数 offset/limit 须出现在 query string 中
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("offset=0"), "缺 offset: {query}");
        assert!(query.contains("limit=10"), "缺 limit: {query}");
    }
}
