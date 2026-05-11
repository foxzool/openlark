//! HR API端点定义（类型安全枚举系统）
//!
//! 本模块提供基于枚举的 API 端点定义，用于生产代码中的类型安全调用。
//!
//! # 使用场景
//!
//! ## 生产代码（推荐）
//! 使用枚举端点获得编译时类型检查和动态 URL 生成能力：
//! ```rust
//! use openlark_hr::common::api_endpoints::AttendanceApiV1;
//!
//! let endpoint = AttendanceApiV1::GroupList;
//! let url = endpoint.to_url(); // 类型安全，动态生成
//! assert!(url.contains("/open-apis/attendance/v1/"));
//! ```
//!
//! # 特性
//! - ✅ **类型安全**：编译时验证参数
//! - ✅ **动态生成**：支持参数化 URL
//! - ✅ **易于维护**：集中管理端点定义
//! - ✅ **避免错误**：消除字符串拼接错误
//!
//! # 业务域
//!
//! | 枚举 | 业务域 | API 数量 |
//! |------|--------|---------|
//! | AttendanceApiV1 | 考勤管理 | 39 |
//! | HireApiV1/V2 | 招聘管理 | 182 |
//! | CompensationApiV1 | 薪酬管理 | 21 |
//! | PerformanceApiV1 | 绩效管理 | 21 |
//! | PayrollApiV1 | 工资管理 | 12 |
//! | OkrApiV1 | 目标管理 | 12 |
//! | EhrApiV1 | EHR 功能 | 2 |
//! | FeishuPeopleApiV1/V2 | 核心人力资源 | 257 |
//!
//! # URL 模式
//!
//! 所有端点遵循 `/open-apis/{project}/v{version}/{resource}/{action}` 模式：
//! - project: 业务项目名称（如 attendance, hire）
//! - version: API 版本（v1, v2）
//! - resource: 资源名称（如 group, shift, talent）
//! - action: 操作名称（如 create, get, list, update, delete）

// Attendance API V1 - 考勤管理 (39 APIs)

/// Attendance API V1 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum AttendanceApiV1 {
    // === user_task 资源 (3个) ===
    /// 查询打卡结果
    UserTaskQuery,
    /// 创建补卡申请
    UserTaskRemedyCreate,
    /// 查询补卡申请
    UserTaskRemedyQuery,
    /// 查询用户可补卡时间段
    UserTaskRemedyQueryUserAllowedRemedys,

    // === group 资源 (6个) ===
    /// 创建考勤组
    GroupCreate,
    /// 删除考勤组
    GroupDelete(String), // group_id
    /// 按 ID 查询考勤组
    GroupGet(String), // group_id
    /// 查询所有考勤组
    GroupList,
    /// 查询考勤组下所有成员
    GroupListUser(String), // group_id
    /// 按名称查询考勤组
    GroupSearch,

    // === shift 资源 (5个) ===
    /// 创建班次
    ShiftCreate,
    /// 删除班次
    ShiftDelete(String), // shift_id
    /// 按 ID 查询班次
    ShiftGet(String), // shift_id
    /// 查询所有班次
    ShiftList,
    /// 按名称查询班次
    ShiftQuery,

    // === user_flow 资源 (4个) ===
    /// 导入打卡流水
    UserFlowBatchCreate,
    /// 删除打卡流水
    UserFlowBatchDel,
    /// 获取打卡流水
    UserFlowGet,
    /// 批量查询打卡流水
    UserFlowQuery,

    // === user_approval 资源 (2个) ===
    /// 写入审批结果
    UserApprovalCreate,
    /// 获取审批数据
    UserApprovalQuery,

    // === user_daily_shift 资源 (3个) ===
    /// 创建或修改排班表
    UserDailyShiftBatchCreate,
    /// 创建或修改临时排班
    UserDailyShiftBatchCreateTemp,
    /// 查询排班表
    UserDailyShiftQuery,

    // === user_setting 资源 (2个) ===
    /// 修改用户人脸识别信息
    UserSettingModify,
    /// 获取用户设置
    UserSettingQuery,

    // === file 资源 (2个) ===
    /// 上传用户人脸识别照片
    FileUpload,
    /// 下载用户人脸识别照片
    FileDownload,

    // === archive_rule 资源 (4个) ===
    /// 查询所有归档规则
    ArchiveRuleList,
    /// 查询归档报表表头
    ArchiveRuleUserStatsFieldsQuery,
    /// 删除归档报表行数据
    ArchiveRuleDelReport,
    /// 写入归档报表结果
    ArchiveRuleUploadReport,

    // === leave_accrual_record 资源 (1个) ===
    /// 修改发放记录
    LeaveAccrualRecordPatch,

    // === leave_employ_expire_record 资源 (1个) ===
    /// 通过过期时间获取发放记录
    LeaveEmployExpireRecordGet,

    // === user_stats_data 资源 (1个) ===
    /// 查询用户统计数据
    UserStatsDataQuery,

    // === user_stats_field 资源 (1个) ===
    /// 查询用户统计字段
    UserStatsFieldQuery,

    // === user_stats_view 资源 (2个) ===
    /// 查询用户统计视图
    UserStatsViewQuery,
    /// 更新用户统计视图
    UserStatsViewUpdate,

    // === approval_info 资源 (1个) ===
    /// 通知审批状态更新
    ApprovalInfoProcess,
}

impl AttendanceApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            // user_task
            AttendanceApiV1::UserTaskQuery => {
                "/open-apis/attendance/v1/user_tasks/query".to_string()
            }
            AttendanceApiV1::UserTaskRemedyCreate => {
                "/open-apis/attendance/v1/user_task_remedys".to_string()
            }
            AttendanceApiV1::UserTaskRemedyQuery => {
                "/open-apis/attendance/v1/user_task_remedys/query".to_string()
            }
            AttendanceApiV1::UserTaskRemedyQueryUserAllowedRemedys => {
                "/open-apis/attendance/v1/user_task_remedys/query_user_allowed_remedys".to_string()
            }

            // group
            AttendanceApiV1::GroupCreate => "/open-apis/attendance/v1/groups".to_string(),
            AttendanceApiV1::GroupDelete(group_id) => {
                format!("/open-apis/attendance/v1/groups/{group_id}")
            }
            AttendanceApiV1::GroupGet(group_id) => {
                format!("/open-apis/attendance/v1/groups/{group_id}")
            }
            AttendanceApiV1::GroupList => "/open-apis/attendance/v1/groups".to_string(),
            AttendanceApiV1::GroupListUser(group_id) => {
                format!("/open-apis/attendance/v1/groups/{group_id}/list_user")
            }
            AttendanceApiV1::GroupSearch => "/open-apis/attendance/v1/groups/search".to_string(),

            // shift
            AttendanceApiV1::ShiftCreate => "/open-apis/attendance/v1/shifts".to_string(),
            AttendanceApiV1::ShiftDelete(shift_id) => {
                format!("/open-apis/attendance/v1/shifts/{shift_id}")
            }
            AttendanceApiV1::ShiftGet(shift_id) => {
                format!("/open-apis/attendance/v1/shifts/{shift_id}")
            }
            AttendanceApiV1::ShiftList => "/open-apis/attendance/v1/shifts".to_string(),
            AttendanceApiV1::ShiftQuery => "/open-apis/attendance/v1/shifts/query".to_string(),

            // user_flow
            AttendanceApiV1::UserFlowBatchCreate => {
                "/open-apis/attendance/v1/user_flows/batch_create".to_string()
            }
            AttendanceApiV1::UserFlowBatchDel => {
                "/open-apis/attendance/v1/user_flows/batch_del".to_string()
            }
            AttendanceApiV1::UserFlowGet => "/open-apis/attendance/v1/user_flows/{}".to_string(),
            AttendanceApiV1::UserFlowQuery => {
                "/open-apis/attendance/v1/user_flows/query".to_string()
            }

            // user_approval
            AttendanceApiV1::UserApprovalCreate => {
                "/open-apis/attendance/v1/user_approvals".to_string()
            }
            AttendanceApiV1::UserApprovalQuery => {
                "/open-apis/attendance/v1/user_approvals/query".to_string()
            }

            // user_daily_shift
            AttendanceApiV1::UserDailyShiftBatchCreate => {
                "/open-apis/attendance/v1/user_daily_shifts/batch_create".to_string()
            }
            AttendanceApiV1::UserDailyShiftBatchCreateTemp => {
                "/open-apis/attendance/v1/user_daily_shifts/batch_create_temp".to_string()
            }
            AttendanceApiV1::UserDailyShiftQuery => {
                "/open-apis/attendance/v1/user_daily_shifts/query".to_string()
            }

            // user_setting
            AttendanceApiV1::UserSettingModify => {
                "/open-apis/attendance/v1/user_settings/modify".to_string()
            }
            AttendanceApiV1::UserSettingQuery => {
                "/open-apis/attendance/v1/user_settings/query".to_string()
            }

            // file
            AttendanceApiV1::FileUpload => "/open-apis/attendance/v1/files/upload".to_string(),
            AttendanceApiV1::FileDownload => {
                "/open-apis/attendance/v1/files/{}/download".to_string()
            }

            // archive_rule
            AttendanceApiV1::ArchiveRuleList => "/open-apis/attendance/v1/archive_rule".to_string(),
            AttendanceApiV1::ArchiveRuleUserStatsFieldsQuery => {
                "/open-apis/attendance/v1/archive_rule/user_stats_fields_query".to_string()
            }
            AttendanceApiV1::ArchiveRuleDelReport => {
                "/open-apis/attendance/v1/archive_rule/del_report".to_string()
            }
            AttendanceApiV1::ArchiveRuleUploadReport => {
                "/open-apis/attendance/v1/archive_rule/upload_report".to_string()
            }

            // leave_accrual_record
            AttendanceApiV1::LeaveAccrualRecordPatch => {
                "/open-apis/attendance/v1/leave_accrual_record/{}".to_string()
            }

            // leave_employ_expire_record
            AttendanceApiV1::LeaveEmployExpireRecordGet => {
                "/open-apis/attendance/v1/leave_employ_expire_records/{}".to_string()
            }

            // user_stats_data
            AttendanceApiV1::UserStatsDataQuery => {
                "/open-apis/attendance/v1/user_stats_datas/query".to_string()
            }

            // user_stats_field
            AttendanceApiV1::UserStatsFieldQuery => {
                "/open-apis/attendance/v1/user_stats_fields/query".to_string()
            }

            // user_stats_view
            AttendanceApiV1::UserStatsViewQuery => {
                "/open-apis/attendance/v1/user_stats_views/query".to_string()
            }
            AttendanceApiV1::UserStatsViewUpdate => {
                "/open-apis/attendance/v1/user_stats_views/{}".to_string()
            }

            // approval_info
            AttendanceApiV1::ApprovalInfoProcess => {
                "/open-apis/attendance/v1/approval_infos/process".to_string()
            }
        }
    }
}

// Hire API V1/V2 - 招聘管理 (182 APIs)

