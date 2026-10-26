//! 查询流程实例列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/process/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::CoreError,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 查询流程实例列表请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 配置信息
    config: Config,
    /// 流程状态列表（可选，1 进行中 / 2 已拒绝 / 4 已撤回 / 8 已撤销 / 9 已完成 / 15 撤销中；
    /// 官方要求同一参数名多次传递，如 statuses=1&statuses=2）
    statuses: Option<Vec<i32>>,
    /// 分页标记
    page_token: Option<String>,
    /// 分页大小（必填，取值范围 1～100）
    page_size: Option<i32>,
    /// 流程实例修改时间的查询起始值（必填，毫秒时间戳，闭区间）
    modify_time_from: String,
    /// 流程实例修改时间的查询终止值（必填，毫秒时间戳，闭区间，与起始值跨度小于 31 天）
    modify_time_to: String,
    /// 流程定义 ID
    flow_definition_id: Option<String>,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            statuses: None,
            page_token: None,
            page_size: None,
            modify_time_from: String::new(),
            modify_time_to: String::new(),
            flow_definition_id: None,
        }
    }

    /// 设置流程状态列表（同一参数名多次传递）
    pub fn statuses(mut self, statuses: Vec<i32>) -> Self {
        self.statuses = Some(statuses);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置分页大小（必填，取值范围 1～100）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置流程实例修改时间的查询起始值（必填，毫秒时间戳）
    pub fn modify_time_from(mut self, modify_time_from: impl Into<String>) -> Self {
        self.modify_time_from = modify_time_from.into();
        self
    }

    /// 设置流程实例修改时间的查询终止值（必填，毫秒时间戳）
    pub fn modify_time_to(mut self, modify_time_to: impl Into<String>) -> Self {
        self.modify_time_to = modify_time_to.into();
        self
    }

    /// 设置流程定义 ID
    pub fn flow_definition_id(mut self, flow_definition_id: impl Into<String>) -> Self {
        self.flow_definition_id = Some(flow_definition_id.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        let page_size = self
            .page_size
            .ok_or_else(|| CoreError::validation_msg("page_size 不能为空"))?;
        if !(1..=100).contains(&page_size) {
            return Err(CoreError::validation_msg("page_size 取值范围为 1～100"));
        }
        validate_required!(self.modify_time_from, "modify_time_from 不能为空");
        validate_required!(self.modify_time_to, "modify_time_to 不能为空");

        let request = ApiRequest::<ListResponse>::get("/open-apis/corehr/v2/processes")
            .query_multi(
                "statuses",
                self.statuses
                    .unwrap_or_default()
                    .into_iter()
                    .map(|status| status.to_string()),
            )
            .query("page_size", page_size.to_string())
            .query("modify_time_from", self.modify_time_from)
            .query("modify_time_to", self.modify_time_to)
            .query_opt("page_token", self.page_token)
            .query_opt("flow_definition_id", self.flow_definition_id);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 查询流程实例列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 响应数据
    /// 流程实例 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_ids: Option<Vec<String>>,
    /// 分页令牌
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/corehr/v2/processes（必填查询参数 + statuses 同名多次传递）
    #[tokio::test]
    async fn test_list_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v2/processes"))
            .and(query_param("statuses", "1"))
            .and(query_param("statuses", "9"))
            .and(query_param("page_size", "10"))
            .and(query_param("modify_time_from", "1547654251506"))
            .and(query_param("modify_time_to", "1550246251506"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "process_ids": ["7278949005675988535"],
                    "has_more": true,
                    "page_token": "7278949005675988536"
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

        let resp = ListRequest::new(config)
            .statuses(vec![1, 9])
            .page_size(10)
            .modify_time_from("1547654251506")
            .modify_time_to("1550246251506")
            .execute()
            .await
            .expect("请求应成功");

        assert_eq!(
            resp.process_ids,
            Some(vec!["7278949005675988535".to_string()])
        );

        // statuses 同名多次传递：statuses=1&statuses=9
        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let statuses_values: Vec<String> = received[0]
            .url
            .query_pairs()
            .filter(|(k, _)| k == "statuses")
            .map(|(_, v)| v.to_string())
            .collect();
        assert_eq!(statuses_values, vec!["1".to_string(), "9".to_string()]);
    }
}
