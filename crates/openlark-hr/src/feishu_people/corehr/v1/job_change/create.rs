//! 发起员工异动(不推荐)
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/job_change/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// CreateRequest
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 配置信息
    config: Config,
    /// 请求体
    body: CreateBody,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: CreateBody::default(),
        }
    }

    /// 设置异动方式 `transfer_mode`（必填：1 直接异动（无审批）/ 2 正常异动（完整流程））。
    pub fn transfer_mode(mut self, transfer_mode: i32) -> Self {
        self.body.transfer_mode = transfer_mode;
        self
    }

    /// 设置雇员 ID `employment_id`（必填）。
    pub fn employment_id(mut self, employment_id: String) -> Self {
        self.body.employment_id = employment_id;
        self
    }

    /// 设置异动类型唯一标识 `transfer_type_unique_identifier`（必填）。
    pub fn transfer_type_unique_identifier(mut self, v: String) -> Self {
        self.body.transfer_type_unique_identifier = v;
        self
    }

    /// 设置异动生效日期 `effective_date`（必填，格式 `YYYY-MM-DD`）。
    pub fn effective_date(mut self, effective_date: String) -> Self {
        self.body.effective_date = effective_date;
        self
    }

    /// 设置异动详细信息 `transfer_info`（必填，字段均为可选）。
    pub fn transfer_info(mut self, transfer_info: TransferInfo) -> Self {
        self.body.transfer_info = transfer_info;
        self
    }

    /// 设置审批流程 ID `flow_id`（可选）。
    pub fn flow_id(mut self, flow_id: String) -> Self {
        self.body.flow_id = Some(flow_id);
        self
    }

    /// 设置异动唯一标识 `transfer_key`（可选）。
    pub fn transfer_key(mut self, transfer_key: String) -> Self {
        self.body.transfer_key = Some(transfer_key);
        self
    }

    /// 设置操作发起人 ID `initiator_id`（可选）。
    pub fn initiator_id(mut self, initiator_id: String) -> Self {
        self.body.initiator_id = Some(initiator_id);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        validate_required!(self.body.employment_id, "employment_id 不能为空");
        validate_required!(
            self.body.transfer_type_unique_identifier,
            "transfer_type_unique_identifier 不能为空"
        );
        validate_required!(self.body.effective_date, "effective_date 不能为空");
        if !matches!(self.body.transfer_mode, 1 | 2) {
            return Err(openlark_core::error::validation_error(
                "transfer_mode",
                "transfer_mode 须为 1（直接异动，无审批）或 2（正常异动，完整流程）",
            ));
        }

        let api_endpoint = FeishuPeopleApiV1::JobChangeCreate;
        let request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url())
            .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 发起员工异动请求体（`transfer_mode`/`employment_id`/`transfer_type_unique_identifier`/`effective_date`/`transfer_info` 必填）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateBody {
    /// 异动方式：1 直接异动（无审批）；2 正常异动（完整流程）
    pub transfer_mode: i32,
    /// 雇员 id，可通过【搜索员工信息】接口获取
    pub employment_id: String,
    /// 异动类型唯一标识，可通过【获取异动类型列表】接口获取详细信息
    pub transfer_type_unique_identifier: String,
    /// 异动生效日期，格式 `YYYY-MM-DD`
    pub effective_date: String,
    /// 异动详细信息
    pub transfer_info: TransferInfo,
    /// 异动发起后审批流程 id，可通过【流程-获取单个流程列表】获取详细信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
    /// 异动唯一标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_key: Option<String>,
    /// 操作发起人 ID，为空时默认系统发起人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator_id: Option<String>,
}