/// Hire API V1 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum HireApiV1 {
    // === talent 资源 (8个) ===
    /// 创建候选人
    TalentCombinedCreate,
    /// 更新候选人
    TalentCombinedUpdate,
    /// 获取候选人信息
    TalentGet(String), // talent_id
    /// 获取候选人列表
    TalentList,
    /// 批量获取人才ID
    TalentBatchGetId,
    /// 将人才加入指定文件夹
    TalentAddToFolder(String), // talent_id
    /// 将人才从指定文件夹移除
    TalentRemoveToFolder(String), // talent_id
    /// 获取候选人外部信息
    TalentExternalInfoCreate(String), // talent_id
    /// 更新候选人外部信息
    TalentExternalInfoUpdate(String), // talent_id
    /// 操作人才标签
    TalentTag(String), // talent_id
    /// 查询人才字段
    TalentObjectQuery,
    /// 获取候选人入职状态
    TalentOnboardStatus(String), // talent_id

    // === talent_folder 资源 (1个) ===
    /// 获取人才文件夹列表
    TalentFolderList,

    // === talent_pool 资源 (3个) ===
    /// 搜索人才库
    TalentPoolSearch,
    /// 批量加入/移除人才库中人才
    TalentPoolBatchChangeTalentPool,
    /// 将人才加入人才库
    TalentPoolMoveTalent,

    // === talent_tag 资源 (1个) ===
    /// 获取人才标签信息列表
    TalentTagList,

    // === talent_blocklist 资源 (1个) ===
    /// 操作候选人拉黑
    TalentBlocklistChangeTalentBlock,

    // === talent_operation_log 资源 (1个) ===
    /// 查询人才操作记录
    TalentOperationLogSearch,

    // === application 资源 (9个) ===
    /// 创建投递
    ApplicationCreate,
    /// 获取投递信息
    ApplicationGet(String), // application_id
    /// 获取投递列表
    ApplicationList,
    /// 获取投递详情
    ApplicationGetDetail(String), // application_id
    /// 更改投递阶段
    ApplicationTransferStage(String), // application_id
    /// 终止投递
    ApplicationTerminate(String), // application_id
    /// 恢复投递
    ApplicationRecover(String), // application_id
    /// 取消候选人入职
    ApplicationCancelOnboard(String), // application_id
    /// 转移投递阶段
    ApplicationTransferOnboard(String), // application_id

    // === interview 资源 (3个) ===
    /// 获取面试信息
    InterviewGetByTalent(String), // talent_id
    /// 获取面试记录列表
    InterviewList,
    /// 获取面试任务列表
    InterviewTaskList,

    // === interview_feedback_form 资源 (1个) ===
    /// 获取面试评价表列表
    InterviewFeedbackFormList,

    // === interview_record 资源 (3个) ===
    /// 获取面试记录
    InterviewRecordGet(String), // interview_record_id
    /// 获取面试记录列表
    InterviewRecordList,
    /// 获取面试记录附件
    InterviewRecordAttachmentGet(String), // interview_record_id

    // === interview_registration_schema 资源 (1个) ===
    /// 获取面试登记表列表
    InterviewRegistrationSchemaList,

    // === interview_round_type 资源 (1个) ===
    /// 获取面试轮次类型列表
    InterviewRoundTypeList,

    // === interviewer 资源 (2个) ===
    /// 获取面试官信息列表
    InterviewerList,
    /// 更新面试官信息
    InterviewerPatch(String), // interviewer_id

    // === employee 资源 (3个) ===
    /// 获取员工信息
    EmployeeGet(String), // employee_id
    /// 获取员工信息（通过投递ID）
    EmployeeGetByApplication(String), // application_id
    /// 更新雇佣信息
    EmployeePatch(String), // employee_id

    // === job 资源 (10个) ===
    /// 创建职位
    JobCombinedCreate,
    /// 更新职位
    JobCombinedUpdate,
    /// 获取职位信息
    JobGet(String), // job_id
    /// 获取职位列表
    JobList,
    /// 获取职位详情
    JobGetDetail(String), // job_id
    /// 关闭职位
    JobClose(String), // job_id
    /// 开启职位
    JobOpen(String), // job_id
    /// 更新职位配置
    JobUpdateConfig(String), // job_id
    /// 获取职位配置
    JobConfig(String), // job_id
    /// 批量更新职位相关人员
    JobManagerBatchUpdate,

    // === job_manager 资源 (2个) ===
    /// 获取职位上的招聘人员信息
    JobManagerGet(String), // job_id

    // === job_function 资源 (1个) ===
    /// 获取职能分类列表
    JobFunctionList,

    // === job_process 资源 (1个) ===
    /// 获取招聘流程信息
    JobProcessList,

    // === job_requirement 资源 (4个) ===
    /// 创建招聘需求
    JobRequirementCreate,
    /// 获取招聘需求列表
    JobRequirementList,
    /// 获取招聘需求信息
    JobRequirementListById,
    /// 更新招聘需求
    JobRequirementUpdate,
    /// 删除招聘需求
    JobRequirementDelete(String), // requirement_id

    // === job_requirement_schema 资源 (1个) ===
    /// 获取招聘需求模板列表
    JobRequirementSchemaList,

    // === job_schema 资源 (1个) ===
    /// 获取职位模板
    JobSchemaList,

    // === job_type 资源 (1个) ===
    /// 获取职位类别列表
    JobTypeList,

    // === job_publish_record 资源 (1个) ===
    /// 搜索职位广告发布记录
    JobPublishRecordSearch,

    // === attachment 资源 (3个) ===
    /// 创建附件
    AttachmentCreate,
    /// 获取附件
    AttachmentGet(String), // attachment_id
    /// 预览附件
    AttachmentPreview(String), // attachment_id

    // === note 资源 (5个) ===
    /// 创建备注
    NoteCreate,
    /// 删除备注
    NoteDelete(String), // note_id
    /// 获取备注
    NoteGet(String), // note_id
    /// 获取备注列表
    NoteList,
    /// 更新备注
    NotePatch(String), // note_id

    // === offer 资源 (5个) ===
    /// 创建 Offer
    OfferCreate,
    /// 获取 Offer 信息
    OfferGet(String), // offer_id
    /// 获取 Offer 列表
    OfferList,
    /// 更新 Offer 信息
    OfferUpdate(String), // offer_id
    /// 获取 Offer 状态
    OfferOfferStatus(String), // offer_id
    /// 获取实习 Offer 入/离职状态
    OfferInternOfferStatus(String), // offer_id

    // === offer_approval_template 资源 (1个) ===
    /// 获取 Offer 审批流列表
    OfferApprovalTemplateList,

    // === offer_application_form 资源 (2个) ===
    /// 获取 Offer 申请表信息
    OfferApplicationFormGet(String), // application_form_id
    /// 获取 Offer 申请表列表
    OfferApplicationFormList,

    // === offer_schema 资源 (1个) ===
    /// 获取 Offer 模板
    OfferSchemaGet(String), // schema_id

    // === offer_custom_field 资源 (1个) ===
    /// 更新 Offer 申请表自定义字段
    OfferCustomFieldUpdate,

    // === referral 资源 (2个) ===
    /// 获取内推信息
    ReferralGetByApplication(String), // application_id
    /// 搜索内推账户
    ReferralSearch,

    // === referral_account 资源 (6个) ===
    /// 创建内推账户
    ReferralAccountCreate,
    /// 启用内推账户
    ReferralAccountEnable(String), // account_id
    /// 停用内推账户
    ReferralAccountDeactivate(String), // account_id
    /// 获取内推账户提现数据对账
    ReferralAccountReconciliation,
    /// 提现内推账户余额
    ReferralAccountWithdraw,
    /// 获取账户资产
    ReferralAccountGetAccountAssets(String), // account_id

    // === referral_website 资源 (2个) ===
    /// 获取内推官网下职位广告列表
    ReferralWebsiteJobPostList,
    /// 获取内推官网下职位广告详情
    ReferralWebsiteJobPostGet(String), // job_post_id

    // === website 资源 (1个) ===
    /// 获取招聘官网列表
    WebsiteList,

    // === website_channel 资源 (4个) ===
    /// 创建推广渠道
    WebsiteChannelCreate,
    /// 获取推广渠道列表
    WebsiteChannelList,
    /// 更新推广渠道
    WebsiteChannelUpdate(String), // channel_id
    /// 删除推广渠道
    WebsiteChannelDelete(String), // channel_id

    // === website_delivery 资源 (2个) ===
    /// 根据简历附件创建投递
    WebsiteDeliveryCreateByAttachment,
    /// 根据简历创建投递
    WebsiteDeliveryCreateByResume,

    // === website_delivery_task 资源 (1个) ===
    /// 获取投递任务结果
    WebsiteDeliveryTaskGet,

    // === website_job_post 资源 (3个) ===
    /// 获取职位广告列表
    WebsiteJobPostList,
    /// 搜索职位广告
    WebsiteJobPostSearch,
    /// 获取职位广告详情
    WebsiteJobPostGet(String), // job_post_id

    // === website_site_user 资源 (1个) ===
    /// 创建招聘官网用户
    WebsiteSiteUserCreate,

    // === resume_source 资源 (1个) ===
    /// 获取简历来源列表
    ResumeSourceList,

    // === role 资源 (2个) ===
    /// 获取角色列表
    RoleList,
    /// 获取角色详情
    RoleGet(String), // role_id

    // === subject 资源 (1个) ===
    /// 获取科目列表
    SubjectList,

    // === questionnaire 资源 (1个) ===
    /// 获取问卷列表
    QuestionnaireList,

    // === portal_apply_schema 资源 (1个) ===
    /// 获取门户申请schema
    PortalApplySchemaList,

    // === registration_schema 资源 (1个) ===
    /// 获取登记表列表
    RegistrationSchemaList,

    // === termination_reason 资源 (1个) ===
    /// 获取终止投递原因
    TerminationReasonList,

    // === test 资源 (1个) ===
    /// 搜索测试
    TestSearch,

    // === todo 资源 (1个) ===
    /// 获取待办列表
    TodoList,

    // === agency 资源 (7个) ===
    /// 批量查询猎头供应商
    AgencyBatchQuery,
    /// 查询猎头供应商信息
    AgencyQuery,
    /// 获取猎头供应商信息
    AgencyGet(String), // agency_id
    /// 查询猎头供应商下猎头列表
    AgencyGetAgencyAccount(String), // agency_id
    /// 禁用/启用猎头
    AgencyOperateAgencyAccount(String), // agency_id
    /// 设置猎头保护期
    AgencyProtect(String), // agency_id
    /// 查询猎头保护期信息
    AgencyProtectSearch,

    // === background_check_order 资源 (2个) ===
    /// 批量查询背调订单
    BackgroundCheckOrderBatchQuery,
    /// 获取背调信息列表
    BackgroundCheckOrderList,

    // === eco_background_check 资源 (3个) ===
    /// 取消背调订单
    EcoBackgroundCheckCancel,
    /// 回传背调订单的最终结果
    EcoBackgroundCheckUpdateResult(String), // order_id
    /// 更新背调进度
    EcoBackgroundCheckUpdateProgress(String), // order_id

    // === eco_background_check_package 资源 (3个) ===
    /// 创建背调套餐和附加调查项
    EcoBackgroundCheckPackageCreate,
    /// 批量删除背调套餐和附加调查项
    EcoBackgroundCheckPackageBatchDelete,
    /// 批量更新背调套餐和附加调查项
    EcoBackgroundCheckPackageBatchUpdate,

    // === eco_background_check_custom_field 资源 (3个) ===
    /// 创建背调自定义字段
    EcoBackgroundCheckCustomFieldCreate,
    /// 批量删除背调自定义字段
    EcoBackgroundCheckCustomFieldBatchDelete,
    /// 批量更新背调自定义字段
    EcoBackgroundCheckCustomFieldBatchUpdate,

    // === eco_exam 资源 (2个) ===
    /// 获取笔试信息
    EcoExamLoginInfo(String), // exam_id
    /// 回传笔试结果
    EcoExamUpdateResult(String), // exam_id

    // === eco_exam_paper 资源 (3个) ===
    /// 创建试卷列表
    EcoExamPaperCreate,
    /// 批量删除试卷列表
    EcoExamPaperBatchDelete,
    /// 批量更新试卷列表
    EcoExamPaperBatchUpdate,

    // === eco_account_custom_field 资源 (3个) ===
    /// 创建内推账户自定义字段
    EcoAccountCustomFieldCreate,
    /// 批量删除内推账户自定义字段
    EcoAccountCustomFieldBatchDelete,
    /// 批量更新内推账户自定义字段
    EcoAccountCustomFieldBatchUpdate,

    // === exam 资源 (2个) ===
    /// 创建笔试
    ExamCreate,
    /// 获取笔试信息
    ExamGet(String), // exam_id
    // === ehr_import_task 资源 (1个) ===
    /// 更新导入任务结果
    EhrImportTaskPatch,

    // === external_application 资源 (4个) ===
    /// 创建外部投递
    ExternalApplicationCreate,
    /// 删除外部投递
    ExternalApplicationDelete(String), // application_id
    /// 获取外部投递列表
    ExternalApplicationList,
    /// 更新外部投递
    ExternalApplicationUpdate(String), // application_id

    // === external_interview 资源 (4个) ===
    /// 创建外部面试
    ExternalInterviewCreate,
    /// 删除外部面试
    ExternalInterviewDelete(String), // interview_id
    /// 获取外部面试列表
    ExternalInterviewBatchQuery,
    /// 更新外部面试
    ExternalInterviewUpdate(String), // interview_id

    // === external_interview_assessment 资源 (2个) ===
    /// 创建外部面评
    ExternalInterviewAssessmentCreate,
    /// 更新外部面评
    ExternalInterviewAssessmentPatch(String), // assessment_id

    // === external_background_check 资源 (4个) ===
    /// 创建外部背调
    ExternalBackgroundCheckCreate,
    /// 删除外部背调
    ExternalBackgroundCheckDelete(String), // check_id
    /// 获取外部背调列表
    ExternalBackgroundCheckBatchQuery,
    /// 更新外部背调
    ExternalBackgroundCheckUpdate(String), // check_id

    // === external_offer 资源 (4个) ===
    /// 创建外部 Offer
    ExternalOfferCreate,
    /// 删除外部 Offer
    ExternalOfferDelete(String), // offer_id
    /// 获取外部 Offer 列表
    ExternalOfferBatchQuery,
    /// 更新外部 Offer
    ExternalOfferUpdate(String), // offer_id

    // === external_referral_reward 资源 (2个) ===
    /// 创建外部内推奖励
    ExternalReferralRewardCreate,
    /// 删除外部内推奖励
    ExternalReferralRewardDelete(String), // reward_id

    // === tripartite_agreement 资源 (4个) ===
    /// 创建三方协议
    TripartiteAgreementCreate,
    /// 删除三方协议
    TripartiteAgreementDelete(String), // agreement_id
    /// 获取三方协议列表
    TripartiteAgreementList,
    /// 更新三方协议
    TripartiteAgreementUpdate(String), // agreement_id

    // === minutes 资源 (1个) ===
    /// 获取面试速记明细
    MinutesGet(String), // minutes_id

    // === diversity_inclusion 资源 (1个) ===
    /// 多元化与包容性搜索
    DiversityInclusionSearch,

    // === job.recruiter 资源 (1个) ===
    /// 获取职位招聘人员
    JobRecruiter(String), // job_id

    // === application.offer 资源 (1个) ===
    /// 获取投递 Offer 信息
    ApplicationOffer(String), // application_id

    // === application.interview 资源 (1个) ===
    /// 获取投递面试列表
    ApplicationInterviewList(String), // application_id

    // === external_interview_assessment 资源 (2个) ===
    /// 获取面试评价详细信息（新版）
    ExternalInterviewAssessmentGet(String), // assessment_id

    // === advertisement 资源 (1个) ===
    /// 发布职位广告
    AdvertisementPublish(String), // job_id
}

