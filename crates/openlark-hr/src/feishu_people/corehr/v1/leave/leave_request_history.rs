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
#[derive(Debug, Clone, Default)]
pub struct LeaveRequestHistoryRequest {
    config: Config,
    /// 分页大小（必填，最小 10，最大 1000）
    page_size: Option<i32>,
    /// 分页标记（可选，第一次请求不填表示从头开始遍历）
    page_token: Option<String>,
    /// 员工 ID 列表（可选，最大 100 个，不传则查询全部员工）
    employment_id_list: Vec<String>,
    /// 休假发起人 ID 列表（可选，最大 100 个）
    initiator_id_list: Vec<String>,
    /// 请假记录状态列表（可选，1～9，不填为不过滤状态）
    leave_request_status: Vec<i32>,
    /// 假期类型 ID 列表（可选）
    leave_type_id_list: Vec<String>,
    /// 休假开始时间晚于等于的日期（可选，`yyyy-MM-dd`）
    leave_start_date_min: Option<String>,
    /// 休假开始时间早于等于的日期（可选，`yyyy-MM-dd`）
    leave_start_date_max: Option<String>,
    /// 休假结束时间晚于等于的日期（可选，`yyyy-MM-dd`）
    leave_end_date_min: Option<String>,
    /// 休假结束时间早于等于的日期（可选，`yyyy-MM-dd`）
    leave_end_date_max: Option<String>,
    /// 休假发起时间晚于等于的日期（可选，`yyyy-MM-dd`）
    leave_submit_date_min: Option<String>,
    /// 休假发起时间早于等于的日期（可选，`yyyy-MM-dd`）
    leave_submit_date_max: Option<String>,
    /// 用户 ID 类型（可选：open_id / union_id / user_id / people_corehr_id，默认 people_corehr_id）
    user_id_type: Option<String>,
    /// 请假记录更新时间晚于等于的时间（可选，`yyyy-MM-dd HH:mm:ss`）
    leave_update_time_min: Option<String>,
    /// 请假记录更新时间早于等于的时间（可选，`yyyy-MM-dd HH:mm:ss`）
    leave_update_time_max: Option<String>,
    /// 是否返回请假详情（可选，暂未开放，默认 false）
    return_detail: Option<bool>,
    /// 长/短假类型（可选：0 不过滤 / 1 短假 / 2 长假，默认 0）
    leave_term_type: Option<i32>,
    /// 请假记录所在时区（可选，如 Asia/Shanghai）
    time_zone: Option<String>,
    /// 请假记录数据源（可选，暂未开放：1 中国大陆 / 2 海外）
    data_source: Option<i32>,
    /// 请假记录 DB 更新时间晚于等于的时间（可选，暂未开放）
    db_update_time_min: Option<String>,
    /// 请假记录 DB 更新时间早于等于的时间（可选，暂未开放）
    db_update_time_max: Option<String>,
    /// WorkDay 专用：是否返回 0 值请假记录（可选，默认 false）
    wd_need_amount_zero_records: Option<bool>,
    /// WorkDay 专用：是否返回拒绝和取消的请假记录（可选，默认 false）
    wd_need_denied_and_canceled_record: Option<bool>,
    /// WorkDay 专用扣薪类型（可选：1 不参与算薪 / 2 影响算薪 / 3 不影响算薪）
    wd_paid_type: Option<i32>,
}