/// CreateResponse（官方 data 节点内容，类型名 job_change）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 异动记录 id，可通过【搜索异动信息】接口获取详细信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_change_id: Option<String>,
    /// 雇员 id，可通过【搜索员工信息】接口获取
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 异动状态：0 = 审批中；1 = 审批通过；2 = 已异动；3 = 已拒绝；4 = 已撤销；5 = 无需审批
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 异动类型唯一标识，可通过【获取异动类型列表】接口获取详细信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_type_unique_identifier: Option<String>,
    /// 异动原因唯一标识，可通过【获取异动原因列表】接口获取详细信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_reason_unique_identifier: Option<String>,
    /// 异动发起后审批流程 id，可通过【流程-获取单个流程列表】获取详细信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    /// 异动生效日期，格式 `YYYY-MM-DD`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// 创建时间，格式 `YYYY-MM-DD`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// 异动详细信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_info: Option<TransferInfo>,
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

/// 异动详细信息
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct TransferInfo {
    /// 备注
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// offer 信息（数据仅会存储到系统，不支持页面上显示）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer_info: Option<String>,
    /// 是否撤销虚线上级
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_dotted_manager_clean: Option<bool>,
    /// 是否有试用期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_exist: Option<bool>,
    /// 原部门，可通过【批量查询部门】接口获取
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_department: Option<String>,
    /// 新部门，可通过【批量查询部门】接口获取
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_department: Option<String>,
    /// 原工作地点，可通过【批量查询地点】接口获取
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_work_location: Option<String>,
    /// 新工作地点，可通过【批量查询地点】接口获取
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_work_location: Option<String>,
    /// 原直属上级，可通过【搜索员工信息】接口获取
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_direct_manager: Option<String>,
    /// 新直属上级，可通过【搜索员工信息】接口获取
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_direct_manager: Option<String>,
    /// 原虚线上级，可通过【搜索员工信息】接口获取
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_dotted_manager: Option<String>,
    /// 新虚线上级，可通过【搜索员工信息】接口获取
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_dotted_manager: Option<String>,
    /// 原职务，可通过【批量查询职务】接口获取
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_job: Option<String>,
    /// 新职务，可通过【批量查询职务】接口获取
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_job: Option<String>,
    /// 原序列，可通过【批量查询序列】接口获取
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_job_family: Option<String>,
    /// 新序列，可通过【批量查询序列】接口获取
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_job_family: Option<String>,
    /// 原级别，可通过【批量查询职级】接口获取
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_job_level: Option<String>,
    /// 新职级，可通过【批量查询职级】接口获取
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_job_level: Option<String>,
    /// 原人员类型，可通过【批量查询人员类型】接口获取
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_workforce_type: Option<String>,
    /// 新人员类型，可通过【批量查询人员类型】接口获取
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_workforce_type: Option<String>,
    /// 原人员子类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_employee_subtype: Option<String>,
    /// 新人员子类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_employee_subtype: Option<String>,
    /// 原公司，详细信息可通过【批量查询公司】接口查询获得
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_company: Option<String>,
    /// 新公司，详细信息可通过【批量查询公司】接口查询获得
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_company: Option<String>,
    /// 原合同编号，可通过【批量查询合同】接口获取详细信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_contract_number: Option<String>,
    /// 新合同编号，可通过【批量查询合同】接口获取详细信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_contract_number: Option<String>,
    /// 原合同类型，可通过【批量查询合同】接口获取详细信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_contract_type: Option<String>,
    /// 新合同类型，可通过【批量查询合同】接口获取详细信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_contract_type: Option<String>,
    /// 原期限类型，可通过【批量查询合同】接口获取详细信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_duration_type: Option<String>,
    /// 新期限类型，可通过【批量查询合同】接口获取详细信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_duration_type: Option<String>,
    /// 原签订类型，可通过【批量查询合同】接口获取详细信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_signing_type: Option<String>,
    /// 新签订类型，可通过【批量查询合同】接口获取详细信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_signing_type: Option<String>,
    /// 原合同开始日期，格式 `YYYY-MM-DD`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_contract_start_date: Option<String>,
    /// 新合同开始日期，格式 `YYYY-MM-DD`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_contract_start_date: Option<String>,
    /// 原合同结束日期，格式 `YYYY-MM-DD`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_contract_end_date: Option<String>,
    /// 新合同结束日期，格式 `YYYY-MM-DD`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_contract_end_date: Option<String>,
    /// 原工时制度，可通过【批量查询工时制度】接口获取
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_working_hours_type: Option<String>,
    /// 新工时制度，可通过【批量查询工时制度】接口获取
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_working_hours_type: Option<String>,
    /// 原工作日历
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_working_calendar: Option<String>,
    /// 新工作日历
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_working_calendar: Option<String>,
    /// 原试用期预计结束日期，格式 `YYYY-MM-DD`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_probation_end_date: Option<String>,
    /// 新试用期预计结束日期，格式 `YYYY-MM-DD`
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
    /// 原成本中心分摊信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_cost_center_rate: Option<Vec<SupportCostCenterItem>>,
    /// 新成本中心分摊信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_cost_center_rate: Option<Vec<SupportCostCenterItem>>,
    /// 原工作信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_employment_change: Option<TransferEmploymentInfo>,
    /// 新工作信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_employment_change: Option<TransferEmploymentInfo>,
    /// 原职等，可通过【查询职等】接口获取
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_job_grade: Option<String>,
    /// 新职等，可通过【查询职等】接口获取
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_job_grade: Option<String>,
    /// 原薪资类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_compensation_type: Option<String>,
    /// 新薪资类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_compensation_type: Option<String>,
    /// 原任职公司，详细信息可通过【批量查询公司】接口查询获得
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_service_company: Option<String>,
    /// 新任职公司，详细信息可通过【批量查询公司】接口查询获得
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_service_company: Option<String>,
    /// 原岗位
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_position: Option<String>,
    /// 新岗位
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_position: Option<String>,
}

