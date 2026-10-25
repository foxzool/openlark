//! 搜索离职信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/offboarding/search>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 搜索离职信息请求
#[derive(Debug, Clone)]
pub struct SearchRequest {
    /// 配置信息
    config: Config,
    body: Value,
}

impl SearchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: Value::Object(serde_json::Map::new()),
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: Value) -> Self {
        self.body = body;
        self
    }

    /// 向请求体添加字段。
    pub fn field(mut self, key: impl Into<String>, value: Value) -> Self {
        if !self.body.is_object() {
            self.body = Value::Object(serde_json::Map::new());
        }
        if let Some(body) = self.body.as_object_mut() {
            body.insert(key.into(), value);
        }
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SearchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SearchResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        let api_endpoint = FeishuPeopleApiV1::OffboardingSearch;
        let request = ApiRequest::<SearchResponse>::post(api_endpoint.to_url()).body(self.body);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "搜索离职信息响应数据为空",
        )
        .await
    }
}

/// 搜索离职信息响应
///
/// 响应 `data` 子树形状：`{items, page_token, has_more}`（由 decode 层解包）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponse {
    /// 查询的员工离职信息列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<OffboardingItem>>,
    /// 分页标记，当 has_more 为 true 时返回
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 离职条目（官方 `offboarding` 结构）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OffboardingItem {
    /// 离职审批信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_info: Option<ApplicationInfo>,
    /// 离职发起类型：offboarding_initiated_by_self（员工申请离职）/
    /// offboarding_initiated_by_others（代发起离职申请）/ offboarding_directly（直接离职）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initiating_type: Option<String>,
    /// 离职办理流程信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_checklist: Option<OffboardingChecklist>,
    /// 离职唯一标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_id: Option<String>,
    /// 员工离职信息（官方约 30 个权限门控字段：离职原因/竞业/电子签等，
    /// 暂不逐一建模，原样透传）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_info: Option<Value>,
    /// 离职状态：Approving/Approved/Offboarded/Rejected/Withdrawn/NoNeedApproval
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

/// 离职审批信息（官方 `application_info` 结构）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationInfo {
    /// 离职审批发起人的雇佣 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply_initiator_id: Option<String>,
    /// 离职申请流程发起时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply_initiating_time: Option<String>,
    /// 离职申请流程结束时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply_finish_time: Option<String>,
    /// 流程 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
}

/// 离职办理流程信息（官方 `offboarding_checklist` 结构）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OffboardingChecklist {
    /// 离职流转状态：AntiBegin/Approving/Finished/Rejected/Withdrawn
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checklist_status: Option<String>,
    /// 离职流转开始时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checklist_start_time: Option<String>,
    /// 离职流转结束时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checklist_finish_time: Option<String>,
    /// 离职流转流程实例 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checklist_process_id: Option<String>,
}

impl ApiResponseTrait for SearchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v1/offboardings/search
    #[tokio::test]
    async fn test_search_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v1/offboardings/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "has_more": false,
                    "items": [
                        {
                            "application_info": {
                                "apply_finish_time": "2022-02-03 11:22:33",
                                "apply_initiating_time": "2022-02-03 11:22:33",
                                "apply_initiator_id": "6838119494196871234",
                                "process_id": "6838119494196871234"
                            },
                            "initiating_type": "offboarding_directly",
                            "offboarding_checklist": {
                                "checklist_finish_time": "2022-02-03 11:22:33",
                                "checklist_process_id": "6838119494196871234",
                                "checklist_start_time": "2022-02-03 11:22:33",
                                "checklist_status": "AntiBegin"
                            },
                            "offboarding_id": "7298499290417251879",
                            "offboarding_info": {
                                "add_block_list": "false"
                            },
                            "status": "Approving"
                        }
                    ]
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

        let resp = SearchRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let item = resp
            .items
            .as_ref()
            .and_then(|items| items.first())
            .expect("items 应包含离职条目");
        assert_eq!(
            item.initiating_type.as_deref(),
            Some("offboarding_directly")
        );
        assert_eq!(item.status.as_deref(), Some("Approving"));
        assert_eq!(item.offboarding_id.as_deref(), Some("7298499290417251879"));
        let application_info = item
            .application_info
            .as_ref()
            .expect("application_info 应存在");
        assert_eq!(
            application_info.apply_initiator_id.as_deref(),
            Some("6838119494196871234")
        );
        assert_eq!(
            application_info.apply_finish_time.as_deref(),
            Some("2022-02-03 11:22:33")
        );
        let checklist = item
            .offboarding_checklist
            .as_ref()
            .expect("offboarding_checklist 应存在");
        assert_eq!(checklist.checklist_status.as_deref(), Some("AntiBegin"));
        assert_eq!(
            checklist.checklist_process_id.as_deref(),
            Some("6838119494196871234")
        );
        assert!(item.offboarding_info.is_some(), "offboarding_info 应透传");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