impl LeaveRequestHistoryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    /// 设置分页大小（必填，最小 10，最大 1000）。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（可选）。
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 设置员工 ID 列表（可选，最大 100 个）。
    pub fn employment_id_list(mut self, employment_id_list: Vec<String>) -> Self {
        self.employment_id_list = employment_id_list;
        self
    }

    /// 设置休假发起人 ID 列表（可选，最大 100 个）。
    pub fn initiator_id_list(mut self, initiator_id_list: Vec<String>) -> Self {
        self.initiator_id_list = initiator_id_list;
        self
    }

    /// 设置请假记录状态列表（可选，1～9）。
    pub fn leave_request_status(mut self, leave_request_status: Vec<i32>) -> Self {
        self.leave_request_status = leave_request_status;
        self
    }

    /// 设置假期类型 ID 列表（可选）。
    pub fn leave_type_id_list(mut self, leave_type_id_list: Vec<String>) -> Self {
        self.leave_type_id_list = leave_type_id_list;
        self
    }

    /// 设置休假开始时间下界（可选，`yyyy-MM-dd`）。
    pub fn leave_start_date_min(mut self, v: String) -> Self {
        self.leave_start_date_min = Some(v);
        self
    }

    /// 设置休假开始时间上界（可选，`yyyy-MM-dd`）。
    pub fn leave_start_date_max(mut self, v: String) -> Self {
        self.leave_start_date_max = Some(v);
        self
    }

    /// 设置休假结束时间下界（可选，`yyyy-MM-dd`）。
    pub fn leave_end_date_min(mut self, v: String) -> Self {
        self.leave_end_date_min = Some(v);
        self
    }

    /// 设置休假结束时间上界（可选，`yyyy-MM-dd`）。
    pub fn leave_end_date_max(mut self, v: String) -> Self {
        self.leave_end_date_max = Some(v);
        self
    }

    /// 设置休假发起时间下界（可选，`yyyy-MM-dd`）。
    pub fn leave_submit_date_min(mut self, v: String) -> Self {
        self.leave_submit_date_min = Some(v);
        self
    }

    /// 设置休假发起时间上界（可选，`yyyy-MM-dd`）。
    pub fn leave_submit_date_max(mut self, v: String) -> Self {
        self.leave_submit_date_max = Some(v);
        self
    }

    /// 设置用户 ID 类型（可选，默认 people_corehr_id）。
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 设置请假记录更新时间下界（可选，`yyyy-MM-dd HH:mm:ss`）。
    pub fn leave_update_time_min(mut self, v: String) -> Self {
        self.leave_update_time_min = Some(v);
        self
    }

    /// 设置请假记录更新时间上界（可选，`yyyy-MM-dd HH:mm:ss`）。
    pub fn leave_update_time_max(mut self, v: String) -> Self {
        self.leave_update_time_max = Some(v);
        self
    }

    /// 设置是否返回请假详情（可选，暂未开放）。
    pub fn return_detail(mut self, return_detail: bool) -> Self {
        self.return_detail = Some(return_detail);
        self
    }

    /// 设置长/短假类型（可选：0 不过滤 / 1 短假 / 2 长假）。
    pub fn leave_term_type(mut self, leave_term_type: i32) -> Self {
        self.leave_term_type = Some(leave_term_type);
        self
    }

    /// 设置请假记录所在时区（可选）。
    pub fn time_zone(mut self, time_zone: String) -> Self {
        self.time_zone = Some(time_zone);
        self
    }

    /// 设置请假记录数据源（可选，暂未开放：1 中国大陆 / 2 海外）。
    pub fn data_source(mut self, data_source: i32) -> Self {
        self.data_source = Some(data_source);
        self
    }

    /// 设置请假记录 DB 更新时间下界（可选，暂未开放）。
    pub fn db_update_time_min(mut self, v: String) -> Self {
        self.db_update_time_min = Some(v);
        self
    }

    /// 设置请假记录 DB 更新时间上界（可选，暂未开放）。
    pub fn db_update_time_max(mut self, v: String) -> Self {
        self.db_update_time_max = Some(v);
        self
    }

    /// WorkDay 专用：设置是否返回 0 值请假记录（可选）。
    pub fn wd_need_amount_zero_records(mut self, v: bool) -> Self {
        self.wd_need_amount_zero_records = Some(v);
        self
    }

    /// WorkDay 专用：设置是否返回拒绝和取消的请假记录（可选）。
    pub fn wd_need_denied_and_canceled_record(mut self, v: bool) -> Self {
        self.wd_need_denied_and_canceled_record = Some(v);
        self
    }

    /// WorkDay 专用：设置扣薪类型（可选：1 / 2 / 3）。
    pub fn wd_paid_type(mut self, wd_paid_type: i32) -> Self {
        self.wd_paid_type = Some(wd_paid_type);
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

        let page_size = self.page_size.unwrap_or(0);
        if !(10..=1000).contains(&page_size) {
            return Err(openlark_core::error::validation_error(
                "page_size",
                "page_size 不能为空且取值范围为 10～1000",
            ));
        }

        let api_endpoint = FeishuPeopleApiV1::LeaveLeaveRequestHistory;
        let request = ApiRequest::<LeaveRequestHistoryResponse>::get(api_endpoint.to_url())
            .query("page_size", page_size.to_string())
            .query_opt("page_token", self.page_token)
            // GET 数组参数按官方规范同名多次传递
            .query_multi("employment_id_list", self.employment_id_list)
            .query_multi("initiator_id_list", self.initiator_id_list)
            .query_multi(
                "leave_request_status",
                self.leave_request_status
                    .iter()
                    .map(|status| status.to_string()),
            )
            .query_multi("leave_type_id_list", self.leave_type_id_list)
            .query_opt("leave_start_date_min", self.leave_start_date_min)
            .query_opt("leave_start_date_max", self.leave_start_date_max)
            .query_opt("leave_end_date_min", self.leave_end_date_min)
            .query_opt("leave_end_date_max", self.leave_end_date_max)
            .query_opt("leave_submit_date_min", self.leave_submit_date_min)
            .query_opt("leave_submit_date_max", self.leave_submit_date_max)
            .query_opt("user_id_type", self.user_id_type)
            .query_opt("leave_update_time_min", self.leave_update_time_min)
            .query_opt("leave_update_time_max", self.leave_update_time_max)
            .query_opt("return_detail", self.return_detail.map(|v| v.to_string()))
            .query_opt(
                "leave_term_type",
                self.leave_term_type.map(|v| v.to_string()),
            )
            .query_opt("time_zone", self.time_zone)
            .query_opt("data_source", self.data_source.map(|v| v.to_string()))
            .query_opt("db_update_time_min", self.db_update_time_min)
            .query_opt("db_update_time_max", self.db_update_time_max)
            .query_opt(
                "wd_need_amount_zero_records",
                self.wd_need_amount_zero_records.map(|v| v.to_string()),
            )
            .query_opt(
                "wd_need_denied_and_canceled_record",
                self.wd_need_denied_and_canceled_record
                    .map(|v| v.to_string()),
            )
            .query_opt("wd_paid_type", self.wd_paid_type.map(|v| v.to_string()));

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
            .page_size(100)
            .employment_id_list(vec!["6919733291281024522".to_string()])
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
        // page_size 为官方必填查询参数；employment_id_list 按同名多次传递
        let query = received[0].url.query().unwrap_or_default();
        assert!(
            query.contains("page_size=100")
                && query.contains("employment_id_list=6919733291281024522"),
            "查询串应含 page_size=100 与 employment_id_list，实际 {query}"
        );
    }
}