impl HireApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            // talent
            HireApiV1::TalentCombinedCreate => {
                "/open-apis/hire/v1/talents/combined_create".to_string()
            }
            HireApiV1::TalentCombinedUpdate => {
                "/open-apis/hire/v1/talents/combined_update".to_string()
            }
            HireApiV1::TalentGet(talent_id) => format!("/open-apis/hire/v1/talents/{talent_id}"),
            HireApiV1::TalentList => "/open-apis/hire/v1/talents".to_string(),
            HireApiV1::TalentBatchGetId => "/open-apis/hire/v1/talents/batch_get_id".to_string(),
            HireApiV1::TalentAddToFolder(talent_id) => {
                format!("/open-apis/hire/v1/talents/{talent_id}/add_to_folder")
            }
            HireApiV1::TalentRemoveToFolder(talent_id) => {
                format!("/open-apis/hire/v1/talents/{talent_id}/remove_to_folder")
            }
            HireApiV1::TalentExternalInfoCreate(talent_id) => {
                format!("/open-apis/hire/v1/talents/{talent_id}/external_info")
            }
            HireApiV1::TalentExternalInfoUpdate(talent_id) => {
                format!("/open-apis/hire/v1/talents/{talent_id}/external_info")
            }
            HireApiV1::TalentTag(talent_id) => {
                format!("/open-apis/hire/v1/talents/{talent_id}/tags")
            }
            HireApiV1::TalentObjectQuery => "/open-apis/hire/v1/talent_object/query".to_string(),
            HireApiV1::TalentOnboardStatus(talent_id) => {
                format!("/open-apis/hire/v1/talents/{talent_id}/onboard_status")
            }

            // talent_folder
            HireApiV1::TalentFolderList => "/open-apis/hire/v1/talent_folders".to_string(),

            // talent_pool
            HireApiV1::TalentPoolSearch => "/open-apis/hire/v1/talent_pools/search".to_string(),
            HireApiV1::TalentPoolBatchChangeTalentPool => {
                "/open-apis/hire/v1/talent_pools/batch_change_talent_pool".to_string()
            }
            HireApiV1::TalentPoolMoveTalent => {
                "/open-apis/hire/v1/talent_pools/move_talent".to_string()
            }

            // talent_tag
            HireApiV1::TalentTagList => "/open-apis/hire/v1/talent_tags".to_string(),

            // talent_blocklist
            HireApiV1::TalentBlocklistChangeTalentBlock => {
                "/open-apis/hire/v1/talent_blocklists/change_talent_block".to_string()
            }

            // talent_operation_log
            HireApiV1::TalentOperationLogSearch => {
                "/open-apis/hire/v1/talent_operation_logs/search".to_string()
            }

            // application
            HireApiV1::ApplicationCreate => "/open-apis/hire/v1/applications".to_string(),
            HireApiV1::ApplicationGet(application_id) => {
                format!("/open-apis/hire/v1/applications/{application_id}")
            }
            HireApiV1::ApplicationList => "/open-apis/hire/v1/applications".to_string(),
            HireApiV1::ApplicationGetDetail(application_id) => {
                format!("/open-apis/hire/v1/applications/{application_id}/detail")
            }
            HireApiV1::ApplicationTransferStage(application_id) => {
                format!("/open-apis/hire/v1/applications/{application_id}/transfer_stage")
            }
            HireApiV1::ApplicationTerminate(application_id) => {
                format!("/open-apis/hire/v1/applications/{application_id}/terminate")
            }
            HireApiV1::ApplicationRecover(application_id) => {
                format!("/open-apis/hire/v1/applications/{application_id}/recover")
            }
            HireApiV1::ApplicationCancelOnboard(application_id) => {
                format!("/open-apis/hire/v1/applications/{application_id}/cancel_onboard")
            }
            HireApiV1::ApplicationTransferOnboard(application_id) => {
                format!("/open-apis/hire/v1/applications/{application_id}/transfer_onboard")
            }

            // interview
            HireApiV1::InterviewGetByTalent(_) => {
                "/open-apis/hire/v1/interviews/get_by_talent".to_string()
            }
            HireApiV1::InterviewList => "/open-apis/hire/v1/interviews".to_string(),
            HireApiV1::InterviewTaskList => "/open-apis/hire/v1/interview_tasks".to_string(),

            // interview_feedback_form
            HireApiV1::InterviewFeedbackFormList => {
                "/open-apis/hire/v1/interview_feedback_forms".to_string()
            }

            // interview_record
            HireApiV1::InterviewRecordGet(interview_record_id) => {
                format!("/open-apis/hire/v1/interview_records/{interview_record_id}")
            }
            HireApiV1::InterviewRecordList => "/open-apis/hire/v1/interview_records".to_string(),
            HireApiV1::InterviewRecordAttachmentGet(interview_record_id) => {
                format!("/open-apis/hire/v1/interview_records/{interview_record_id}/attachment")
            }

            // interview_registration_schema
            HireApiV1::InterviewRegistrationSchemaList => {
                "/open-apis/hire/v1/interview_registration_schemas".to_string()
            }

            // interview_round_type
            HireApiV1::InterviewRoundTypeList => {
                "/open-apis/hire/v1/interview_round_types".to_string()
            }

            // interviewer
            HireApiV1::InterviewerList => "/open-apis/hire/v1/interviewers".to_string(),
            HireApiV1::InterviewerPatch(interviewer_id) => {
                format!("/open-apis/hire/v1/interviewers/{interviewer_id}")
            }

            // employee
            HireApiV1::EmployeeGet(employee_id) => {
                format!("/open-apis/hire/v1/employees/{employee_id}")
            }
            HireApiV1::EmployeeGetByApplication(_) => {
                "/open-apis/hire/v1/employees/get_by_application".to_string()
            }
            HireApiV1::EmployeePatch(employee_id) => {
                format!("/open-apis/hire/v1/employees/{employee_id}")
            }

            // job
            HireApiV1::JobCombinedCreate => "/open-apis/hire/v1/jobs/combined_create".to_string(),
            HireApiV1::JobCombinedUpdate => "/open-apis/hire/v1/jobs/combined_update".to_string(),
            HireApiV1::JobGet(job_id) => format!("/open-apis/hire/v1/jobs/{job_id}"),
            HireApiV1::JobList => "/open-apis/hire/v1/jobs".to_string(),
            HireApiV1::JobGetDetail(job_id) => {
                format!("/open-apis/hire/v1/jobs/{job_id}/detail")
            }
            HireApiV1::JobClose(job_id) => {
                format!("/open-apis/hire/v1/jobs/{job_id}/close")
            }
            HireApiV1::JobOpen(job_id) => {
                format!("/open-apis/hire/v1/jobs/{job_id}/open")
            }
            HireApiV1::JobUpdateConfig(job_id) => {
                format!("/open-apis/hire/v1/jobs/{job_id}/update_config")
            }
            HireApiV1::JobConfig(job_id) => {
                format!("/open-apis/hire/v1/jobs/{job_id}/config")
            }
            HireApiV1::JobManagerBatchUpdate => {
                "/open-apis/hire/v1/jobs/managers/batch_update".to_string()
            }

            // job_manager
            HireApiV1::JobManagerGet(job_id) => {
                format!("/open-apis/hire/v1/jobs/{job_id}/managers")
            }

            // job_function
            HireApiV1::JobFunctionList => "/open-apis/hire/v1/job_functions".to_string(),

            // job_process
            HireApiV1::JobProcessList => "/open-apis/hire/v1/job_processes".to_string(),

            // job_requirement
            HireApiV1::JobRequirementCreate => "/open-apis/hire/v1/job_requirements".to_string(),
            HireApiV1::JobRequirementList => "/open-apis/hire/v1/job_requirements".to_string(),
            HireApiV1::JobRequirementListById => {
                "/open-apis/hire/v1/job_requirements/search".to_string()
            }
            HireApiV1::JobRequirementUpdate => {
                "/open-apis/hire/v1/job_requirements/update".to_string()
            }
            HireApiV1::JobRequirementDelete(requirement_id) => {
                format!("/open-apis/hire/v1/job_requirements/{requirement_id}")
            }

            // job_requirement_schema
            HireApiV1::JobRequirementSchemaList => {
                "/open-apis/hire/v1/job_requirement_schemas".to_string()
            }

            // job_schema
            HireApiV1::JobSchemaList => "/open-apis/hire/v1/job_schemas".to_string(),

            // job_type
            HireApiV1::JobTypeList => "/open-apis/hire/v1/job_types".to_string(),

            // job_publish_record
            HireApiV1::JobPublishRecordSearch => {
                "/open-apis/hire/v1/job_publish_records/search".to_string()
            }

            // attachment
            HireApiV1::AttachmentCreate => "/open-apis/hire/v1/attachments".to_string(),
            HireApiV1::AttachmentGet(attachment_id) => {
                format!("/open-apis/hire/v1/attachments/{attachment_id}")
            }
            HireApiV1::AttachmentPreview(attachment_id) => {
                format!("/open-apis/hire/v1/attachments/{attachment_id}/preview")
            }

            // note
            HireApiV1::NoteCreate => "/open-apis/hire/v1/notes".to_string(),
            HireApiV1::NoteDelete(note_id) => {
                format!("/open-apis/hire/v1/notes/{note_id}")
            }
            HireApiV1::NoteGet(note_id) => {
                format!("/open-apis/hire/v1/notes/{note_id}")
            }
            HireApiV1::NoteList => "/open-apis/hire/v1/notes".to_string(),
            HireApiV1::NotePatch(note_id) => {
                format!("/open-apis/hire/v1/notes/{note_id}")
            }

            // offer
            HireApiV1::OfferCreate => "/open-apis/hire/v1/offers".to_string(),
            HireApiV1::OfferGet(offer_id) => {
                format!("/open-apis/hire/v1/offers/{offer_id}")
            }
            HireApiV1::OfferList => "/open-apis/hire/v1/offers".to_string(),
            HireApiV1::OfferUpdate(offer_id) => {
                format!("/open-apis/hire/v1/offers/{offer_id}")
            }
            HireApiV1::OfferOfferStatus(offer_id) => {
                format!("/open-apis/hire/v1/offers/{offer_id}/offer_status")
            }
            HireApiV1::OfferInternOfferStatus(offer_id) => {
                format!("/open-apis/hire/v1/offers/{offer_id}/intern_offer_status")
            }

            // offer_approval_template
            HireApiV1::OfferApprovalTemplateList => {
                "/open-apis/hire/v1/offer_approval_templates".to_string()
            }

            // offer_application_form
            HireApiV1::OfferApplicationFormGet(application_form_id) => {
                format!("/open-apis/hire/v1/offer_application_forms/{application_form_id}")
            }
            HireApiV1::OfferApplicationFormList => {
                "/open-apis/hire/v1/offer_application_forms".to_string()
            }

            // offer_schema
            HireApiV1::OfferSchemaGet(schema_id) => {
                format!("/open-apis/hire/v1/offer_schemas/{schema_id}")
            }

            // offer_custom_field
            HireApiV1::OfferCustomFieldUpdate => {
                "/open-apis/hire/v1/offer_custom_fields/update".to_string()
            }

            // referral
            HireApiV1::ReferralGetByApplication(application_id) => {
                format!("/open-apis/hire/v1/applications/{application_id}/referral")
            }
            HireApiV1::ReferralSearch => "/open-apis/hire/v1/referrals/search".to_string(),

            // referral_account
            HireApiV1::ReferralAccountCreate => "/open-apis/hire/v1/referral_accounts".to_string(),
            HireApiV1::ReferralAccountEnable(account_id) => {
                format!("/open-apis/hire/v1/referral_accounts/{account_id}/enable")
            }
            HireApiV1::ReferralAccountDeactivate(account_id) => {
                format!("/open-apis/hire/v1/referral_accounts/{account_id}/deactivate")
            }
            HireApiV1::ReferralAccountReconciliation => {
                "/open-apis/hire/v1/referral_accounts/reconciliation".to_string()
            }
            HireApiV1::ReferralAccountWithdraw => {
                "/open-apis/hire/v1/referral_accounts/withdraw".to_string()
            }
            HireApiV1::ReferralAccountGetAccountAssets(account_id) => {
                format!("/open-apis/hire/v1/referral_accounts/{account_id}/assets")
            }

            // referral_website
            HireApiV1::ReferralWebsiteJobPostList => {
                "/open-apis/hire/v1/referral_websites/job_posts".to_string()
            }
            HireApiV1::ReferralWebsiteJobPostGet(job_post_id) => {
                format!("/open-apis/hire/v1/referral_websites/job_posts/{job_post_id}")
            }

            // website
            HireApiV1::WebsiteList => "/open-apis/hire/v1/websites".to_string(),

            // website_channel
            HireApiV1::WebsiteChannelCreate => "/open-apis/hire/v1/website_channels".to_string(),
            HireApiV1::WebsiteChannelList => "/open-apis/hire/v1/website_channels".to_string(),
            HireApiV1::WebsiteChannelUpdate(channel_id) => {
                format!("/open-apis/hire/v1/website_channels/{channel_id}")
            }
            HireApiV1::WebsiteChannelDelete(channel_id) => {
                format!("/open-apis/hire/v1/website_channels/{channel_id}")
            }

            // website_delivery
            HireApiV1::WebsiteDeliveryCreateByAttachment => {
                "/open-apis/hire/v1/website_deliveries/create_by_attachment".to_string()
            }
            HireApiV1::WebsiteDeliveryCreateByResume => {
                "/open-apis/hire/v1/website_deliveries/create_by_resume".to_string()
            }

            // website_delivery_task
            HireApiV1::WebsiteDeliveryTaskGet => {
                "/open-apis/hire/v1/website_delivery_tasks/get".to_string()
            }

            // website_job_post
            HireApiV1::WebsiteJobPostList => {
                "/open-apis/hire/v1/website_job_posts/list".to_string()
            }
            HireApiV1::WebsiteJobPostSearch => {
                "/open-apis/hire/v1/website_job_posts/search".to_string()
            }
            HireApiV1::WebsiteJobPostGet(job_post_id) => {
                format!("/open-apis/hire/v1/website_job_posts/{job_post_id}")
            }

            // website_site_user
            HireApiV1::WebsiteSiteUserCreate => "/open-apis/hire/v1/website_site_users".to_string(),

            // resume_source
            HireApiV1::ResumeSourceList => "/open-apis/hire/v1/resume_sources".to_string(),

            // role
            HireApiV1::RoleList => "/open-apis/hire/v1/roles".to_string(),
            HireApiV1::RoleGet(role_id) => {
                format!("/open-apis/hire/v1/roles/{role_id}")
            }

            // subject
            HireApiV1::SubjectList => "/open-apis/hire/v1/subjects".to_string(),

            // questionnaire
            HireApiV1::QuestionnaireList => "/open-apis/hire/v1/questionnaires".to_string(),

            // portal_apply_schema
            HireApiV1::PortalApplySchemaList => {
                "/open-apis/hire/v1/portal_apply_schemas".to_string()
            }

            // registration_schema
            HireApiV1::RegistrationSchemaList => {
                "/open-apis/hire/v1/registration_schemas".to_string()
            }

            // termination_reason
            HireApiV1::TerminationReasonList => {
                "/open-apis/hire/v1/termination_reasons".to_string()
            }

            // test
            HireApiV1::ExamCreate => "/open-apis/hire/v1/exams".to_string(),
            HireApiV1::ExamGet(exam_id) => format!("/open-apis/hire/v1/exams/{exam_id}"),
            HireApiV1::TestSearch => "/open-apis/hire/v1/tests/search".to_string(),

            // todo
            HireApiV1::TodoList => "/open-apis/hire/v1/todos".to_string(),

            // agency
            HireApiV1::AgencyBatchQuery => "/open-apis/hire/v1/agencies/batch_query".to_string(),
            HireApiV1::AgencyQuery => "/open-apis/hire/v1/agencies/query".to_string(),
            HireApiV1::AgencyGet(agency_id) => {
                format!("/open-apis/hire/v1/agencies/{agency_id}")
            }
            HireApiV1::AgencyGetAgencyAccount(agency_id) => {
                format!("/open-apis/hire/v1/agencies/{agency_id}/account")
            }
            HireApiV1::AgencyOperateAgencyAccount(agency_id) => {
                format!("/open-apis/hire/v1/agencies/{agency_id}/account/operate")
            }
            HireApiV1::AgencyProtect(agency_id) => {
                format!("/open-apis/hire/v1/agencies/{agency_id}/protect")
            }
            HireApiV1::AgencyProtectSearch => {
                "/open-apis/hire/v1/agencies/protect_search".to_string()
            }

            // background_check_order
            HireApiV1::BackgroundCheckOrderBatchQuery => {
                "/open-apis/hire/v1/background_check_orders/batch_query".to_string()
            }
            HireApiV1::BackgroundCheckOrderList => {
                "/open-apis/hire/v1/background_check_orders".to_string()
            }

            // eco_background_check
            HireApiV1::EcoBackgroundCheckCancel => {
                "/open-apis/hire/v1/eco_background_checks/cancel".to_string()
            }
            HireApiV1::EcoBackgroundCheckUpdateResult(order_id) => {
                format!("/open-apis/hire/v1/eco_background_checks/{order_id}/update_result")
            }
            HireApiV1::EcoBackgroundCheckUpdateProgress(order_id) => {
                format!("/open-apis/hire/v1/eco_background_checks/{order_id}/update_progress")
            }

            // eco_background_check_package
            HireApiV1::EcoBackgroundCheckPackageCreate => {
                "/open-apis/hire/v1/eco_background_check_packages".to_string()
            }
            HireApiV1::EcoBackgroundCheckPackageBatchDelete => {
                "/open-apis/hire/v1/eco_background_check_packages/batch_delete".to_string()
            }
            HireApiV1::EcoBackgroundCheckPackageBatchUpdate => {
                "/open-apis/hire/v1/eco_background_check_packages/batch_update".to_string()
            }

            // eco_background_check_custom_field
            HireApiV1::EcoBackgroundCheckCustomFieldCreate => {
                "/open-apis/hire/v1/eco_background_check_custom_fields".to_string()
            }
            HireApiV1::EcoBackgroundCheckCustomFieldBatchDelete => {
                "/open-apis/hire/v1/eco_background_check_custom_fields/batch_delete".to_string()
            }
            HireApiV1::EcoBackgroundCheckCustomFieldBatchUpdate => {
                "/open-apis/hire/v1/eco_background_check_custom_fields/batch_update".to_string()
            }

            // eco_exam
            HireApiV1::EcoExamLoginInfo(exam_id) => {
                format!("/open-apis/hire/v1/eco_exams/{exam_id}/login_info")
            }
            HireApiV1::EcoExamUpdateResult(exam_id) => {
                format!("/open-apis/hire/v1/eco_exams/{exam_id}/update_result")
            }

            // eco_exam_paper
            HireApiV1::EcoExamPaperCreate => "/open-apis/hire/v1/eco_exam_papers".to_string(),
            HireApiV1::EcoExamPaperBatchDelete => {
                "/open-apis/hire/v1/eco_exam_papers/batch_delete".to_string()
            }
            HireApiV1::EcoExamPaperBatchUpdate => {
                "/open-apis/hire/v1/eco_exam_papers/batch_update".to_string()
            }

            // eco_account_custom_field
            HireApiV1::EcoAccountCustomFieldCreate => {
                "/open-apis/hire/v1/eco_account_custom_fields".to_string()
            }
            HireApiV1::EcoAccountCustomFieldBatchDelete => {
                "/open-apis/hire/v1/eco_account_custom_fields/batch_delete".to_string()
            }
            HireApiV1::EcoAccountCustomFieldBatchUpdate => {
                "/open-apis/hire/v1/eco_account_custom_fields/batch_update".to_string()
            }

            // ehr_import_task
            HireApiV1::EhrImportTaskPatch => {
                "/open-apis/hire/v1/ehr_import_tasks/patch".to_string()
            }

            // external_application
            HireApiV1::ExternalApplicationCreate => {
                "/open-apis/hire/v1/external_applications".to_string()
            }
            HireApiV1::ExternalApplicationDelete(application_id) => {
                format!("/open-apis/hire/v1/external_applications/{application_id}")
            }
            HireApiV1::ExternalApplicationList => {
                "/open-apis/hire/v1/external_applications".to_string()
            }
            HireApiV1::ExternalApplicationUpdate(application_id) => {
                format!("/open-apis/hire/v1/external_applications/{application_id}")
            }

            // external_interview
            HireApiV1::ExternalInterviewCreate => {
                "/open-apis/hire/v1/external_interviews".to_string()
            }
            HireApiV1::ExternalInterviewDelete(interview_id) => {
                format!("/open-apis/hire/v1/external_interviews/{interview_id}")
            }
            HireApiV1::ExternalInterviewBatchQuery => {
                "/open-apis/hire/v1/external_interviews/batch_query".to_string()
            }
            HireApiV1::ExternalInterviewUpdate(interview_id) => {
                format!("/open-apis/hire/v1/external_interviews/{interview_id}")
            }

            // external_interview_assessment
            HireApiV1::ExternalInterviewAssessmentCreate => {
                "/open-apis/hire/v1/external_interview_assessments".to_string()
            }
            HireApiV1::ExternalInterviewAssessmentPatch(assessment_id) => {
                format!("/open-apis/hire/v1/external_interview_assessments/{assessment_id}")
            }

            // external_background_check
            HireApiV1::ExternalBackgroundCheckCreate => {
                "/open-apis/hire/v1/external_background_checks".to_string()
            }
            HireApiV1::ExternalBackgroundCheckDelete(check_id) => {
                format!("/open-apis/hire/v1/external_background_checks/{check_id}")
            }
            HireApiV1::ExternalBackgroundCheckBatchQuery => {
                "/open-apis/hire/v1/external_background_checks/batch_query".to_string()
            }
            HireApiV1::ExternalBackgroundCheckUpdate(check_id) => {
                format!("/open-apis/hire/v1/external_background_checks/{check_id}")
            }

            // external_offer
            HireApiV1::ExternalOfferCreate => "/open-apis/hire/v1/external_offers".to_string(),
            HireApiV1::ExternalOfferDelete(offer_id) => {
                format!("/open-apis/hire/v1/external_offers/{offer_id}")
            }
            HireApiV1::ExternalOfferBatchQuery => {
                "/open-apis/hire/v1/external_offers/batch_query".to_string()
            }
            HireApiV1::ExternalOfferUpdate(offer_id) => {
                format!("/open-apis/hire/v1/external_offers/{offer_id}")
            }

            // external_referral_reward
            HireApiV1::ExternalReferralRewardCreate => {
                "/open-apis/hire/v1/external_referral_rewards".to_string()
            }
            HireApiV1::ExternalReferralRewardDelete(reward_id) => {
                format!("/open-apis/hire/v1/external_referral_rewards/{reward_id}")
            }

            // tripartite_agreement
            HireApiV1::TripartiteAgreementCreate => {
                "/open-apis/hire/v1/tripartite_agreements".to_string()
            }
            HireApiV1::TripartiteAgreementDelete(agreement_id) => {
                format!("/open-apis/hire/v1/tripartite_agreements/{agreement_id}")
            }
            HireApiV1::TripartiteAgreementList => {
                "/open-apis/hire/v1/tripartite_agreements".to_string()
            }
            HireApiV1::TripartiteAgreementUpdate(agreement_id) => {
                format!("/open-apis/hire/v1/tripartite_agreements/{agreement_id}")
            }

            // minutes
            HireApiV1::MinutesGet(minutes_id) => {
                format!("/open-apis/hire/v1/minutes/{minutes_id}")
            }

            // diversity_inclusion
            HireApiV1::DiversityInclusionSearch => {
                "/open-apis/hire/v1/applications/diversity_inclusions/search".to_string()
            }

            // job.recruiter
            HireApiV1::JobRecruiter(job_id) => {
                format!("/open-apis/hire/v1/jobs/{job_id}/recruiter")
            }

            // application.offer
            HireApiV1::ApplicationOffer(application_id) => {
                format!("/open-apis/hire/v1/applications/{application_id}/offer")
            }

            // application.interview
            HireApiV1::ApplicationInterviewList(application_id) => {
                format!("/open-apis/hire/v1/applications/{application_id}/interviews")
            }

            // external_interview_assessment (duplicate variant, using get variant)
            HireApiV1::ExternalInterviewAssessmentGet(assessment_id) => {
                format!("/open-apis/hire/v1/external_interview_assessments/{assessment_id}")
            }

            // advertisement
            HireApiV1::AdvertisementPublish(job_id) => {
                format!("/open-apis/hire/v1/jobs/{job_id}/advertisement/publish")
            }
        }
    }
}

