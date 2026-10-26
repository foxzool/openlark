//! 发起员工异动
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/job_change/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 发起员工异动请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateBody {
    /// 异动方式（必填）：1：直接异动（无审批）；2：正常异动（完整流程）
    pub transfer_mode: i32,
    /// 雇员 ID（必填，ID 类型与查询参数 user_id_type 的取值一致）
    pub employment_id: String,
    /// 异动类型唯一标识（必填）
    pub transfer_type_unique_identifier: String,
    /// 生效日期（必填，格式："YYYY-MM-DD"）
    pub effective_date: String,
    /// 异动详细信息（必填，内部字段如不传，代表对应数据无异动）
    pub transfer_info: CreateTransferInfo,
    /// 关联流程唯一标识符（transfer_mode 为 2 时必填）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
    /// 异动记录标识符，发起失败可以重新用此标志继续请求
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_key: Option<String>,
    /// 异动发起人 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initiator_id: Option<String>,
    /// 异动原因唯一标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_reason_unique_identifier: Option<String>,
}

/// 发起员工异动的异动详细信息（官方 create_transfer_info 结构）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateTransferInfo {
    /// 备注
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// offer 信息（json 转义后的字符串）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer_info: Option<String>,
    /// 是否撤销虚线上级，当值为 true 时，target_dotted_manager 值为空
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_dotted_manager_clean: Option<bool>,
    /// 是否有试用期，当值为 true 时，target_probation_end_date 值不应为空
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_exist: Option<bool>,
    /// 新部门 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_department: Option<String>,
    /// 新工作地点
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_work_location: Option<String>,
    /// 新直属上级
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_direct_manager: Option<String>,
    /// 新虚线上级
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_dotted_manager: Option<String>,
    /// 新职务
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_job: Option<String>,
    /// 新序列 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_job_family: Option<String>,
    /// 新职级 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_job_level: Option<String>,
    /// 新人员类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_workforce_type: Option<String>,
    /// 新人员子类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_employee_subtype: Option<String>,
    /// 新公司
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_company: Option<String>,
    /// 新合同编号
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_contract_number: Option<String>,
    /// 新合同类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_contract_type: Option<String>,
    /// 新期限类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_duration_type: Option<String>,
    /// 新签订类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_signing_type: Option<String>,
    /// 新合同开始日期（YYYY-MM-DD）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_contract_start_date: Option<String>,
    /// 新合同结束日期（YYYY-MM-DD）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_contract_end_date: Option<String>,
    /// 新工时制度
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_working_hours_type: Option<String>,
    /// 新工作日历
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_working_calendar: Option<String>,
    /// 新试用期预计结束日期（YYYY-MM-DD）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_probation_end_date: Option<String>,
    /// 新周工作时长（取值范围 1-168，单位是小时）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_weekly_working_hours: Option<String>,
    /// 新排班
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_work_shift: Option<String>,
    /// 新成本中心分摊方式（0～100 条）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_cost_center_rates: Option<Vec<CreateCostCenterRate>>,
    /// 新工作信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_employment_change: Option<CreateTransferEmploymentInfo>,
    /// 新职等
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_job_grade: Option<String>,
    /// 新薪资类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_compensation_type: Option<String>,
    /// 新任职公司
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_service_company: Option<String>,
    /// 新岗位
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_position: Option<String>,
    /// 新社保城市
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_social_security_city: Option<String>,
    /// 新通道
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_pathway: Option<String>,
    /// 编制随人员一起调整
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_transfer_with_workforce: Option<bool>,
}

/// 发起员工异动的成本中心分摊方式（官方 job_data_cost_center 结构）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateCostCenterRate {
    /// 成本中心 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    /// 分摊比例（整数）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<i32>,
}

/// 发起员工异动的工作信息（官方 tranfer_employment_info 结构）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateTransferEmploymentInfo {
    /// 转正式员工日期（YYYY-MM-DD）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regular_employee_start_date: Option<String>,
    /// 司龄起算日期（YYYY-MM-DD）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seniority_date: Option<String>,
    /// 员工编号
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_number: Option<String>,
    /// 自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CreateCustomFieldData>>,
}

