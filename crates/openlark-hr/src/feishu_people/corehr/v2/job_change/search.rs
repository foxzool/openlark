//! 搜索员工异动信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/job_change/search>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::common::shared_models::I18nText;

/// SearchRequest
#[derive(Debug, Clone)]
pub struct SearchRequest {
    /// 配置信息
    config: Config,
    /// 请求体（可选）
    body: Option<Value>,
}

impl SearchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self { config, body: None }
    }

    /// 设置请求体
    pub fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
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
        let mut request =
            ApiRequest::<SearchResponse>::post("/open-apis/corehr/v2/job_changes/search");

        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// SearchResponse
///
/// 对应官方响应体 `data` 节点（`ResponseFormat::Data` 下直接反序列化）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponse {
    /// 员工异动列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<JobChangeSearchItem>>,
    /// 是否还有更多项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记，当 has_more 为 true 时返回
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 员工异动条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobChangeSearchItem {
    /// 异动记录 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_change_id: Option<String>,
    /// 雇员 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 异动状态（Approving / Approved / Transformed / Rejected / Cancelled / NoNeedApproval）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 异动类型唯一标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_type_unique_identifier: Option<String>,
    /// 异动原因唯一标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_reason_unique_identifier: Option<String>,
    /// 异动流程 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    /// 生效时间，格式："YYYY-MM-DD"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// 创建时间，毫秒级时间戳
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// 更新时间，毫秒级时间戳
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
    /// 异动详细信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_info: Option<TransferInfo>,
    /// 是否调整薪酬
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_adjust_salary: Option<bool>,
    /// 异动自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldData>>,
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

/// 异动详细信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TransferInfo {
    /// 备注
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// offer 信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer_info: Option<String>,
    /// 是否撤销虚线上级
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_dotted_manager_clean: Option<bool>,
    /// 是否有试用期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_exist: Option<bool>,
    /// 原部门
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_department: Option<String>,
    /// 新部门
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_department: Option<String>,
    /// 原工作地点
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_work_location: Option<String>,
    /// 新工作地点
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_work_location: Option<String>,
    /// 原直属上级
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_direct_manager: Option<String>,
    /// 新直属上级
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_direct_manager: Option<String>,
    /// 原虚线上级
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_dotted_manager: Option<String>,
    /// 新虚线上级
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_dotted_manager: Option<String>,
    /// 原职务
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_job: Option<String>,
    /// 新职务
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_job: Option<String>,
    /// 原序列
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_job_family: Option<String>,
    /// 新序列
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_job_family: Option<String>,
    /// 原职级
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_job_level: Option<String>,
    /// 新级别
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_job_level: Option<String>,
    /// 原人员类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_workforce_type: Option<String>,
    /// 新人员类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_workforce_type: Option<String>,
    /// 原人员子类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_employee_subtype: Option<String>,
    /// 新人员子类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_employee_subtype: Option<String>,
    /// 原公司
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_company: Option<String>,
    /// 新公司
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_company: Option<String>,
    /// 原合同编号
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_contract_number: Option<String>,
    /// 新合同编号
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_contract_number: Option<String>,
    /// 原合同类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_contract_type: Option<String>,
    /// 新合同类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_contract_type: Option<String>,
    /// 原期限类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_duration_type: Option<String>,
    /// 新期限类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_duration_type: Option<String>,
    /// 原签订类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_signing_type: Option<String>,
    /// 新签订类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_signing_type: Option<String>,
    /// 原合同开始日期，格式："YYYY-MM-DD"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_contract_start_date: Option<String>,
    /// 新合同开始日期，格式："YYYY-MM-DD"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_contract_start_date: Option<String>,
    /// 原合同结束日期，格式："YYYY-MM-DD"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_contract_end_date: Option<String>,
    /// 新合同结束日期，格式："YYYY-MM-DD"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_contract_end_date: Option<String>,
    /// 原工时制度
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_working_hours_type: Option<String>,
    /// 新工时制度
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_working_hours_type: Option<String>,
    /// 原工作日历
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_working_calendar: Option<String>,
    /// 新工作日历
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_working_calendar: Option<String>,
    /// 原试用期预计结束日期，格式："YYYY-MM-DD"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_probation_end_date: Option<String>,
    /// 新试用期预计结束日期，格式："YYYY-MM-DD"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_probation_end_date: Option<String>,
    /// 原周工作时长
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_weekly_working_hours: Option<String>,
    /// 新周工作时长
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_weekly_working_hours: Option<String>,
    /// 原排班
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_work_shift: Option<String>,
    /// 新排班
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_work_shift: Option<String>,
    /// 原成本中心分摊方式
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_cost_center_rate: Option<Vec<JobDataCostCenter>>,
    /// 新成本中心分摊方式
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_cost_center_rate: Option<Vec<JobDataCostCenter>>,
    /// 原工作信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_employment_change: Option<TransferEmploymentInfo>,
    /// 新工作信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_employment_change: Option<TransferEmploymentInfo>,
    /// 原职等
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_job_grade: Option<String>,
    /// 新职等
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_job_grade: Option<String>,
    /// 原薪资类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_compensation_type: Option<String>,
    /// 新薪资类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_compensation_type: Option<String>,
    /// 原任职公司
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_service_company: Option<String>,
    /// 新任职公司
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_service_company: Option<String>,
    /// 原岗位
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_position: Option<String>,
    /// 新岗位
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_position: Option<String>,
    /// 原社保城市
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_social_security_city: Option<String>,
    /// 新社保城市
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_social_security_city: Option<String>,
    /// 原通道
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_pathway: Option<String>,
    /// 新通道
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_pathway: Option<String>,
    /// 编制随人员一起调整
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_transfer_with_workforce: Option<bool>,
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

/// 成本中心分摊方式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobDataCostCenter {
    /// 成本中心 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    /// 分摊比例（整数）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<i32>,
}