/// Hire API V2 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum HireApiV2 {
    // === interview_record 资源 (2个) ===
    /// 获取面试评价详细信息（新版）
    InterviewRecordGet(String), // interview_record_id
    /// 批量获取面试评价详细信息（新版）
    InterviewRecordList,

    // === talent 资源 (1个) ===
    /// 获取人才详情
    TalentGet(String), // talent_id
}

impl HireApiV2 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            // interview_record
            HireApiV2::InterviewRecordGet(interview_record_id) => {
                format!("/open-apis/hire/v2/interview_records/{interview_record_id}")
            }
            HireApiV2::InterviewRecordList => "/open-apis/hire/v2/interview_records".to_string(),

            // talent
            HireApiV2::TalentGet(talent_id) => {
                format!("/open-apis/hire/v2/talents/{talent_id}")
            }
        }
    }
}

// Compensation API V1 - 薪酬管理 (21 APIs)

/// Compensation API V1 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum CompensationApiV1 {
    // === archive 资源 (2个) ===
    /// 创建薪资档案
    ArchiveCreate,
    /// 批量查询员工薪资档案
    ArchiveQuery,

    // === change_reason 资源 (1个) ===
    /// 批量查询定调薪原因
    ChangeReasonList,

    // === indicator 资源 (1个) ===
    /// 批量查询薪资统计指标
    IndicatorList,

    // === item 资源 (1个) ===
    /// 批量查询薪资项
    ItemList,

    // === item_category 资源 (1个) ===
    /// 批量获取薪资项分类信息
    ItemCategoryList,

    // === lump_sum_payment 资源 (4个) ===
    /// 批量创建一次性支付记录
    LumpSumPaymentBatchCreate,
    /// 批量删除一次性支付记录
    LumpSumPaymentBatchRemove,
    /// 批量更正一次性支付记录
    LumpSumPaymentBatchUpdate,
    /// 查询一次性支付授予记录
    LumpSumPaymentQuery,
    /// 查询一次性支付授予明细
    LumpSumPaymentQueryDetail,

    // === plan 资源 (1个) ===
    /// 批量查询薪资方案
    PlanList,

    // === recurring_payment 资源 (4个) ===
    /// 批量创建经常性支付记录
    RecurringPaymentBatchCreate,
    /// 批量删除经常性支付记录
    RecurringPaymentBatchRemove,
    /// 批量更正经常性支付记录
    RecurringPaymentBatchUpdate,
    /// 查询经常性支付记录
    RecurringPaymentQuery,

    // === social_archive 资源 (1个) ===
    /// 批量获取员工参保档案
    SocialArchiveQuery,

    // === social_archive_adjust_record 资源 (1个) ===
    /// 通过员工ID批量获取社保增减员记录
    SocialArchiveAdjustRecordQuery,

    // === social_insurance 资源 (1个) ===
    /// 获取险种配置列表
    SocialInsuranceList,

    // === social_plan 资源 (2个) ===
    /// 根据生效日期分页查询参保方案
    SocialPlanList,
    /// 根据方案ID和生效日期批量查询参保方案
    SocialPlanQuery,
}

impl CompensationApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            // archive
            CompensationApiV1::ArchiveCreate => "/open-apis/compensation/v1/archives".to_string(),
            CompensationApiV1::ArchiveQuery => {
                "/open-apis/compensation/v1/archives/query".to_string()
            }

            // change_reason
            CompensationApiV1::ChangeReasonList => {
                "/open-apis/compensation/v1/change_reasons".to_string()
            }

            // indicator
            CompensationApiV1::IndicatorList => "/open-apis/compensation/v1/indicators".to_string(),

            // item
            CompensationApiV1::ItemList => "/open-apis/compensation/v1/items".to_string(),

            // item_category
            CompensationApiV1::ItemCategoryList => {
                "/open-apis/compensation/v1/item_categories".to_string()
            }

            // lump_sum_payment
            CompensationApiV1::LumpSumPaymentBatchCreate => {
                "/open-apis/compensation/v1/lump_sum_payment/batch_create".to_string()
            }
            CompensationApiV1::LumpSumPaymentBatchRemove => {
                "/open-apis/compensation/v1/lump_sum_payment/batch_remove".to_string()
            }
            CompensationApiV1::LumpSumPaymentBatchUpdate => {
                "/open-apis/compensation/v1/lump_sum_payment/batch_update".to_string()
            }
            CompensationApiV1::LumpSumPaymentQuery => {
                "/open-apis/compensation/v1/lump_sum_payment/query".to_string()
            }
            CompensationApiV1::LumpSumPaymentQueryDetail => {
                "/open-apis/compensation/v1/lump_sum_payment/query_detail".to_string()
            }

            // plan
            CompensationApiV1::PlanList => "/open-apis/compensation/v1/plans".to_string(),

            // recurring_payment
            CompensationApiV1::RecurringPaymentBatchCreate => {
                "/open-apis/compensation/v1/recurring_payment/batch_create".to_string()
            }
            CompensationApiV1::RecurringPaymentBatchRemove => {
                "/open-apis/compensation/v1/recurring_payment/batch_remove".to_string()
            }
            CompensationApiV1::RecurringPaymentBatchUpdate => {
                "/open-apis/compensation/v1/recurring_payment/batch_update".to_string()
            }
            CompensationApiV1::RecurringPaymentQuery => {
                "/open-apis/compensation/v1/recurring_payment/query".to_string()
            }

            // social_archive
            CompensationApiV1::SocialArchiveQuery => {
                "/open-apis/compensation/v1/social_archive/query".to_string()
            }

            // social_archive_adjust_record
            CompensationApiV1::SocialArchiveAdjustRecordQuery => {
                "/open-apis/compensation/v1/social_archive_adjust_record/query".to_string()
            }

            // social_insurance
            CompensationApiV1::SocialInsuranceList => {
                "/open-apis/compensation/v1/social_insurances".to_string()
            }

            // social_plan
            CompensationApiV1::SocialPlanList => {
                "/open-apis/compensation/v1/social_plans".to_string()
            }
            CompensationApiV1::SocialPlanQuery => {
                "/open-apis/compensation/v1/social_plans/query".to_string()
            }
        }
    }
}

// Performance API V1 - 绩效管理 (21 APIs)

/// Performance API V1 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum PerformanceApiV1 {
    // === activity 资源 (1个) ===
    /// 查询绩效活动
    ActivityQuery,

    // === additional_information 资源 (3个) ===
    /// 导入附加信息
    AdditionalInformationImport,
    /// 查询附加信息
    AdditionalInformationQuery,
    /// 批量删除附加信息
    AdditionalInformationsBatchDelete,

    // === indicator 资源 (1个) ===
    /// 查询指标
    IndicatorQuery,

    // === metric_detail 资源 (2个) ===
    /// 导入指标明细
    MetricDetailImport,
    /// 查询指标明细
    MetricDetailQuery,

    // === metric_field 资源 (1个) ===
    /// 查询指标字段
    MetricFieldQuery,

    // === metric_lib 资源 (1个) ===
    /// 查询指标库
    MetricLibQuery,

    // === metric_tag 资源 (1个) ===
    /// 查询指标标签
    MetricTagList,

    // === metric_template 资源 (1个) ===
    /// 查询指标模板
    MetricTemplateQuery,

    // === question 资源 (1个) ===
    /// 查询问题
    QuestionQuery,

    // === review 资源 (1个) ===
    /// 查询评审
    ReviewQuery,

    // === review_data 资源 (2个) ===
    /// 查询评审数据
    ReviewDataQuery,

    // === review_template 资源 (1个) ===
    /// 查询评审模板
    ReviewTemplateQuery,

    // === reviewee 资源 (1个) ===
    /// 查询被评估人
    RevieweeQuery,

    // === semester 资源 (1个) ===
    /// 查询学期
    SemesterList,

    // === stage_task 资源 (2个) ===
    /// 按页查找阶段任务
    StageTaskFindByPage,
    /// 按用户列表查找阶段任务
    StageTaskFindByUserList,

    // === user_group_user_rel 资源 (1个) ===
    /// 写入用户组用户关系
    UserGroupUserRelWrite,

    // === user_info 资源 (1个) ===
    /// 查询用户信息
    UserInfoQuery,
}

impl PerformanceApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            // activity
            PerformanceApiV1::ActivityQuery => {
                "/open-apis/performance/v2/activity/query".to_string()
            }

            // additional_information
            PerformanceApiV1::AdditionalInformationImport => {
                "/open-apis/performance/v2/additional_informations/import".to_string()
            }
            PerformanceApiV1::AdditionalInformationQuery => {
                "/open-apis/performance/v2/additional_informations/query".to_string()
            }
            PerformanceApiV1::AdditionalInformationsBatchDelete => {
                "/open-apis/performance/v2/additional_informations/batch".to_string()
            }

            // indicator
            PerformanceApiV1::IndicatorQuery => {
                "/open-apis/performance/v2/indicators/query".to_string()
            }

            // metric_detail
            PerformanceApiV1::MetricDetailImport => {
                "/open-apis/performance/v2/metric_details/import".to_string()
            }
            PerformanceApiV1::MetricDetailQuery => {
                "/open-apis/performance/v2/metric_details/query".to_string()
            }

            // metric_field
            PerformanceApiV1::MetricFieldQuery => {
                "/open-apis/performance/v2/metric_fields/query".to_string()
            }

            // metric_lib
            PerformanceApiV1::MetricLibQuery => {
                "/open-apis/performance/v2/metric_libs/query".to_string()
            }

            // metric_tag
            PerformanceApiV1::MetricTagList => "/open-apis/performance/v2/metric_tags".to_string(),

            // metric_template
            PerformanceApiV1::MetricTemplateQuery => {
                "/open-apis/performance/v2/metric_templates/query".to_string()
            }

            // question
            PerformanceApiV1::QuestionQuery => {
                "/open-apis/performance/v2/questions/query".to_string()
            }

            // review
            PerformanceApiV1::ReviewQuery => "/open-apis/performance/v1/reviews/query".to_string(),

            // review_data
            PerformanceApiV1::ReviewDataQuery => {
                "/open-apis/performance/v2/review_datas/query".to_string()
            }

            // review_template
            PerformanceApiV1::ReviewTemplateQuery => {
                "/open-apis/performance/v2/review_templates/query".to_string()
            }

            // reviewee
            PerformanceApiV1::RevieweeQuery => {
                "/open-apis/performance/v2/reviewees/query".to_string()
            }

            // semester
            PerformanceApiV1::SemesterList => "/open-apis/performance/v1/semesters".to_string(),

            // stage_task
            PerformanceApiV1::StageTaskFindByPage => {
                "/open-apis/performance/v1/stage_tasks/find_by_page".to_string()
            }
            PerformanceApiV1::StageTaskFindByUserList => {
                "/open-apis/performance/v1/stage_tasks/find_by_user_list".to_string()
            }

            // user_group_user_rel
            PerformanceApiV1::UserGroupUserRelWrite => {
                "/open-apis/performance/v2/user_group_user_rels/write".to_string()
            }

            // user_info
            PerformanceApiV1::UserInfoQuery => {
                "/open-apis/performance/v2/user_info/query".to_string()
            }
        }
    }
}

