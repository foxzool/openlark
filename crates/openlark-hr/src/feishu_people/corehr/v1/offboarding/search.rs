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
///
/// 官方查询参数：`page_size` 必填（1~100），`page_token`/`user_id_type` 可选；
/// 请求体字段全部可选（见 [`SearchRequestBody`]）。
#[derive(Debug, Clone)]
pub struct SearchRequest {
    /// 配置信息
    config: Config,
    /// 请求体
    body: SearchRequestBody,
    /// 分页大小（必填查询参数，取值范围 1~100）
    page_size: Option<i32>,
    /// 分页标记
    page_token: Option<String>,
    /// 用户 ID 类型
    user_id_type: Option<String>,
}

impl SearchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: SearchRequestBody::default(),
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }

    /// 设置分页大小（必填，1~100）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 设置请求体
    pub fn body(mut self, body: SearchRequestBody) -> Self {
        self.body = body;
        self
    }

    /// 设置雇佣 ID 列表
    pub fn employment_ids(mut self, employment_ids: Vec<String>) -> Self {
        self.body.employment_ids = Some(employment_ids);
        self
    }

    /// 设置离职审批发起时间（搜索的起始范围，秒级时间戳）
    pub fn apply_initiating_time_start(mut self, time: String) -> Self {
        self.body.apply_initiating_time_start = Some(time);
        self
    }

    /// 设置离职审批发起时间（搜索的结束范围，秒级时间戳）
    pub fn apply_initiating_time_end(mut self, time: String) -> Self {
        self.body.apply_initiating_time_end = Some(time);
        self
    }

    /// 设置离职审批结束时间（搜索的起始范围，秒级时间戳）
    pub fn apply_finished_time_start(mut self, time: String) -> Self {
        self.body.apply_finished_time_start = Some(time);
        self
    }

    /// 设置离职审批结束时间（搜索的结束范围，秒级时间戳）
    pub fn apply_finished_time_end(mut self, time: String) -> Self {
        self.body.apply_finished_time_end = Some(time);
        self
    }

    /// 设置期望离职日期（搜索的起始范围，日期格式）
    pub fn expected_offboarding_date_start(mut self, date: String) -> Self {
        self.body.expected_offboarding_date_start = Some(date);
        self
    }

    /// 设置期望离职日期（搜索的结束范围，日期格式）
    pub fn expected_offboarding_date_end(mut self, date: String) -> Self {
        self.body.expected_offboarding_date_end = Some(date);
        self
    }

    /// 设置离职日期（搜索的起始范围，日期格式）
    pub fn offboarding_date_start(mut self, date: String) -> Self {
        self.body.offboarding_date_start = Some(date);
        self
    }

    /// 设置离职日期（搜索的结束范围，日期格式）
    pub fn offboarding_date_end(mut self, date: String) -> Self {
        self.body.offboarding_date_end = Some(date);
        self
    }

    /// 设置离职状态列表（多个状态为「或」关系，最多 10 个）
    pub fn statuses(mut self, statuses: Vec<String>) -> Self {
        self.body.statuses = Some(statuses);
        self
    }

    /// 设置离职原因列表
    pub fn reasons(mut self, reasons: Vec<String>) -> Self {
        self.body.reasons = Some(reasons);
        self
    }

    /// 设置离职原因（员工）列表
    pub fn employee_reasons(mut self, employee_reasons: Vec<String>) -> Self {
        self.body.employee_reasons = Some(employee_reasons);
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

        // page_size 为官方必填查询参数，取值范围 1~100
        let page_size = self.page_size.ok_or_else(|| {
            openlark_core::error::validation_error("page_size 不能为空", "分页大小为必填查询参数")
        })?;
        if !(1..=100).contains(&page_size) {
            return Err(openlark_core::error::validation_error(
                "page_size 取值无效",
                format!("分页大小必须在 1-100 之间，当前值为: {page_size}"),
            ));
        }

        let api_endpoint = FeishuPeopleApiV1::OffboardingSearch;
        let request = ApiRequest::<SearchResponse>::post(api_endpoint.to_url())
            .query("page_size", page_size.to_string())
            .query_opt("page_token", self.page_token)
            .query_opt("user_id_type", self.user_id_type)
            .body(serde_json::to_value(&self.body).map_err(|e| {
                openlark_core::error::validation_error(
                    "请求体序列化失败",
                    format!("无法序列化请求参数: {e}"),
                )
            })?);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "搜索离职信息响应数据为空",
        )
        .await
    }
}