/// 异动工作信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TransferEmploymentInfo {
    /// 转正式员工日期，格式："YYYY-MM-DD"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regular_employee_start_date: Option<String>,
    /// 司龄起算日期，格式："YYYY-MM-DD"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seniority_date: Option<String>,
    /// 员工编号
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_number: Option<String>,
    /// 自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldData>>,
}

/// 异动自定义字段
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CustomFieldData {
    /// 自定义字段 apiname，即自定义字段的唯一标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_api_name: Option<String>,
    /// 自定义字段名称（`{zh_cn, en_us}`）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 自定义字段类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    /// 字段值，是 json 转义后的字符串
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
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

    /// 端到端：POST /open-apis/corehr/v2/job_changes/search
    #[tokio::test]
    async fn test_search_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/job_changes/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "job_change_id": "6991776076699549697",
                            "employment_id": "ou_a294793e8fa21529f2a60e3e9de45520",
                            "status": "Approved",
                            "transfer_type_unique_identifier": "direct_leader_change",
                            "transfer_reason_unique_identifier": "involuntary_transfer",
                            "process_id": "6991776078461142564",
                            "effective_date": "2022-03-01",
                            "created_time": "1627899724000",
                            "updated_time": "1647434443000",
                            "transfer_info": {
                                "remark": "异动详情",
                                "offer_info": "优质人才，加急处理",
                                "target_dotted_manager_clean": true,
                                "probation_exist": false,
                                "original_department": "6966236933198579208",
                                "target_department": "6966236933198579208",
                                "original_cost_center_rate": [
                                    {"cost_center_id": "6950635856373745165", "rate": 100}
                                ],
                                "original_employment_change": {
                                    "regular_employee_start_date": "2023-01-01",
                                    "seniority_date": "2023-01-01",
                                    "employee_number": "1111111",
                                    "custom_fields": [
                                        {
                                            "custom_api_name": "name",
                                            "name": {"zh_cn": "自定义姓名", "en_us": "Custom Name"},
                                            "type": 1,
                                            "value": "231"
                                        }
                                    ]
                                },
                                "is_transfer_with_workforce": false
                            },
                            "is_adjust_salary": true,
                            "custom_fields": [
                                {
                                    "custom_api_name": "name",
                                    "name": {"zh_cn": "自定义姓名", "en_us": "Custom Name"},
                                    "type": 1,
                                    "value": "\"231\""
                                }
                            ]
                        }
                    ],
                    "has_more": true,
                    "page_token": "6891251722631890445"
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

        let items = resp.items.expect("items 应解析出异动列表");
        assert_eq!(items.len(), 1);
        assert_eq!(
            items[0].job_change_id.as_deref(),
            Some("6991776076699549697")
        );
        assert_eq!(items[0].status.as_deref(), Some("Approved"));
        assert_eq!(
            items[0].transfer_type_unique_identifier.as_deref(),
            Some("direct_leader_change")
        );
        let transfer_info = items[0]
            .transfer_info
            .as_ref()
            .expect("transfer_info 应解析");
        assert_eq!(transfer_info.remark.as_deref(), Some("异动详情"));
        assert_eq!(transfer_info.target_dotted_manager_clean, Some(true));
        assert_eq!(transfer_info.probation_exist, Some(false));
        assert_eq!(transfer_info.is_transfer_with_workforce, Some(false));
        assert_eq!(
            transfer_info
                .original_cost_center_rate
                .as_ref()
                .and_then(|c| c.first())
                .and_then(|c| c.cost_center_id.as_deref()),
            Some("6950635856373745165")
        );
        assert_eq!(items[0].is_adjust_salary, Some(true));
        assert_eq!(
            items[0]
                .custom_fields
                .as_ref()
                .and_then(|f| f.first())
                .and_then(|f| f.custom_api_name.as_deref()),
            Some("name")
        );
        assert_eq!(resp.has_more, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