/// 发起员工异动的自定义字段（官方 custom_field_data 结构，custom_api_name 与 value 必填）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateCustomFieldData {
    /// 自定义字段 apiname，即自定义字段的唯一标识（必填）
    pub custom_api_name: String,
    /// 字段值，是 json 转义后的字符串（必填）
    pub value: String,
}

/// CreateRequest
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 配置信息
    config: Config,
    /// 请求体
    body: CreateBody,
    /// 用户 ID 类型（open_id/union_id/user_id/people_admin_id/people_corehr_id）
    user_id_type: Option<String>,
    /// 部门 ID 类型（open_department_id/department_id/people_corehr_department_id）
    department_id_type: Option<String>,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: CreateBody::default(),
            user_id_type: None,
            department_id_type: None,
        }
    }

    /// 设置请求体
    pub fn body(mut self, body: CreateBody) -> Self {
        self.body = body;
        self
    }

    /// 设置异动方式（必填）：1：直接异动（无审批）；2：正常异动（完整流程）。
    pub fn transfer_mode(mut self, transfer_mode: i32) -> Self {
        self.body.transfer_mode = transfer_mode;
        self
    }

    /// 设置雇员 ID（必填）。
    pub fn employment_id(mut self, employment_id: impl Into<String>) -> Self {
        self.body.employment_id = employment_id.into();
        self
    }

    /// 设置异动类型唯一标识（必填）。
    pub fn transfer_type_unique_identifier(
        mut self,
        transfer_type_unique_identifier: impl Into<String>,
    ) -> Self {
        self.body.transfer_type_unique_identifier = transfer_type_unique_identifier.into();
        self
    }

    /// 设置生效日期（必填，格式："YYYY-MM-DD"）。
    pub fn effective_date(mut self, effective_date: impl Into<String>) -> Self {
        self.body.effective_date = effective_date.into();
        self
    }

    /// 设置异动详细信息（必填）。
    pub fn transfer_info(mut self, transfer_info: CreateTransferInfo) -> Self {
        self.body.transfer_info = transfer_info;
        self
    }

    /// 设置关联流程唯一标识符（transfer_mode 为 2 时必填）。
    pub fn flow_id(mut self, flow_id: impl Into<String>) -> Self {
        self.body.flow_id = Some(flow_id.into());
        self
    }

    /// 设置异动记录标识符，发起失败可以重新用此标志继续请求。
    pub fn transfer_key(mut self, transfer_key: impl Into<String>) -> Self {
        self.body.transfer_key = Some(transfer_key.into());
        self
    }

    /// 设置异动发起人 ID。
    pub fn initiator_id(mut self, initiator_id: impl Into<String>) -> Self {
        self.body.initiator_id = Some(initiator_id.into());
        self
    }

    /// 设置异动原因唯一标识。
    pub fn transfer_reason_unique_identifier(
        mut self,
        transfer_reason_unique_identifier: impl Into<String>,
    ) -> Self {
        self.body.transfer_reason_unique_identifier =
            Some(transfer_reason_unique_identifier.into());
        self
    }

    /// 设置用户 ID 类型（open_id/union_id/user_id/people_admin_id/people_corehr_id）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置部门 ID 类型（open_department_id/department_id/people_corehr_department_id）。
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.department_id_type = Some(department_id_type.into());
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
        validate_required!(self.body.employment_id, "employment_id 不能为空");
        validate_required!(
            self.body.transfer_type_unique_identifier,
            "transfer_type_unique_identifier 不能为空"
        );
        validate_required!(self.body.effective_date, "effective_date 不能为空");
        if !(1..=2).contains(&self.body.transfer_mode) {
            return Err(openlark_core::error::CoreError::validation_msg(
                "transfer_mode 取值范围为 1～2",
            ));
        }
        let transfer_info = &self.body.transfer_info;
        for cost_center_rate in transfer_info.target_cost_center_rates.iter().flatten() {
            validate_required!(
                cost_center_rate
                    .cost_center_id
                    .as_deref()
                    .unwrap_or_default(),
                "target_cost_center_rates.cost_center_id 不能为空"
            );
        }
        for custom_field in transfer_info
            .target_employment_change
            .as_ref()
            .and_then(|change| change.custom_fields.as_deref())
            .unwrap_or_default()
        {
            validate_required!(
                custom_field.custom_api_name,
                "custom_fields.custom_api_name 不能为空"
            );
            validate_required!(custom_field.value, "custom_fields.value 不能为空");
        }

        let request = ApiRequest::<CreateResponse>::post("/open-apis/corehr/v2/job_changes")
            .query_opt("user_id_type", self.user_id_type)
            .query_opt("department_id_type", self.department_id_type)
            .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// CreateResponse
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 异动记录 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_change_id: Option<String>,
    /// 雇员 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 异动状态：Approving（审批中）/ Approved（审批通过）/ Transformed（已异动）/ Rejected（已拒绝）/ Cancelled（已撤销）/ NoNeedApproval（无需审批）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 异动类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_type_unique_identifier: Option<String>,
    /// 异动原因
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_reason_unique_identifier: Option<String>,
    /// 异动流程 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    /// 生效时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// 创建时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// 异动详细信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_info: Option<TransferInfo>,
    /// 是否调整薪酬
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_adjust_salary: Option<bool>,
    /// 异动自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldData>>,
}

