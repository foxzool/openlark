//! 批量查询员工请假记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/leave/leave_request_history>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 批量查询员工请假记录请求
#[derive(Debug, Clone)]
pub struct LeaveRequestHistoryRequest {
    config: Config,
    body: Option<Value>,
    query_params: Vec<(String, String)>,
}

impl LeaveRequestHistoryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: None,
            query_params: Vec::new(),
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    /// 追加查询参数。
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<LeaveRequestHistoryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<LeaveRequestHistoryResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        let api_endpoint = FeishuPeopleApiV1::LeaveLeaveRequestHistory;
        let mut request = ApiRequest::<LeaveRequestHistoryResponse>::get(api_endpoint.to_url());
        for (key, value) in self.query_params {
            request = request.query(&key, value);
        }
        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "批量查询员工请假记录响应数据为空",
        )
        .await
    }
}

/// 批量查询员工请假记录响应
///
/// 对应官方响应体 `data` 节点（`ResponseFormat::Data` 下直接反序列化），
/// 官方列表键为 `leave_request_list`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LeaveRequestHistoryResponse {
    /// 请假记录信息列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_request_list: Option<Vec<LeaveRequestHistoryItem>>,
    /// 是否还有更多项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记，当 has_more 为 true 时返回
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

/// 请假记录条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LeaveRequestHistoryItem {
    /// 请假记录ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_request_id: Option<String>,
    /// 员工ID，ID 类型与 user_id_type 一致
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 员工姓名（多语言，`[{lang, value}]` 形态）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_name: Option<Vec<crate::common::shared_models::I18nLangValue>>,
    /// 假期类型ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_type_id: Option<String>,
    /// 假期类型名称（多语言，`[{lang, value}]` 形态）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_type_name: Option<Vec<crate::common::shared_models::I18nLangValue>>,
    /// 假期开始时间（如 "2022-09-09" / "2022-09-09 morning"）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 假期结束时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 假期时长
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_duration: Option<String>,
    /// 假期时长单位（1 天 / 2 小时）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_duration_unit: Option<i32>,
    /// 请假记录的状态（1 已通过 / 2 审批中 / 3 审批中（更正）/ 4 审批中（取消休假）/ 5 审批中（返岗）/ 6 已返岗 / 7 已拒绝 / 8 已取消 / 9 已撤回）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_request_status: Option<i32>,
    /// 数据来源（"manual" 手动创建 / "system" 系统创建）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grant_source: Option<String>,
    /// 返岗时间，秒级时间戳
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_time: Option<String>,
    /// 发起时间，秒级时间戳
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<String>,
    /// 发起人，ID 类型与 user_id_type 一致
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submitted_by: Option<String>,
    /// 备注
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// （暂未开放）审批通过日期，格式为 yyyy-MM-dd
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_date: Option<String>,
    /// （暂未开放）是否带薪
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_deducted: Option<bool>,
    /// 请假详情
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<LeaveRequestDetail>>,
    /// 请假申请流程ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_process_id: Option<Vec<String>>,
    /// 请假更正流程ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_correct_process_id: Option<Vec<String>>,
    /// 请假取消流程ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_cancel_process_id: Option<Vec<String>>,
    /// 请假返岗流程ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_return_process_id: Option<Vec<String>>,
    /// WorkDay 专用扣薪类型（1 不参与算薪 / 2 影响算薪 / 3 不影响算薪）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wd_paid_type: Option<i32>,
    /// 请假更正流程信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_correct_process_info: Option<Vec<LeaveProcessInfo>>,
    /// workday 扩展字段信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workday_extend_infos: Option<Vec<LeaveExtendItem>>,
    /// （暂未开放）请假标签配置
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_tag_conf: Option<LeaveTagConf>,
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

/// 请假详情条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LeaveRequestDetail {
    /// 请假记录 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_request_id: Option<String>,
    /// 假期发生日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_date: Option<String>,
    /// 假期时长
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_duration: Option<String>,
    /// 假期时长单位（1 天 / 2 小时）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_duration_unit: Option<i32>,
    /// 是否影响算薪（1 不参与算薪计算 / 2 影响算薪 / 3 不影响算薪）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paid_type: Option<i32>,
    /// （暂未开放）假期类型枚举
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_type_code: Option<String>,
    /// （暂未开放）实际结束日期，格式为 yyyy-MM-dd
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actual_end_date: Option<String>,
    /// （暂未开放）预估结束日期，格式为 yyyy-MM-dd
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estimated_end_date: Option<String>,
    /// 时区
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// （暂未开放）请假记录数据来源
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_source: Option<i32>,
}

/// 请假更正流程信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LeaveProcessInfo {
    /// 流程 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    /// 流程状态（"inProgress" / "rejected" / "withdrawn" / "passed" / "revoked" / "toStart"）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_status: Option<String>,
    /// 流程发起时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_apply_time: Option<String>,
}

/// workday 扩展字段
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LeaveExtendItem {
    /// 扩展字段的键
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 扩展字段的值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 请假标签配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LeaveTagConf {
    /// 请假标签列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<LeaveTag>>,
}

/// 请假标签
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LeaveTag {
    /// 标签键
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

impl ApiResponseTrait for LeaveRequestHistoryResponse {
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

    /// 端到端：GET /open-apis/corehr/v1/leaves/leave_request_history
    #[tokio::test]
    async fn test_leave_history_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v1/leaves/leave_request_history"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "leave_request_list": [
                        {
                            "leave_request_id": "4718803945687580505",
                            "employment_id": "4718803945687580505",
                            "employment_name": [{"lang": "zh-CN", "value": "张三"}],
                            "leave_type_id": "0",
                            "leave_type_name": [{"lang": "zh-CN", "value": "年假"}],
                            "start_time": "2022-07-06",
                            "end_time": "2023-01-05",
                            "leave_duration": "2",
                            "leave_duration_unit": 2,
                            "leave_request_status": 2,
                            "grant_source": "manual",
                            "submitted_at": "1659080476",
                            "submitted_by": "7109664941775241244",
                            "details": [
                                {
                                    "leave_request_id": "4718803945687580505",
                                    "leave_date": "2022-07-07",
                                    "leave_duration": "1",
                                    "leave_duration_unit": 1,
                                    "paid_type": 1
                                }
                            ],
                            "leave_process_id": ["7304865941202929196"]
                        }
                    ],
                    "has_more": true,
                    "page_token": "[1712932008000,\"7356863257632491046\"]"
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

        let resp = LeaveRequestHistoryRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let list = resp
            .leave_request_list
            .expect("leave_request_list 应解析出请假记录列表");
        assert_eq!(list.len(), 1);
        assert_eq!(
            list[0].leave_request_id.as_deref(),
            Some("4718803945687580505")
        );
        assert_eq!(
            list[0].employment_id.as_deref(),
            Some("4718803945687580505")
        );
        assert_eq!(
            list[0]
                .employment_name
                .as_ref()
                .and_then(|n| n.first())
                .and_then(|n| n.value.as_deref()),
            Some("张三")
        );
        assert_eq!(list[0].leave_request_status, Some(2));
        assert_eq!(list[0].details.as_ref().map(|d| d.len()), Some(1));
        assert_eq!(resp.has_more, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