// Payroll API V1 - 工资管理 (12 APIs)

/// Payroll API V1 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum PayrollApiV1 {
    // === acct_item 资源 (1个) ===
    /// 查询科目项
    AcctItemList,

    // === cost_allocation_detail 资源 (1个) ===
    /// 查询成本分摊明细
    CostAllocationDetailList,

    // === cost_allocation_plan 资源 (1个) ===
    /// 查询成本分摊方案
    CostAllocationPlanList,

    // === cost_allocation_report 资源 (1个) ===
    /// 查询成本分摊报表
    CostAllocationReportList,

    // === datasource 资源 (1个) ===
    /// 查询数据源
    DatasourceList,

    // === datasource_record 资源 (2个) ===
    /// 保存数据源记录
    DatasourceRecordSave,
    /// 查询数据源记录
    DatasourceRecordQuery,

    // === paygroup 资源 (1个) ===
    /// 查询薪资组
    PaygroupList,

    // === payment_activity 资源 (2个) ===
    /// 封存发薪活动
    PaymentActivityArchive(String), // activity_id
    /// 查询发薪活动
    PaymentActivityList,

    // === payment_activity_detail 资源 (1个) ===
    /// 查询发薪活动明细
    PaymentActivityDetailList,

    // === payment_detail 资源 (1个) ===
    /// 查询发薪明细
    PaymentDetailQuery,
}

impl PayrollApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            // acct_item
            PayrollApiV1::AcctItemList => "/open-apis/payroll/v1/acct_items".to_string(),

            // cost_allocation_detail
            PayrollApiV1::CostAllocationDetailList => {
                "/open-apis/payroll/v1/cost_allocation_details".to_string()
            }

            // cost_allocation_plan
            PayrollApiV1::CostAllocationPlanList => {
                "/open-apis/payroll/v1/cost_allocation_plans".to_string()
            }

            // cost_allocation_report
            PayrollApiV1::CostAllocationReportList => {
                "/open-apis/payroll/v1/cost_allocation_reports".to_string()
            }

            // datasource
            PayrollApiV1::DatasourceList => "/open-apis/payroll/v1/datasources".to_string(),

            // datasource_record
            PayrollApiV1::DatasourceRecordSave => {
                "/open-apis/payroll/v1/datasource_records/save".to_string()
            }
            PayrollApiV1::DatasourceRecordQuery => {
                "/open-apis/payroll/v1/datasource_records/query".to_string()
            }

            // paygroup
            PayrollApiV1::PaygroupList => "/open-apis/payroll/v1/paygroups".to_string(),

            // payment_activity
            PayrollApiV1::PaymentActivityArchive(_) => {
                "/open-apis/payroll/v1/payment_activitys/archive".to_string()
            }
            PayrollApiV1::PaymentActivityList => {
                "/open-apis/payroll/v1/payment_activitys".to_string()
            }

            // payment_activity_detail
            PayrollApiV1::PaymentActivityDetailList => {
                "/open-apis/payroll/v1/payment_activity_details".to_string()
            }

            // payment_detail
            PayrollApiV1::PaymentDetailQuery => {
                "/open-apis/payroll/v1/payment_detail/query".to_string()
            }
        }
    }
}

// OKR API V1 - 目标管理 (12 APIs)

/// OKR API V1 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum OkrApiV1 {
    // === image 资源 (1个) ===
    /// 上传进展记录图片
    ImageUpload,

    // === okr 资源 (1个) ===
    /// 批量获取 OKR
    OkrBatchGet,

    // === period 资源 (3个) ===
    /// 创建周期
    PeriodCreate,
    /// 查询周期列表
    PeriodList,
    /// 更新周期
    PeriodPatch(String), // period_id

    // === period_rule 资源 (1个) ===
    /// 查询周期规则
    PeriodRuleList,

    // === progress_record 资源 (4个) ===
    /// 创建进展记录
    ProgressRecordCreate,
    /// 删除进展记录
    ProgressRecordDelete(String), // record_id
    /// 获取进展记录
    ProgressRecordGet(String), // record_id
    /// 更新进展记录
    ProgressRecordUpdate(String), // record_id

    // === review 资源 (1个) ===
    /// 查询评审
    ReviewQuery,

    // === user.okr 资源 (1个) ===
    /// 获取用户的 OKR 列表
    UserOkrList,
}

impl OkrApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            // image
            OkrApiV1::ImageUpload => "/open-apis/okr/v1/images/upload".to_string(),

            // okr
            OkrApiV1::OkrBatchGet => "/open-apis/okr/v1/okrs/batch_get".to_string(),

            // period
            OkrApiV1::PeriodCreate => "/open-apis/okr/v1/periods".to_string(),
            OkrApiV1::PeriodList => "/open-apis/okr/v1/periods".to_string(),
            OkrApiV1::PeriodPatch(period_id) => {
                format!("/open-apis/okr/v1/periods/{period_id}")
            }

            // period_rule
            OkrApiV1::PeriodRuleList => "/open-apis/okr/v1/period_rules".to_string(),

            // progress_record
            OkrApiV1::ProgressRecordCreate => "/open-apis/okr/v1/progress_records".to_string(),
            OkrApiV1::ProgressRecordDelete(record_id) => {
                format!("/open-apis/okr/v1/progress_records/{record_id}")
            }
            OkrApiV1::ProgressRecordGet(record_id) => {
                format!("/open-apis/okr/v1/progress_records/{record_id}")
            }
            OkrApiV1::ProgressRecordUpdate(record_id) => {
                format!("/open-apis/okr/v1/progress_records/{record_id}")
            }

            // review
            OkrApiV1::ReviewQuery => "/open-apis/okr/v1/reviews/query".to_string(),

            // user.okr
            OkrApiV1::UserOkrList => "/open-apis/okr/v1/user_okrs/list".to_string(),
        }
    }
}

// EHR API V1 - EHR 功能 (2 APIs)

/// EHR API V1 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum EhrApiV1 {
    // === attachment 资源 (1个) ===
    /// 下载人员的附件
    AttachmentGet(String), // attachment_id

    // === employee 资源 (1个) ===
    /// 批量获取员工花名册信息
    EmployeeList,
}

impl EhrApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            // attachment
            EhrApiV1::AttachmentGet(attachment_id) => {
                format!("/open-apis/ehr/v1/attachments/{attachment_id}")
            }

            // employee
            EhrApiV1::EmployeeList => "/open-apis/ehr/v1/employees".to_string(),
        }
    }
}

// FeishuPeople API V1/V2 - 核心人力资源 (257 APIs)
// 注：由于 API 数量巨大，这里列出主要的资源分类

/// FeishuPeople API V1 端点枚举（核心人力资源）
#[derive(Debug, Clone, PartialEq)]
pub enum FeishuPeopleApiV1 {
    // === common_data.id 资源 (1个) ===
    /// ID 转换
    CommonDataIdConvert,

    // === common_data.meta_data 资源 (2个) ===
    /// 增加字段枚举值选项
    CommonDataMetaDataAddEnumOption,
    /// 修改字段枚举值选项
    CommonDataMetaDataEditEnumOption,

    // === authorization 资源 (5个) ===
    /// 为用户授权角色
    AuthorizationAddRoleAssign,
    /// 查询单个用户授权
    AuthorizationGetByParam,
    /// 批量查询用户授权
    AuthorizationQuery,
    /// 移除用户被授权的角色
    AuthorizationRemoveRoleAssign,
    /// 更新用户被授权的数据范围
    AuthorizationUpdateRoleAssign,

    // === assigned_user 资源 (1个) ===
    /// 获取组织类角色授权列表
    AssignedUserSearch,

    /// 批量获取角色列表
    SecurityGroupList,
    /// 查询部门 / 地点的 HRBP / 属地 BP
    SecurityGroupQuery,

    // === approval_groups 资源 (4个) ===
    /// 获取审批组
    ApprovalGroupsGet,
    /// 查询部门异动审批列表
    ApprovalGroupsOpenQueryDepartmentChangeListByIds,
    /// 查询岗位异动审批列表
    ApprovalGroupsOpenQueryJobChangeListByIds,
    /// 查询职位异动审批列表
    ApprovalGroupsOpenQueryPositionChangeListByIds,

    // === approver 资源 (1个) ===
    /// 获取审批人列表
    ApproverList,

    // === basic_info.* 资源 (8个) ===
    /// 查询银行信息
    BasicInfoBankSearch,
    /// 查询支行信息
    BasicInfoBankBranchSearch,
    /// 查询城市信息
    BasicInfoCitySearch,
    /// 查询国家/地区信息
    BasicInfoCountryRegionSearch,
    /// 查询省/主要行政区信息
    BasicInfoCountryRegionSubdivisionSearch,
    /// 查询货币信息
    BasicInfoCurrencySearch,
    /// 查询区/县信息
    BasicInfoDistrictSearch,
    /// 查询语言信息
    BasicInfoLanguageSearch,
    /// 查询国籍信息
    BasicInfoNationalitySearch,
    /// 查询时区信息
    BasicInfoTimeZoneSearch,

    // === bp 资源 (2个) ===
    /// 根据部门查询 HRBP / 属地 BP
    BpGetByDepartment,
    /// 查询部门 / 地点的 HRBP / 属地 BP
    BpList,

    // === company 资源 (7个) ===
    /// 创建公司
    CompanyCreate,
    /// 删除公司
    CompanyDelete(String), // company_id
    /// 查询单个公司
    CompanyGet(String), // company_id
    /// 批量查询公司
    CompanyList,
    /// 更新公司
    CompanyPatch(String), // company_id
    /// 批量获取公司
    CompanyBatchGet,
    /// 激活公司
    CompanyActive(String), // company_id
    /// 查询近期变更
    CompanyQueryRecentChange,

    // === compensation_standard 资源 (1个) ===
    /// 获取员工薪资标准
    CompensationStandardMatch,

    // === contract 资源 (5个) ===
    /// 新建合同
    ContractCreate,
    /// 删除合同
    ContractDelete(String), // contract_id
    /// 查询单个合同
    ContractGet(String), // contract_id
    /// 批量查询合同
    ContractList,
    /// 更新合同
    ContractPatch(String), // contract_id
    /// 搜索合同
    ContractSearch,

    // === cost_center 资源 (8个) ===
    /// 创建成本中心
    CostCenterCreate,
    /// 删除成本中心
    CostCenterDelete(String), // cost_center_id
    /// 更新成本中心
    CostCenterPatch(String), // cost_center_id
    /// 搜索成本中心
    CostCenterSearch,
    /// 查询近期变更
    CostCenterQueryRecentChange,
    /// 创建版本
    CostCenterVersionCreate,
    /// 删除版本
    CostCenterVersionDelete(String), // version_id
    /// 更新版本
    CostCenterVersionPatch(String), // version_id
    /// 批量查询
    CostCenterBatchQuery,

    // === country_region 资源 (2个) ===
    /// 查询单条国家/地区信息
    CountryRegionGet(String), // country_region_id
    /// 批量查询国家/地区信息
    CountryRegionList,

    // === currency 资源 (2个) ===
    /// 查询单个货币信息
    CurrencyGet(String), // currency_id
    /// 批量查询货币信息
    CurrencyList,

    // === custom_field 资源 (3个) ===
    /// 获取字段详情
    CustomFieldGetByParam,
    /// 获取飞书人事对象列表
    CustomFieldListObjectName,
    /// 获取自定义字段列表
    CustomFieldQuery,

    // === custom_org 资源 (6个) ===
    /// 创建自定义组织
    CustomOrgCreate,
    /// 删除自定义组织
    CustomOrgDeleteOrg(String), // org_id
    /// 更新自定义组织
    CustomOrgPatch(String), // org_id
    /// 查询自定义组织
    CustomOrgQuery,
    /// 激活自定义组织
    CustomOrgActive(String), // org_id
    /// 查询近期变更
    CustomOrgQueryRecentChange,
    /// 更新匹配规则
    CustomOrgUpdateRule,

    // === default_cost_center 资源 (4个) ===
    /// 创建版本
    DefaultCostCenterCreateVersion,
    /// 删除版本
    DefaultCostCenterRemoveVersion(String), // version_id
    /// 更新版本
    DefaultCostCenterUpdateVersion(String), // version_id
    /// 批量查询
    DefaultCostCenterBatchQuery,

    // === department 资源 (12个) ===
    /// 创建部门
    DepartmentCreate,
    /// 删除部门
    DepartmentDelete(String), // department_id
    /// 查询单个部门
    DepartmentGet(String), // department_id
    /// 批量查询部门
    DepartmentList,
    /// 更新部门
    DepartmentPatch(String), // department_id
    /// 搜索部门
    DepartmentSearch,
    /// 批量获取
    DepartmentBatchGet,
    /// 获取父部门信息
    DepartmentParents(String), // department_id
    /// 获取部门树
    DepartmentTree,
    /// 查询时间轴
    DepartmentQueryTimeline,
    /// 查询多时间轴
    DepartmentQueryMultiTimeline,
    /// 查询操作日志
    DepartmentQueryOperationLogs,
    /// 查询近期变更
    DepartmentQueryRecentChange,

    // === draft 资源 (1个) ===
    /// 获取草稿
    DraftGet(String), // draft_id

    // === employee 资源 (4个) ===
    /// 创建员工
    EmployeeCreate,
    /// 删除员工
    EmployeeDelete(String), // employee_id
    /// 批量获取员工
    EmployeeBatchGet,
    /// 获取员工列表
    EmployeeList,
    /// 搜索员工
    EmployeeSearch,

    // === employee_type 资源 (5个) ===
    /// 创建人员类型
    EmployeeTypeCreate,
    /// 删除人员类型
    EmployeeTypeDelete(String), // employee_type_id
    /// 查询单个人员类型
    EmployeeTypeGet(String), // employee_type_id
    /// 批量查询人员类型
    EmployeeTypeList,
    /// 更新人员类型
    EmployeeTypePatch(String), // employee_type_id

    // === employees.additional_job 资源 (4个) ===
    /// 创建兼职
    EmployeesAdditionalJobCreate,
    /// 删除兼职
    EmployeesAdditionalJobDelete(String), // additional_job_id
    /// 更新兼职
    EmployeesAdditionalJobPatch(String), // additional_job_id
    /// 批量操作兼职
    EmployeesAdditionalJobBatch,

    // === employees.bp 资源 (1个) ===
    /// 批量获取 HRBP
    EmployeesBpBatchGet,

    // === employment 资源 (3个) ===
    /// 创建雇佣信息
    EmploymentCreate,
    /// 删除雇佣信息
    EmploymentDelete(String), // employment_id
    /// 更新雇佣信息
    EmploymentPatch(String), // employment_id

    // === file 资源 (1个) ===
    /// 下载文件
    FileGet(String), // file_id

    // === job 资源 (5个) ===
    /// 创建职务
    JobCreate,
    /// 删除职务
    JobDelete(String), // job_id
    /// 查询单个职务
    JobGet(String), // job_id
    /// 批量查询职务
    JobList,
    /// 更新职务
    JobPatch(String), // job_id

    // === job_change 资源 (1个) ===
    /// 发起员工异动
    JobChangeCreate,