/// 异动详细信息（官方 transfer_info 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransferInfo {
    /// 备注
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// offer 信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer_info: Option<String>,
    /// 是否撤销虚线上级，当值为 true 时，target_dotted_manager 值为空
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_dotted_manager_clean: Option<bool>,
    /// 是否有试用期，当值为 true 时，target_probation_end_date 值不应为空
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
    /// 原级别
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
    /// 原合同开始日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_contract_start_date: Option<String>,
    /// 新合同开始日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_contract_start_date: Option<String>,
    /// 原合同结束日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_contract_end_date: Option<String>,
    /// 新合同结束日期
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
    /// 原试用期预计结束日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_probation_end_date: Option<String>,
    /// 新试用期预计结束日期
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
}

/// 成本中心分摊方式（官方 job_data_cost_center 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobDataCostCenter {
    /// 成本中心 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    /// 分摊比例（整数）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<i32>,
}

/// 异动工作信息（官方 tranfer_employment_info 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransferEmploymentInfo {
    /// 转正式员工日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regular_employee_start_date: Option<String>,
    /// 司龄起算日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seniority_date: Option<String>,
    /// 员工编号
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_number: Option<String>,
    /// 自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldData>>,
}

/// 自定义字段（官方 custom_field_data 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomFieldData {
    /// 自定义字段 apiname，即自定义字段的唯一标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_api_name: Option<String>,
    /// 自定义字段名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<CustomFieldName>,
    /// 自定义字段类型
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i64>,
    /// 字段值，是 json 转义后的字符串
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 自定义字段名称：{zh_cn, en_us}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/job_changes
    #[tokio::test]
    async fn test_create_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/job_changes"))
            .and(body_json(json!({
                "transfer_mode": 2,
                "employment_id": "ou_a294793e8fa21529f2a60e3e9de45520",
                "transfer_type_unique_identifier": "internal_transfer",
                "flow_id": "people_6963913041981490725_6983885526583627531",
                "effective_date": "2022-03-01",
                "transfer_info": {
                    "remark": "异动详情",
                    "target_department": "6966236933198579208",
                    "probation_exist": true,
                    "target_cost_center_rates": [
                        { "cost_center_id": "6950635856373745165", "rate": 100 }
                    ],
                    "target_employment_change": {
                        "regular_employee_start_date": "2023-01-01",
                        "seniority_date": "2023-01-01",
                        "employee_number": "1111111",
                        "custom_fields": [
                            { "custom_api_name": "name", "value": "\"231\"" }
                        ]
                    },
                    "is_transfer_with_workforce": false
                },
                "transfer_key": "transfer_3627531",
                "initiator_id": "ou_a294793e8fa21529f2a60e3e9de45520",
                "transfer_reason_unique_identifier": "involuntary_transfer"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "job_change_id": "6991776076699549697",
                    "employment_id": "ou_a294793e8fa21529f2a60e3e9de45520",
                    "status": "Approved",
                    "transfer_type_unique_identifier": "direct_leader_change",
                    "transfer_reason_unique_identifier": "involuntary_transfer",
                    "process_id": "6991776078461142564",
                    "effective_date": "2022-03-01",
                    "created_time": "1627899724000",
                    "transfer_info": {
                        "remark": "异动详情",
                        "offer_info": "优质人才，加急处理",
                        "target_dotted_manager_clean": false,
                        "probation_exist": true,
                        "original_department": "6966236933198579208",
                        "target_department": "6966236933198579208",
                        "original_cost_center_rate": [
                            { "cost_center_id": "6950635856373745165", "rate": 100 }
                        ],
                        "target_employment_change": {
                            "regular_employee_start_date": "2023-01-01",
                            "seniority_date": "2023-01-01",
                            "employee_number": "1111111",
                            "custom_fields": [
                                {
                                    "custom_api_name": "name",
                                    "name": { "zh_cn": "自定义姓名", "en_us": "Custom Name" },
                                    "type": 1,
                                    "value": "\"231\""
                                }
                            ]
                        },
                        "is_transfer_with_workforce": false
                    },
                    "is_adjust_salary": true,
                    "custom_fields": [
                        {
                            "custom_api_name": "name",
                            "name": { "zh_cn": "自定义姓名", "en_us": "Custom Name" },
                            "type": 1,
                            "value": "\"231\""
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

        let resp = CreateRequest::new(config)
            .transfer_mode(2)
            .employment_id("ou_a294793e8fa21529f2a60e3e9de45520")
            .transfer_type_unique_identifier("internal_transfer")
            .flow_id("people_6963913041981490725_6983885526583627531")
            .effective_date("2022-03-01")
            .transfer_info(CreateTransferInfo {
                remark: Some("异动详情".to_string()),
                target_department: Some("6966236933198579208".to_string()),
                probation_exist: Some(true),
                target_cost_center_rates: Some(vec![CreateCostCenterRate {
                    cost_center_id: Some("6950635856373745165".to_string()),
                    rate: Some(100),
                }]),
                target_employment_change: Some(CreateTransferEmploymentInfo {
                    regular_employee_start_date: Some("2023-01-01".to_string()),
                    seniority_date: Some("2023-01-01".to_string()),
                    employee_number: Some("1111111".to_string()),
                    custom_fields: Some(vec![CreateCustomFieldData {
                        custom_api_name: "name".to_string(),
                        value: "\"231\"".to_string(),
                    }]),
                }),
                is_transfer_with_workforce: Some(false),
                ..Default::default()
            })
            .transfer_key("transfer_3627531")
            .initiator_id("ou_a294793e8fa21529f2a60e3e9de45520")
            .transfer_reason_unique_identifier("involuntary_transfer")
            .execute()
            .await
            .expect("请求应成功");

        assert_eq!(resp.job_change_id, Some("6991776076699549697".to_string()));
        assert_eq!(resp.status, Some("Approved".to_string()));
        assert_eq!(
            resp.transfer_type_unique_identifier,
            Some("direct_leader_change".to_string())
        );
        assert_eq!(
            resp.transfer_reason_unique_identifier,
            Some("involuntary_transfer".to_string())
        );
        assert_eq!(resp.is_adjust_salary, Some(true));
        let transfer_info = resp.transfer_info.expect("transfer_info 应存在");
        assert_eq!(transfer_info.probation_exist, Some(true));
        assert_eq!(
            transfer_info.original_department,
            Some("6966236933198579208".to_string())
        );
        assert_eq!(
            transfer_info.target_department,
            Some("6966236933198579208".to_string())
        );
        assert_eq!(
            transfer_info.original_cost_center_rate,
            Some(vec![JobDataCostCenter {
                cost_center_id: Some("6950635856373745165".to_string()),
                rate: Some(100),
            }])
        );
        let target_employment_change = transfer_info
            .target_employment_change
            .expect("target_employment_change 应存在");
        assert_eq!(
            target_employment_change.employee_number,
            Some("1111111".to_string())
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