/// 成本中心分摊信息：`{cost_center_id, rate}`
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct SupportCostCenterItem {
    /// 支持的成本中心 id，详细信息可通过【搜索成本中心信息】接口查询获得
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    /// 分摊比例
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<i32>,
}

/// 异动工作信息
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct TransferEmploymentInfo {
    /// 转正式员工日期，格式 `YYYY-MM-DD`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regular_employee_start_date: Option<String>,
    /// 司龄起算日期，格式 `YYYY-MM-DD`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seniority_date: Option<String>,
    /// 员工编号，可通过【搜索员工信息】接口获取
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_number: Option<String>,
    /// 自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldData>>,
}

/// 自定义字段数据
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct CustomFieldData {
    /// 自定义字段 apiname，即自定义字段的唯一标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_api_name: Option<String>,
    /// 自定义字段名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<CustomFieldName>,
    /// 自定义字段类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    /// 字段值（json 转义后的字符串，根据元数据定义不同格式不同）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 自定义字段名称：`{zh_cn, en_us}`
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct CustomFieldName {
    /// 中文
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 英文
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

impl ApiResponseTrait for CreateResponse {
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

    /// 端到端：POST /open-apis/corehr/v1/job_changes
    #[tokio::test]
    async fn test_create_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v1/job_changes"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "job_change_id": "6991776076699549697",
                    "employment_id": "ou_a294793e8fa21529f2a60e3e9de45520",
                    "status": 4,
                    "transfer_type_unique_identifier": "reporting_line_change",
                    "transfer_reason_unique_identifier": "voluntary_transfer",
                    "process_id": "proc_1",
                    "effective_date": "2022-04-08",
                    "created_time": "2022-04-08",
                    "transfer_info": {
                        "remark": "备注",
                        "target_dotted_manager_clean": true,
                        "probation_exist": false,
                        "original_department": "6966236933198579208",
                        "target_department": "6966236933198579209",
                        "original_work_location": "6967271100992587295",
                        "target_work_location": "6967271100992587296",
                        "original_cost_center_rate": [
                            { "cost_center_id": "cc_1", "rate": 100 }
                        ],
                        "target_employment_change": {
                            "regular_employee_start_date": "2022-04-08",
                            "seniority_date": "2022-04-08",
                            "employee_number": "e_1",
                            "custom_fields": [
                                {
                                    "custom_api_name": "cf_1",
                                    "name": { "zh_cn": "字段", "en_us": "Field" },
                                    "type": 1,
                                    "value": "123"
                                }
                            ]
                        }
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

        let resp = CreateRequest::new(config)
            .transfer_mode(2)
            .employment_id("ou_a294793e8fa21529f2a60e3e9de45520".to_string())
            .transfer_type_unique_identifier("reporting_line_change".to_string())
            .effective_date("2022-04-08".to_string())
            .transfer_info(TransferInfo {
                remark: Some("备注".to_string()),
                target_department: Some("6966236933198579209".to_string()),
                ..Default::default()
            })
            .initiator_id("6982509313466189341".to_string())
            .execute()
            .await
            .expect("请求应成功");

        assert_eq!(resp.job_change_id, Some("6991776076699549697".to_string()));
        assert_eq!(
            resp.employment_id,
            Some("ou_a294793e8fa21529f2a60e3e9de45520".to_string())
        );
        assert_eq!(resp.status, Some(4));
        let transfer_info = resp.transfer_info.as_ref().expect("transfer_info 不能为空");
        assert_eq!(transfer_info.target_dotted_manager_clean, Some(true));
        assert_eq!(transfer_info.probation_exist, Some(false));
        assert_eq!(
            transfer_info.original_department,
            Some("6966236933198579208".to_string())
        );
        assert_eq!(
            transfer_info.target_department,
            Some("6966236933198579209".to_string())
        );
        assert_eq!(
            transfer_info.original_work_location,
            Some("6967271100992587295".to_string())
        );
        let cost_center_rate = transfer_info
            .original_cost_center_rate
            .as_deref()
            .expect("original_cost_center_rate 不能为空");
        assert_eq!(cost_center_rate[0].cost_center_id, Some("cc_1".to_string()));
        assert_eq!(cost_center_rate[0].rate, Some(100));
        let employment_change = transfer_info
            .target_employment_change
            .as_ref()
            .expect("target_employment_change 不能为空");
        assert_eq!(employment_change.employee_number, Some("e_1".to_string()));
        let custom_fields = employment_change
            .custom_fields
            .as_deref()
            .expect("custom_fields 不能为空");
        assert_eq!(custom_fields[0].custom_api_name, Some("cf_1".to_string()));
        assert_eq!(custom_fields[0].r#type, Some(1));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        // 请求体须与官方文档一致：transfer_mode/employment_id/异动类型/生效日期/transfer_info
        let sent: Value = received[0].body_json().expect("请求体应为 JSON");
        assert_eq!(
            sent,
            json!({
                "transfer_mode": 2,
                "employment_id": "ou_a294793e8fa21529f2a60e3e9de45520",
                "transfer_type_unique_identifier": "reporting_line_change",
                "effective_date": "2022-04-08",
                "transfer_info": {
                    "remark": "备注",
                    "target_department": "6966236933198579209"
                },
                "initiator_id": "6982509313466189341"
            })
        );
    }

    /// 必填校验：缺必填字段 / 非法 transfer_mode 快速失败
    #[tokio::test]
    async fn test_create_missing_required_fields_rejected() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let err = CreateRequest::new(config.clone())
            .execute()
            .await
            .expect_err("缺必填字段应被拒绝");
        assert!(err.to_string().contains("employment_id"), "实际: {err}");

        let err = CreateRequest::new(config)
            .transfer_mode(9)
            .employment_id("ou_x".to_string())
            .transfer_type_unique_identifier("t".to_string())
            .effective_date("2022-04-08".to_string())
            .execute()
            .await
            .expect_err("非法 transfer_mode 应被拒绝");
        assert!(err.to_string().contains("transfer_mode"), "实际: {err}");
    }
}