    // === job_data 资源 (5个) ===
    /// 创建任职信息
    JobDataCreate,
    /// 删除任职信息
    JobDataDelete(String), // job_data_id
    /// 查询单个任职信息
    JobDataGet(String), // job_data_id
    /// 批量查询任职信息
    JobDataList,
    /// 更新任职信息
    JobDataPatch(String), // job_data_id

    // === job_family 资源 (5个) ===
    /// 创建序列
    JobFamilyCreate,
    /// 删除序列
    JobFamilyDelete(String), // job_family_id
    /// 查询单个序列
    JobFamilyGet(String), // job_family_id
    /// 批量查询序列
    JobFamilyList,
    /// 更新序列
    JobFamilyPatch(String), // job_family_id

    // === job_level 资源 (5个) ===
    /// 新建职级
    JobLevelCreate,
    /// 删除职级
    JobLevelDelete(String), // job_level_id
    /// 查询单个职级
    JobLevelGet(String), // job_level_id
    /// 批量查询职级
    JobLevelList,
    /// 更新单个职级
    JobLevelPatch(String), // job_level_id

    // === leave 资源 (8个) ===
    /// 根据适用条件获取工作日历ID
    LeaveCalendarByScope,
    /// 批量查询员工假期余额
    LeaveLeaveBalances,
    /// 批量查询员工请假记录
    LeaveLeaveRequestHistory,
    /// 获取假期类型列表
    LeaveLeaveTypes,
    /// 获取工作日历
    LeaveWorkCalendar,
    /// 获取工作日历日期详情
    LeaveWorkCalendarDate,
    /// 创建请假日程
    LeaveRequest,
    /// 删除请假日程
    LeaveRequestDelete(String), // request_id

    // === leave_granting_record 资源 (3个) ===
    /// 创建假期发放记录
    LeaveGrantingRecordCreate,
    /// 删除假期发放记录
    LeaveGrantingRecordDelete(String), // record_id

    // === location 资源 (5个) ===
    /// 创建地点
    LocationCreate,
    /// 删除地点
    LocationDelete(String), // location_id
    /// 查询单个地点
    LocationGet(String), // location_id
    /// 批量分页查询地点信息
    LocationList,
    /// 更新地点
    LocationPatch(String), // location_id

    // === national_id_type 资源 (5个) ===
    /// 创建国家证件类型
    NationalIdTypeCreate,
    /// 删除国家证件类型
    NationalIdTypeDelete(String), // national_id_type_id
    /// 查询单个国家证件类型
    NationalIdTypeGet(String), // national_id_type_id
    /// 批量查询国家证件类型
    NationalIdTypeList,
    /// 更新国家证件类型
    NationalIdTypePatch(String), // national_id_type_id

    // === subdivision 资源 (2个) ===
    /// 查询单条省份/行政区信息
    SubdivisionGet(String), // subdivision_id
    /// 批量查询省份/行政区信息
    SubdivisionList,

    // === subregion 资源 (2个) ===
    /// 查询单条城市/区域信息
    SubregionGet(String), // subregion_id
    /// 批量查询城市/区域信息
    SubregionList,

    // === working_hours_type 资源 (5个) ===
    /// 创建工时制度
    WorkingHoursTypeCreate,
    /// 删除工时制度
    WorkingHoursTypeDelete(String), // working_hours_type_id
    /// 查询单个工时制度
    WorkingHoursTypeGet(String), // working_hours_type_id
    /// 批量查询工时制度
    WorkingHoursTypeList,
    /// 更新工时制度
    WorkingHoursTypePatch(String), // working_hours_type_id

    /// `OffboardingQuery` 变体。
    OffboardingQuery,
    /// `OffboardingSearch` 变体。
    OffboardingSearch,
    /// `OffboardingSubmit` 变体。
    OffboardingSubmit,

    /// `PersonCreate` 变体。
    PersonCreate,
    /// `PersonDelete` 变体。
    PersonDelete(String),
    /// `PersonGet` 变体。
    PersonGet(String),
    /// `PersonPatch` 变体。
    PersonPatch(String),
    /// `PersonUpload` 变体。
    PersonUpload,

    /// `PreHireDelete` 变体。
    PreHireDelete(String),
    /// `PreHireGet` 变体。
    PreHireGet(String),
    /// `PreHireList` 变体。
    PreHireList,
    /// `PreHirePatch` 变体。
    PreHirePatch(String),
    /// 获取流程表单数据
    ProcessFormVariableDataGet,
    /// 查询异动原因
    TransferReasonQuery,
    /// 查询异动类型
    TransferTypeQuery,
    // 注：由于 FeishuPeople API 数量巨大 (257+),
    // 这里只列出部分主要资源，完整实现需要根据实际需求继续添加
    // 主要资源还包括：
    // - pre_hire (预入职)
    // - offboarding (离职)
    // - onboarding (入职)
    // - work_calendar (工作日历)
    // - organization (组织)
    // - user (用户)
    // 等等
}