/// 搜索离职信息请求体（官方字段全部可选）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchRequestBody {
    /// 雇佣 ID 列表（ID 类型与查询参数 user_id_type 一致）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_ids: Option<Vec<String>>,
    /// 离职审批发起时间（搜索的起始范围，秒级时间戳）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply_initiating_time_start: Option<String>,
    /// 离职审批发起时间（搜索的结束范围，秒级时间戳）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply_initiating_time_end: Option<String>,
    /// 离职审批结束时间（搜索的起始范围，秒级时间戳）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply_finished_time_start: Option<String>,
    /// 离职审批结束时间（搜索的结束范围，秒级时间戳）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply_finished_time_end: Option<String>,
    /// 期望离职日期（搜索的起始范围，日期格式）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_offboarding_date_start: Option<String>,
    /// 期望离职日期（搜索的结束范围，日期格式）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_offboarding_date_end: Option<String>,
    /// 离职日期（搜索的起始范围，日期格式）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_date_start: Option<String>,
    /// 离职日期（搜索的结束范围，日期格式）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_date_end: Option<String>,
    /// 离职状态列表（多个状态为「或」关系，最多 10 个）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<String>>,
    /// 离职原因列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasons: Option<Vec<String>>,
    /// 离职原因（员工）列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_reasons: Option<Vec<String>>,
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
    /// 查询参数（page_size 必填 + page_token/user_id_type 可选）与请求体须与官方文档一致。
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
            .page_size(100)
            .page_token("6891251722631890445".to_string())
            .user_id_type("open_id".to_string())
            .employment_ids(vec!["7140964208476371111".to_string()])
            .apply_initiating_time_start("1672578336".to_string())
            .apply_initiating_time_end("1674133537".to_string())
            .statuses(vec!["Approving".to_string()])
            .reasons(vec!["voluntary".to_string()])
            .employee_reasons(vec!["voluntary".to_string()])
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
        // 查询参数须与官方文档一致：page_size 必填 + page_token/user_id_type 可选
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("page_size=100"));
        assert!(query.contains("page_token=6891251722631890445"));
        assert!(query.contains("user_id_type=open_id"));
        // 请求体须与官方文档一致：仅序列化已设置的可选字段
        let sent: serde_json::Value = received[0].body_json().expect("请求体应为 JSON");
        assert_eq!(
            sent,
            json!({
                "employment_ids": ["7140964208476371111"],
                "apply_initiating_time_start": "1672578336",
                "apply_initiating_time_end": "1674133537",
                "statuses": ["Approving"],
                "reasons": ["voluntary"],
                "employee_reasons": ["voluntary"]
            })
        );
    }

    /// page_size 为官方必填查询参数：缺省时应校验失败
    #[test]
    fn test_search_validation_fails_without_page_size() {
        let config = openlark_core::testing::prelude::TestConfigBuilder::new().build();
        let request = SearchRequest::new(config);
        let rt = tokio::runtime::Runtime::new().expect("创建 tokio runtime 失败");
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }

    /// page_size 官方取值范围为 1~100：越界时应校验失败
    #[test]
    fn test_search_validation_fails_on_invalid_page_size() {
        let config = openlark_core::testing::prelude::TestConfigBuilder::new().build();
        let request = SearchRequest::new(config).page_size(101);
        let rt = tokio::runtime::Runtime::new().expect("创建 tokio runtime 失败");
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }
}