impl FeishuPeopleApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            // common_data.id
            FeishuPeopleApiV1::CommonDataIdConvert => {
                "/open-apis/corehr/v1/common_data/id/convert".to_string()
            }

            // common_data.meta_data
            FeishuPeopleApiV1::CommonDataMetaDataAddEnumOption => {
                "/open-apis/corehr/v1/common_data/meta_data/add_enum_option".to_string()
            }
            FeishuPeopleApiV1::CommonDataMetaDataEditEnumOption => {
                "/open-apis/corehr/v1/common_data/meta_data/edit_enum_option".to_string()
            }

            // authorization
            FeishuPeopleApiV1::AuthorizationAddRoleAssign => {
                "/open-apis/corehr/v1/authorizations/add_role_assign".to_string()
            }
            FeishuPeopleApiV1::AuthorizationGetByParam => {
                "/open-apis/corehr/v1/authorizations/get_by_param".to_string()
            }
            FeishuPeopleApiV1::AuthorizationQuery => {
                "/open-apis/corehr/v1/authorizations/query".to_string()
            }
            FeishuPeopleApiV1::AuthorizationRemoveRoleAssign => {
                "/open-apis/corehr/v1/authorizations/remove_role_assign".to_string()
            }
            FeishuPeopleApiV1::AuthorizationUpdateRoleAssign => {
                "/open-apis/corehr/v1/authorizations/update_role_assign".to_string()
            }

            // assigned_user
            FeishuPeopleApiV1::AssignedUserSearch => {
                "/open-apis/corehr/v1/assigned_users/search".to_string()
            }

            FeishuPeopleApiV1::SecurityGroupList => {
                "/open-apis/corehr/v1/security_groups".to_string()
            }
            FeishuPeopleApiV1::SecurityGroupQuery => {
                "/open-apis/corehr/v1/security_groups/query".to_string()
            }

            // approval_groups
            FeishuPeopleApiV1::ApprovalGroupsGet => {
                "/open-apis/corehr/v1/approval_groups/get".to_string()
            }
            FeishuPeopleApiV1::ApprovalGroupsOpenQueryDepartmentChangeListByIds => {
                "/open-apis/corehr/v1/approval_groups/open_query_department_change_list_by_ids"
                    .to_string()
            }
            FeishuPeopleApiV1::ApprovalGroupsOpenQueryJobChangeListByIds => {
                "/open-apis/corehr/v1/approval_groups/open_query_job_change_list_by_ids".to_string()
            }
            FeishuPeopleApiV1::ApprovalGroupsOpenQueryPositionChangeListByIds => {
                "/open-apis/corehr/v1/approval_groups/open_query_position_change_list_by_ids"
                    .to_string()
            }

            // approver
            FeishuPeopleApiV1::ApproverList => "/open-apis/corehr/v1/approvers/list".to_string(),

            // basic_info.*
            FeishuPeopleApiV1::BasicInfoBankSearch => {
                "/open-apis/corehr/v1/basic_info.banks/search".to_string()
            }
            FeishuPeopleApiV1::BasicInfoBankBranchSearch => {
                "/open-apis/corehr/v1/basic_info.bank_branches/search".to_string()
            }
            FeishuPeopleApiV1::BasicInfoCitySearch => {
                "/open-apis/corehr/v1/basic_info.cities/search".to_string()
            }
            FeishuPeopleApiV1::BasicInfoCountryRegionSearch => {
                "/open-apis/corehr/v1/basic_info.country_regions/search".to_string()
            }
            FeishuPeopleApiV1::BasicInfoCountryRegionSubdivisionSearch => {
                "/open-apis/corehr/v1/basic_info.country_region_subdivisions/search".to_string()
            }
            FeishuPeopleApiV1::BasicInfoCurrencySearch => {
                "/open-apis/corehr/v1/basic_info.currencies/search".to_string()
            }
            FeishuPeopleApiV1::BasicInfoDistrictSearch => {
                "/open-apis/corehr/v1/basic_info.districts/search".to_string()
            }
            FeishuPeopleApiV1::BasicInfoLanguageSearch => {
                "/open-apis/corehr/v1/basic_info.languages/search".to_string()
            }
            FeishuPeopleApiV1::BasicInfoNationalitySearch => {
                "/open-apis/corehr/v1/basic_info.nationalities/search".to_string()
            }
            FeishuPeopleApiV1::BasicInfoTimeZoneSearch => {
                "/open-apis/corehr/v1/basic_info.time_zones/search".to_string()
            }

            // bp
            FeishuPeopleApiV1::BpGetByDepartment => {
                "/open-apis/corehr/v1/bps/get_by_department".to_string()
            }
            FeishuPeopleApiV1::BpList => "/open-apis/corehr/v1/bps/list".to_string(),

            // company
            FeishuPeopleApiV1::CompanyCreate => "/open-apis/corehr/v1/companies".to_string(),
            FeishuPeopleApiV1::CompanyDelete(company_id) => {
                format!("/open-apis/corehr/v1/companies/{company_id}")
            }
            FeishuPeopleApiV1::CompanyGet(company_id) => {
                format!("/open-apis/corehr/v1/companies/{company_id}")
            }
            FeishuPeopleApiV1::CompanyList => "/open-apis/corehr/v1/companies".to_string(),
            FeishuPeopleApiV1::CompanyPatch(company_id) => {
                format!("/open-apis/corehr/v1/companies/{company_id}")
            }
            FeishuPeopleApiV1::CompanyBatchGet => {
                "/open-apis/corehr/v1/companies/batch_get".to_string()
            }
            FeishuPeopleApiV1::CompanyActive(company_id) => {
                format!("/open-apis/corehr/v1/companies/{company_id}/active")
            }
            FeishuPeopleApiV1::CompanyQueryRecentChange => {
                "/open-apis/corehr/v1/companies/query_recent_change".to_string()
            }

            // compensation_standard
            FeishuPeopleApiV1::CompensationStandardMatch => {
                "/open-apis/corehr/v1/compensation_standards/match".to_string()
            }

            // contract
            FeishuPeopleApiV1::ContractCreate => "/open-apis/corehr/v1/contracts".to_string(),
            FeishuPeopleApiV1::ContractDelete(contract_id) => {
                format!("/open-apis/corehr/v1/contracts/{contract_id}")
            }
            FeishuPeopleApiV1::ContractGet(contract_id) => {
                format!("/open-apis/corehr/v1/contracts/{contract_id}")
            }
            FeishuPeopleApiV1::ContractList => "/open-apis/corehr/v1/contracts".to_string(),
            FeishuPeopleApiV1::ContractPatch(contract_id) => {
                format!("/open-apis/corehr/v1/contracts/{contract_id}")
            }
            FeishuPeopleApiV1::ContractSearch => {
                "/open-apis/corehr/v1/contracts/search".to_string()
            }

            // cost_center
            FeishuPeopleApiV1::CostCenterCreate => "/open-apis/corehr/v1/cost_centers".to_string(),
            FeishuPeopleApiV1::CostCenterDelete(cost_center_id) => {
                format!("/open-apis/corehr/v1/cost_centers/{cost_center_id}")
            }
            FeishuPeopleApiV1::CostCenterPatch(cost_center_id) => {
                format!("/open-apis/corehr/v1/cost_centers/{cost_center_id}")
            }
            FeishuPeopleApiV1::CostCenterSearch => {
                "/open-apis/corehr/v1/cost_centers/search".to_string()
            }
            FeishuPeopleApiV1::CostCenterQueryRecentChange => {
                "/open-apis/corehr/v1/cost_centers/query_recent_change".to_string()
            }
            FeishuPeopleApiV1::CostCenterVersionCreate => {
                "/open-apis/corehr/v1/cost_center_versions".to_string()
            }
            FeishuPeopleApiV1::CostCenterVersionDelete(version_id) => {
                format!("/open-apis/corehr/v1/cost_center_versions/{version_id}")
            }
            FeishuPeopleApiV1::CostCenterVersionPatch(version_id) => {
                format!("/open-apis/corehr/v1/cost_center_versions/{version_id}")
            }
            FeishuPeopleApiV1::CostCenterBatchQuery => {
                "/open-apis/corehr/v1/cost_centers/batch_query".to_string()
            }

            // country_region
            FeishuPeopleApiV1::CountryRegionGet(country_region_id) => {
                format!("/open-apis/corehr/v1/country_regions/{country_region_id}")
            }
            FeishuPeopleApiV1::CountryRegionList => {
                "/open-apis/corehr/v1/country_regions".to_string()
            }

            // currency
            FeishuPeopleApiV1::CurrencyGet(currency_id) => {
                format!("/open-apis/corehr/v1/currencies/{currency_id}")
            }
            FeishuPeopleApiV1::CurrencyList => "/open-apis/corehr/v1/currencies".to_string(),

            // custom_field
            FeishuPeopleApiV1::CustomFieldGetByParam => {
                "/open-apis/corehr/v1/custom_fields/get_by_param".to_string()
            }
            FeishuPeopleApiV1::CustomFieldListObjectName => {
                "/open-apis/corehr/v1/custom_fields/list_object_api_name".to_string()
            }
            FeishuPeopleApiV1::CustomFieldQuery => {
                "/open-apis/corehr/v1/custom_fields/query".to_string()
            }

            // custom_org
            FeishuPeopleApiV1::CustomOrgCreate => "/open-apis/corehr/v1/custom_orgs".to_string(),
            FeishuPeopleApiV1::CustomOrgDeleteOrg(org_id) => {
                format!("/open-apis/corehr/v1/custom_orgs/{org_id}")
            }
            FeishuPeopleApiV1::CustomOrgPatch(org_id) => {
                format!("/open-apis/corehr/v1/custom_orgs/{org_id}")
            }
            FeishuPeopleApiV1::CustomOrgQuery => {
                "/open-apis/corehr/v1/custom_orgs/query".to_string()
            }
            FeishuPeopleApiV1::CustomOrgActive(org_id) => {
                format!("/open-apis/corehr/v1/custom_orgs/{org_id}/active")
            }
            FeishuPeopleApiV1::CustomOrgQueryRecentChange => {
                "/open-apis/corehr/v1/custom_orgs/query_recent_change".to_string()
            }
            FeishuPeopleApiV1::CustomOrgUpdateRule => {
                "/open-apis/corehr/v1/custom_orgs/update_rule".to_string()
            }

            // default_cost_center
            FeishuPeopleApiV1::DefaultCostCenterCreateVersion => {
                "/open-apis/corehr/v1/default_cost_centers/create_version".to_string()
            }
            FeishuPeopleApiV1::DefaultCostCenterRemoveVersion(version_id) => {
                format!("/open-apis/corehr/v1/default_cost_centers/{version_id}")
            }
            FeishuPeopleApiV1::DefaultCostCenterUpdateVersion(version_id) => {
                format!("/open-apis/corehr/v1/default_cost_centers/{version_id}")
            }
            FeishuPeopleApiV1::DefaultCostCenterBatchQuery => {
                "/open-apis/corehr/v1/default_cost_centers/batch_query".to_string()
            }

            // department
            FeishuPeopleApiV1::DepartmentCreate => "/open-apis/corehr/v1/departments".to_string(),
            FeishuPeopleApiV1::DepartmentDelete(department_id) => {
                format!("/open-apis/corehr/v1/departments/{department_id}")
            }
            FeishuPeopleApiV1::DepartmentGet(department_id) => {
                format!("/open-apis/corehr/v1/departments/{department_id}")
            }
            FeishuPeopleApiV1::DepartmentList => "/open-apis/corehr/v1/departments".to_string(),
            FeishuPeopleApiV1::DepartmentPatch(department_id) => {
                format!("/open-apis/corehr/v1/departments/{department_id}")
            }
            FeishuPeopleApiV1::DepartmentSearch => {
                "/open-apis/corehr/v1/departments/search".to_string()
            }
            FeishuPeopleApiV1::DepartmentBatchGet => {
                "/open-apis/corehr/v1/departments/batch_get".to_string()
            }
            FeishuPeopleApiV1::DepartmentParents(department_id) => {
                format!("/open-apis/corehr/v1/departments/{department_id}/parents")
            }
            FeishuPeopleApiV1::DepartmentTree => {
                "/open-apis/corehr/v1/departments/tree".to_string()
            }
            FeishuPeopleApiV1::DepartmentQueryTimeline => {
                "/open-apis/corehr/v1/departments/query_timeline".to_string()
            }
            FeishuPeopleApiV1::DepartmentQueryMultiTimeline => {
                "/open-apis/corehr/v1/departments/query_multi_timeline".to_string()
            }
            FeishuPeopleApiV1::DepartmentQueryOperationLogs => {
                "/open-apis/corehr/v1/departments/query_operation_logs".to_string()
            }
            FeishuPeopleApiV1::DepartmentQueryRecentChange => {
                "/open-apis/corehr/v1/departments/query_recent_change".to_string()
            }

            // draft
            FeishuPeopleApiV1::DraftGet(draft_id) => {
                format!("/open-apis/corehr/v1/drafts/{draft_id}")
            }

            // employee
            FeishuPeopleApiV1::EmployeeCreate => "/open-apis/corehr/v1/employees".to_string(),
            FeishuPeopleApiV1::EmployeeDelete(employee_id) => {
                format!("/open-apis/corehr/v1/employees/{employee_id}")
            }
            FeishuPeopleApiV1::EmployeeBatchGet => {
                "/open-apis/corehr/v1/employees/batch_get".to_string()
            }
            FeishuPeopleApiV1::EmployeeList => "/open-apis/corehr/v1/employees".to_string(),
            FeishuPeopleApiV1::EmployeeSearch => {
                "/open-apis/corehr/v1/employees/search".to_string()
            }

            // employee_type
            FeishuPeopleApiV1::EmployeeTypeCreate => {
                "/open-apis/corehr/v1/employee_types".to_string()
            }
            FeishuPeopleApiV1::EmployeeTypeDelete(employee_type_id) => {
                format!("/open-apis/corehr/v1/employee_types/{employee_type_id}")
            }
            FeishuPeopleApiV1::EmployeeTypeGet(employee_type_id) => {
                format!("/open-apis/corehr/v1/employee_types/{employee_type_id}")
            }
            FeishuPeopleApiV1::EmployeeTypeList => {
                "/open-apis/corehr/v1/employee_types".to_string()
            }
            FeishuPeopleApiV1::EmployeeTypePatch(employee_type_id) => {
                format!("/open-apis/corehr/v1/employee_types/{employee_type_id}")
            }

            // employees.additional_job
            FeishuPeopleApiV1::EmployeesAdditionalJobCreate => {
                "/open-apis/corehr/v1/employees.additional_jobs".to_string()
            }
            FeishuPeopleApiV1::EmployeesAdditionalJobDelete(additional_job_id) => {
                format!("/open-apis/corehr/v1/employees.additional_jobs/{additional_job_id}")
            }
            FeishuPeopleApiV1::EmployeesAdditionalJobPatch(additional_job_id) => {
                format!("/open-apis/corehr/v1/employees.additional_jobs/{additional_job_id}")
            }
            FeishuPeopleApiV1::EmployeesAdditionalJobBatch => {
                "/open-apis/corehr/v1/employees.additional_jobs/batch".to_string()
            }

            // employees.bp
            FeishuPeopleApiV1::EmployeesBpBatchGet => {
                "/open-apis/corehr/v1/employees.bps/batch_get".to_string()
            }

            // employment
            FeishuPeopleApiV1::EmploymentCreate => "/open-apis/corehr/v1/employments".to_string(),
            FeishuPeopleApiV1::EmploymentDelete(employment_id) => {
                format!("/open-apis/corehr/v1/employments/{employment_id}")
            }
            FeishuPeopleApiV1::EmploymentPatch(employment_id) => {
                format!("/open-apis/corehr/v1/employments/{employment_id}")
            }

            // file
            FeishuPeopleApiV1::FileGet(file_id) => {
                format!("/open-apis/corehr/v1/files/{file_id}")
            }

            // job
            FeishuPeopleApiV1::JobCreate => "/open-apis/corehr/v1/jobs".to_string(),
            FeishuPeopleApiV1::JobDelete(job_id) => {
                format!("/open-apis/corehr/v1/jobs/{job_id}")
            }
            FeishuPeopleApiV1::JobGet(job_id) => {
                format!("/open-apis/corehr/v1/jobs/{job_id}")
            }
            FeishuPeopleApiV1::JobList => "/open-apis/corehr/v1/jobs".to_string(),
            FeishuPeopleApiV1::JobPatch(job_id) => {
                format!("/open-apis/corehr/v1/jobs/{job_id}")
            }

            // job_change
            FeishuPeopleApiV1::JobChangeCreate => "/open-apis/corehr/v1/job_changes".to_string(),

            // job_data
            FeishuPeopleApiV1::JobDataCreate => "/open-apis/corehr/v1/job_datas".to_string(),
            FeishuPeopleApiV1::JobDataDelete(job_data_id) => {
                format!("/open-apis/corehr/v1/job_datas/{job_data_id}")
            }
            FeishuPeopleApiV1::JobDataGet(job_data_id) => {
                format!("/open-apis/corehr/v1/job_datas/{job_data_id}")
            }
            FeishuPeopleApiV1::JobDataList => "/open-apis/corehr/v1/job_datas".to_string(),
            FeishuPeopleApiV1::JobDataPatch(job_data_id) => {
                format!("/open-apis/corehr/v1/job_datas/{job_data_id}")
            }

            // job_family
            FeishuPeopleApiV1::JobFamilyCreate => "/open-apis/corehr/v1/job_families".to_string(),
            FeishuPeopleApiV1::JobFamilyDelete(job_family_id) => {
                format!("/open-apis/corehr/v1/job_families/{job_family_id}")
            }
            FeishuPeopleApiV1::JobFamilyGet(job_family_id) => {
                format!("/open-apis/corehr/v1/job_families/{job_family_id}")
            }
            FeishuPeopleApiV1::JobFamilyList => "/open-apis/corehr/v1/job_families".to_string(),
            FeishuPeopleApiV1::JobFamilyPatch(job_family_id) => {
                format!("/open-apis/corehr/v1/job_families/{job_family_id}")
            }

            // job_level
            FeishuPeopleApiV1::JobLevelCreate => "/open-apis/corehr/v1/job_levels".to_string(),
            FeishuPeopleApiV1::JobLevelDelete(job_level_id) => {
                format!("/open-apis/corehr/v1/job_levels/{job_level_id}")
            }
            FeishuPeopleApiV1::JobLevelGet(job_level_id) => {
                format!("/open-apis/corehr/v1/job_levels/{job_level_id}")
            }
            FeishuPeopleApiV1::JobLevelList => "/open-apis/corehr/v1/job_levels".to_string(),
            FeishuPeopleApiV1::JobLevelPatch(job_level_id) => {
                format!("/open-apis/corehr/v1/job_levels/{job_level_id}")
            }

            // leave
            FeishuPeopleApiV1::LeaveCalendarByScope => {
                "/open-apis/corehr/v1/leaves/calendar_by_scope".to_string()
            }
            FeishuPeopleApiV1::LeaveLeaveBalances => {
                "/open-apis/corehr/v1/leaves/leave_balances".to_string()
            }
            FeishuPeopleApiV1::LeaveLeaveRequestHistory => {
                "/open-apis/corehr/v1/leaves/leave_request_history".to_string()
            }
            FeishuPeopleApiV1::LeaveLeaveTypes => {
                "/open-apis/corehr/v1/leaves/leave_types".to_string()
            }
            FeishuPeopleApiV1::LeaveWorkCalendar => {
                "/open-apis/corehr/v1/leaves/work_calendar".to_string()
            }
            FeishuPeopleApiV1::LeaveWorkCalendarDate => {
                "/open-apis/corehr/v1/leaves/work_calendar_date".to_string()
            }
            FeishuPeopleApiV1::LeaveRequest => "/open-apis/corehr/v1/leave_requests".to_string(),
            FeishuPeopleApiV1::LeaveRequestDelete(request_id) => {
                format!("/open-apis/corehr/v1/leave_requests/{request_id}")
            }

            // leave_granting_record
            FeishuPeopleApiV1::LeaveGrantingRecordCreate => {
                "/open-apis/corehr/v1/leave_granting_records".to_string()
            }
            FeishuPeopleApiV1::LeaveGrantingRecordDelete(record_id) => {
                format!("/open-apis/corehr/v1/leave_granting_records/{record_id}")
            }

            // location
            FeishuPeopleApiV1::LocationCreate => "/open-apis/corehr/v1/locations".to_string(),
            FeishuPeopleApiV1::LocationDelete(location_id) => {
                format!("/open-apis/corehr/v1/locations/{location_id}")
            }
            FeishuPeopleApiV1::LocationGet(location_id) => {
                format!("/open-apis/corehr/v1/locations/{location_id}")
            }
            FeishuPeopleApiV1::LocationList => "/open-apis/corehr/v1/locations".to_string(),
            FeishuPeopleApiV1::LocationPatch(location_id) => {
                format!("/open-apis/corehr/v1/locations/{location_id}")
            }

            // national_id_type
            FeishuPeopleApiV1::NationalIdTypeCreate => {
                "/open-apis/corehr/v1/national_id_types".to_string()
            }
            FeishuPeopleApiV1::NationalIdTypeDelete(national_id_type_id) => {
                format!("/open-apis/corehr/v1/national_id_types/{national_id_type_id}")
            }
            FeishuPeopleApiV1::NationalIdTypeGet(national_id_type_id) => {
                format!("/open-apis/corehr/v1/national_id_types/{national_id_type_id}")
            }
            FeishuPeopleApiV1::NationalIdTypeList => {
                "/open-apis/corehr/v1/national_id_types".to_string()
            }
            FeishuPeopleApiV1::NationalIdTypePatch(national_id_type_id) => {
                format!("/open-apis/corehr/v1/national_id_types/{national_id_type_id}")
            }

            // subdivision
            FeishuPeopleApiV1::SubdivisionGet(subdivision_id) => {
                format!("/open-apis/corehr/v1/subdivisions/{subdivision_id}")
            }
            FeishuPeopleApiV1::SubdivisionList => "/open-apis/corehr/v1/subdivisions".to_string(),

            // subregion
            FeishuPeopleApiV1::SubregionGet(subregion_id) => {
                format!("/open-apis/corehr/v1/subregions/{subregion_id}")
            }
            FeishuPeopleApiV1::SubregionList => "/open-apis/corehr/v1/subregions".to_string(),

            // working_hours_type
            FeishuPeopleApiV1::WorkingHoursTypeCreate => {
                "/open-apis/corehr/v1/working_hours_types".to_string()
            }
            FeishuPeopleApiV1::WorkingHoursTypeDelete(working_hours_type_id) => {
                format!("/open-apis/corehr/v1/working_hours_types/{working_hours_type_id}")
            }
            FeishuPeopleApiV1::WorkingHoursTypeGet(working_hours_type_id) => {
                format!("/open-apis/corehr/v1/working_hours_types/{working_hours_type_id}")
            }
            FeishuPeopleApiV1::WorkingHoursTypeList => {
                "/open-apis/corehr/v1/working_hours_types".to_string()
            }
            FeishuPeopleApiV1::WorkingHoursTypePatch(working_hours_type_id) => {
                format!("/open-apis/corehr/v1/working_hours_types/{working_hours_type_id}")
            }

            // offboarding
            FeishuPeopleApiV1::OffboardingQuery => {
                "/open-apis/corehr/v1/offboardings/query".to_string()
            }
            FeishuPeopleApiV1::OffboardingSearch => {
                "/open-apis/corehr/v1/offboardings/search".to_string()
            }
            FeishuPeopleApiV1::OffboardingSubmit => {
                "/open-apis/corehr/v1/offboardings/submit".to_string()
            }

            FeishuPeopleApiV1::PersonCreate => "/open-apis/corehr/v1/persons".to_string(),
            FeishuPeopleApiV1::PersonDelete(person_id) => {
                format!("/open-apis/corehr/v1/persons/{person_id}")
            }
            FeishuPeopleApiV1::PersonGet(person_id) => {
                format!("/open-apis/corehr/v1/persons/{person_id}")
            }
            FeishuPeopleApiV1::PersonPatch(person_id) => {
                format!("/open-apis/corehr/v1/persons/{person_id}")
            }
            FeishuPeopleApiV1::PersonUpload => "/open-apis/corehr/v1/persons/upload".to_string(),

            FeishuPeopleApiV1::PreHireDelete(pre_hire_id) => {
                format!("/open-apis/corehr/v1/pre_hires/{pre_hire_id}")
            }
            FeishuPeopleApiV1::PreHireGet(pre_hire_id) => {
                format!("/open-apis/corehr/v1/pre_hires/{pre_hire_id}")
            }
            FeishuPeopleApiV1::PreHireList => "/open-apis/corehr/v1/pre_hires".to_string(),
            FeishuPeopleApiV1::PreHirePatch(pre_hire_id) => {
                format!("/open-apis/corehr/v1/pre_hires/{pre_hire_id}")
            }
            FeishuPeopleApiV1::ProcessFormVariableDataGet => {
                "/open-apis/corehr/v1/processes/{}/form_variable_data".to_string()
            }
            FeishuPeopleApiV1::TransferReasonQuery => {
                "/open-apis/corehr/v1/transfer_reasons/query".to_string()
            }
            FeishuPeopleApiV1::TransferTypeQuery => {
                "/open-apis/corehr/v1/transfer_types/query".to_string()
            }
        }
    }
}

// FeishuPeople API V2 - 核心人力资源 V2 (14 APIs)

/// FeishuPeople API V2 端点枚举（核心人力资源 V2）
#[derive(Debug, Clone, PartialEq)]
pub enum FeishuPeopleApiV2 {
    // === company 资源 (1个) ===
    /// 启用/停用公司
    CompanyActive(String), // company_id

    // === contract 资源 (1个) ===
    /// 搜索合同
    ContractSearch,

    // === department 资源 (7个) ===
    /// 批量查询部门
    DepartmentBatchGet,
    /// 批量查询部门版本信息
    DepartmentQueryMultiTimeline,
    /// 批量查询部门操作日志
    DepartmentQueryOperationLogs,
    /// 获取父部门信息
    DepartmentParents(String), // department_id
    /// 搜索部门信息
    DepartmentSearch,
    /// 查询指定生效日期的部门基本信息
    DepartmentQueryTimeline,
    /// 查询指定生效日期的部门架构树
    DepartmentTree,

    // === employee 资源 (3个) ===
    /// 批量查询员工信息
    EmployeeBatchGet,
    /// 添加人员
    EmployeeCreate,
    /// 搜索员工信息
    EmployeeSearch,

    /// `EmployeesAdditionalJobBatch` 变体。
    EmployeesAdditionalJobBatch,
    /// `EmployeesAdditionalJobCreate` 变体。
    EmployeesAdditionalJobCreate,
    /// `EmployeesAdditionalJobDelete` 变体。
    EmployeesAdditionalJobDelete(String),
    /// `EmployeesAdditionalJobPatch` 变体。
    EmployeesAdditionalJobPatch(String),

    /// `EmployeesBpBatchGet` 变体。
    EmployeesBpBatchGet,

    /// `EmployeesInternationalAssignmentCreate` 变体。
    EmployeesInternationalAssignmentCreate,
    /// `EmployeesInternationalAssignmentDelete` 变体。
    EmployeesInternationalAssignmentDelete(String),
    /// `EmployeesInternationalAssignmentList` 变体。
    EmployeesInternationalAssignmentList,
    /// `EmployeesInternationalAssignmentPatch` 变体。
    EmployeesInternationalAssignmentPatch(String),

    /// `EmployeesJobDataBatchGet` 变体。
    EmployeesJobDataBatchGet,
    /// `EmployeesJobDataQuery` 变体。
    EmployeesJobDataQuery,

    /// `PersonCreate` 变体。
    PersonCreate,
    /// `PersonPatch` 变体。
    PersonPatch(String),

    /// `PreHireComplete` 变体。
    PreHireComplete,
    /// `PreHireCreate` 变体。
    PreHireCreate,
    /// `PreHireDelete` 变体。
    PreHireDelete(String),
    /// `PreHirePatch` 变体。
    PreHirePatch(String),
    /// `PreHireQuery` 变体。
    PreHireQuery,
    /// `PreHireRestoreFlowInstance` 变体。
    PreHireRestoreFlowInstance,
    /// `PreHireSearch` 变体。
    PreHireSearch,
    /// `PreHireTransformOnboardingTask` 变体。
    PreHireTransformOnboardingTask,
    /// `PreHireTransitTask` 变体。
    PreHireTransitTask,
    /// `PreHireWithdrawOnboarding` 变体。
    PreHireWithdrawOnboarding,

    /// `OffboardingEdit` 变体。
    OffboardingEdit,
    /// `OffboardingRevoke` 变体。
    OffboardingRevoke,
    /// `OffboardingSubmitV2` 变体。
    OffboardingSubmitV2,

    /// `ProbationEnableDisableAssessment` 变体。
    ProbationEnableDisableAssessment,
    /// `ProbationSearch` 变体。
    ProbationSearch,
    /// `ProbationSubmit` 变体。
    ProbationSubmit,
    /// `ProbationWithdraw` 变体。
    ProbationWithdraw,
    /// `ProbationAssessmentCreate` 变体。
    ProbationAssessmentCreate,
    /// `ProbationAssessmentPatch` 变体。
    ProbationAssessmentPatch(String), // assessment_id
    /// `ProbationAssessmentDelete` 变体。
    ProbationAssessmentDelete(String), // assessment_id

    // === location 资源 (1个) ===
    /// 更新地点
    LocationPatch(String), // location_id

    // === basic_info 资源 (10个) ===
    /// 查询银行信息
    BasicInfoBankSearch,
    /// 查询支行信息
    BasicInfoBankBranchSearch,
    /// 查询城市信息
    BasicInfoCitySearch,
    /// 查询国家/地区信息
    BasicInfoCountryRegionSearch,
    /// 查询省份/主要行政区信息
    BasicInfoCountryRegionSubdivisionSearch,
    /// 查询货币信息
    BasicInfoCurrencySearch,
    /// 查询区/县信息
    BasicInfoDistrictSearch,
    /// 查询语言信息
    BasicInfoLanguageSearch,
    /// 查询国籍信息
    BasicInfoNationalitySearch,
    /// 查询时区信息
    BasicInfoTimeZoneSearch,
}

impl FeishuPeopleApiV2 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            // company
            FeishuPeopleApiV2::CompanyActive(company_id) => {
                format!("/open-apis/corehr/v2/companies/{company_id}/active")
            }

            // contract
            FeishuPeopleApiV2::ContractSearch => {
                "/open-apis/corehr/v2/contracts/search".to_string()
            }

            // department
            FeishuPeopleApiV2::DepartmentBatchGet => {
                "/open-apis/corehr/v2/departments/batch_get".to_string()
            }
            FeishuPeopleApiV2::DepartmentQueryMultiTimeline => {
                "/open-apis/corehr/v2/departments/query_multi_timeline".to_string()
            }
            FeishuPeopleApiV2::DepartmentQueryOperationLogs => {
                "/open-apis/corehr/v2/departments/query_operation_logs".to_string()
            }
            FeishuPeopleApiV2::DepartmentParents(_) => {
                "/open-apis/corehr/v2/departments/parents".to_string()
            }
            FeishuPeopleApiV2::DepartmentSearch => {
                "/open-apis/corehr/v2/departments/search".to_string()
            }
            FeishuPeopleApiV2::DepartmentQueryTimeline => {
                "/open-apis/corehr/v2/departments/query_timeline".to_string()
            }
            FeishuPeopleApiV2::DepartmentTree => {
                "/open-apis/corehr/v2/departments/tree".to_string()
            }

            // employee
            FeishuPeopleApiV2::EmployeeBatchGet => {
                "/open-apis/corehr/v2/employees/batch_get".to_string()
            }
            FeishuPeopleApiV2::EmployeeCreate => "/open-apis/corehr/v2/employees".to_string(),
            FeishuPeopleApiV2::EmployeeSearch => {
                "/open-apis/corehr/v2/employees/search".to_string()
            }

            FeishuPeopleApiV2::EmployeesAdditionalJobBatch => {
                "/open-apis/corehr/v2/employees/additional_jobs/batch".to_string()
            }
            FeishuPeopleApiV2::EmployeesAdditionalJobCreate => {
                "/open-apis/corehr/v2/employees/additional_jobs".to_string()
            }
            FeishuPeopleApiV2::EmployeesAdditionalJobDelete(additional_job_id) => {
                format!("/open-apis/corehr/v2/employees/additional_jobs/{additional_job_id}")
            }
            FeishuPeopleApiV2::EmployeesAdditionalJobPatch(additional_job_id) => {
                format!("/open-apis/corehr/v2/employees/additional_jobs/{additional_job_id}")
            }

            FeishuPeopleApiV2::EmployeesBpBatchGet => {
                "/open-apis/corehr/v2/employees/bps/batch_get".to_string()
            }

            FeishuPeopleApiV2::EmployeesInternationalAssignmentCreate => {
                "/open-apis/corehr/v2/employees/international_assignments".to_string()
            }
            FeishuPeopleApiV2::EmployeesInternationalAssignmentDelete(
                international_assignment_id,
            ) => {
                format!(
                    "/open-apis/corehr/v2/employees/international_assignments/{international_assignment_id}"
                )
            }
            FeishuPeopleApiV2::EmployeesInternationalAssignmentList => {
                "/open-apis/corehr/v2/employees/international_assignments".to_string()
            }
            FeishuPeopleApiV2::EmployeesInternationalAssignmentPatch(
                international_assignment_id,
            ) => {
                format!(
                    "/open-apis/corehr/v2/employees/international_assignments/{international_assignment_id}"
                )
            }

            FeishuPeopleApiV2::EmployeesJobDataBatchGet => {
                "/open-apis/corehr/v2/employees/job_datas/batch_get".to_string()
            }
            FeishuPeopleApiV2::EmployeesJobDataQuery => {
                "/open-apis/corehr/v2/employees/job_datas/query".to_string()
            }

            FeishuPeopleApiV2::PersonCreate => "/open-apis/corehr/v2/persons".to_string(),
            FeishuPeopleApiV2::PersonPatch(person_id) => {
                format!("/open-apis/corehr/v2/persons/{person_id}")
            }

            FeishuPeopleApiV2::PreHireComplete => {
                "/open-apis/corehr/v2/pre_hires/complete".to_string()
            }
            FeishuPeopleApiV2::PreHireCreate => "/open-apis/corehr/v2/pre_hires".to_string(),
            FeishuPeopleApiV2::PreHireDelete(pre_hire_id) => {
                format!("/open-apis/corehr/v2/pre_hires/{pre_hire_id}")
            }
            FeishuPeopleApiV2::PreHirePatch(pre_hire_id) => {
                format!("/open-apis/corehr/v2/pre_hires/{pre_hire_id}")
            }
            FeishuPeopleApiV2::PreHireQuery => "/open-apis/corehr/v2/pre_hires/query".to_string(),
            FeishuPeopleApiV2::PreHireRestoreFlowInstance => {
                "/open-apis/corehr/v2/pre_hires/restore_flow_instance".to_string()
            }
            FeishuPeopleApiV2::PreHireSearch => "/open-apis/corehr/v2/pre_hires/search".to_string(),
            FeishuPeopleApiV2::PreHireTransformOnboardingTask => {
                "/open-apis/corehr/v2/pre_hires/transform_onboarding_task".to_string()
            }
            FeishuPeopleApiV2::PreHireTransitTask => {
                "/open-apis/corehr/v2/pre_hires/transit_task".to_string()
            }
            FeishuPeopleApiV2::PreHireWithdrawOnboarding => {
                "/open-apis/corehr/v2/pre_hires/withdraw_onboarding".to_string()
            }

            // location
            FeishuPeopleApiV2::LocationPatch(location_id) => {
                format!("/open-apis/corehr/v2/locations/{location_id}")
            }

            FeishuPeopleApiV2::BasicInfoBankSearch => {
                "/open-apis/corehr/v2/basic_info/banks/search".to_string()
            }
            FeishuPeopleApiV2::BasicInfoBankBranchSearch => {
                "/open-apis/corehr/v2/basic_info/bank_branchs/search".to_string()
            }
            FeishuPeopleApiV2::BasicInfoCitySearch => {
                "/open-apis/corehr/v2/basic_info/cities/search".to_string()
            }
            FeishuPeopleApiV2::BasicInfoCountryRegionSearch => {
                "/open-apis/corehr/v2/basic_info/country_regions/search".to_string()
            }
            FeishuPeopleApiV2::BasicInfoCountryRegionSubdivisionSearch => {
                "/open-apis/corehr/v2/basic_info/country_region_subdivisions/search".to_string()
            }
            FeishuPeopleApiV2::BasicInfoCurrencySearch => {
                "/open-apis/corehr/v2/basic_info/currencies/search".to_string()
            }
            FeishuPeopleApiV2::BasicInfoDistrictSearch => {
                "/open-apis/corehr/v2/basic_info/districts/search".to_string()
            }
            FeishuPeopleApiV2::BasicInfoLanguageSearch => {
                "/open-apis/corehr/v2/basic_info/languages/search".to_string()
            }
            FeishuPeopleApiV2::BasicInfoNationalitySearch => {
                "/open-apis/corehr/v2/basic_info/nationalities/search".to_string()
            }
            FeishuPeopleApiV2::BasicInfoTimeZoneSearch => {
                "/open-apis/corehr/v2/basic_info/time_zones/search".to_string()
            }

            FeishuPeopleApiV2::OffboardingEdit => {
                "/open-apis/corehr/v2/offboardings/edit".to_string()
            }
            FeishuPeopleApiV2::OffboardingRevoke => {
                "/open-apis/corehr/v2/offboardings/revoke".to_string()
            }
            FeishuPeopleApiV2::OffboardingSubmitV2 => {
                "/open-apis/corehr/v2/offboardings/submit_v2".to_string()
            }

            FeishuPeopleApiV2::ProbationEnableDisableAssessment => {
                "/open-apis/corehr/v2/probation/enable_disable_assessment".to_string()
            }
            FeishuPeopleApiV2::ProbationSearch => {
                "/open-apis/corehr/v2/probation/search".to_string()
            }
            FeishuPeopleApiV2::ProbationSubmit => {
                "/open-apis/corehr/v2/probation/submit".to_string()
            }
            FeishuPeopleApiV2::ProbationWithdraw => {
                "/open-apis/corehr/v2/probation/withdraw".to_string()
            }
            FeishuPeopleApiV2::ProbationAssessmentCreate => {
                "/open-apis/corehr/v2/probation/assessments".to_string()
            }
            FeishuPeopleApiV2::ProbationAssessmentPatch(assessment_id) => {
                format!("/open-apis/corehr/v2/probation/assessments/{assessment_id}")
            }
            FeishuPeopleApiV2::ProbationAssessmentDelete(assessment_id) => {
                format!("/open-apis/corehr/v2/probation/assessments/{assessment_id}")
            }
        }
    }
}

/// `CorehrApiV1` 类型别名。
pub type CorehrApiV1 = FeishuPeopleApiV1;

/// `CorehrApiV2` 类型别名。
pub type CorehrApiV2 = FeishuPeopleApiV2;

// 测试模块

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_attendance_api_urls() {
        let url = AttendanceApiV1::GroupList.to_url();
        assert_eq!(url, "/open-apis/attendance/v1/groups");

        let url = AttendanceApiV1::GroupGet("123".to_string()).to_url();
        assert_eq!(url, "/open-apis/attendance/v1/groups/123");
    }

    #[test]
    fn test_hire_api_urls() {
        let url = HireApiV1::TalentList.to_url();
        assert_eq!(url, "/open-apis/hire/v1/talents");

        let url = HireApiV1::TalentGet("456".to_string()).to_url();
        assert_eq!(url, "/open-apis/hire/v1/talents/456");
    }

    #[test]
    fn test_compensation_api_urls() {
        let url = CompensationApiV1::ArchiveQuery.to_url();
        assert_eq!(url, "/open-apis/compensation/v1/archives/query");
    }

    #[test]
    fn test_performance_api_urls() {
        let url = PerformanceApiV1::ActivityQuery.to_url();
        assert_eq!(url, "/open-apis/performance/v2/activity/query");
    }

    #[test]
    fn test_payroll_api_urls() {
        let url = PayrollApiV1::PaygroupList.to_url();
        assert_eq!(url, "/open-apis/payroll/v1/paygroups");
    }

    #[test]
    fn test_okr_api_urls() {
        let url = OkrApiV1::PeriodList.to_url();
        assert_eq!(url, "/open-apis/okr/v1/periods");
    }

    #[test]
    fn test_ehr_api_urls() {
        let url = EhrApiV1::EmployeeList.to_url();
        assert_eq!(url, "/open-apis/ehr/v1/employees");
    }

    #[test]
    fn test_feishu_people_api_urls() {
        let url = FeishuPeopleApiV1::DepartmentList.to_url();
        assert_eq!(url, "/open-apis/corehr/v1/departments");

        let url = FeishuPeopleApiV1::DepartmentGet("789".to_string()).to_url();
        assert_eq!(url, "/open-apis/corehr/v1/departments/789");
    }

    #[test]
    fn test_feishu_people_api_v2_urls() {
        let url = FeishuPeopleApiV2::CompanyActive("123".to_string()).to_url();
        assert_eq!(url, "/open-apis/corehr/v2/companies/123/active");

        let url = FeishuPeopleApiV2::DepartmentTree.to_url();
        assert_eq!(url, "/open-apis/corehr/v2/departments/tree");

        let url = FeishuPeopleApiV2::EmployeeCreate.to_url();
        assert_eq!(url, "/open-apis/corehr/v2/employees");
    }
}
