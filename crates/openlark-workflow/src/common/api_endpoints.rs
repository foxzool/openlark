//! 任务 API 端点定义（类型安全枚举系统）
//!
//! 本模块提供基于枚举的 API 端点定义，用于生产代码中的类型安全调用。

/// 任务 API V1 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum TaskApiV1 {
    /// 创建任务
    TaskCreate,
    /// 获取任务详情
    TaskGet(String),
    /// 更新任务
    TaskUpdate(String),
    /// 删除任务
    TaskDelete(String),
    /// 完成任务
    TaskComplete(String),
    /// 取消完成任务
    TaskUncomplete(String),
    /// 获取任务列表
    TaskList,
    /// 创建任务关注者
    TaskFollowerCreate(String),
    /// 删除任务关注者
    TaskFollowerDelete(String, String),
    /// 获取任务关注者列表
    TaskFollowerList(String),
    /// 批量删除任务关注者
    TaskFollowerBatchDelete(String),
    /// 创建任务协作者
    TaskCollaboratorCreate(String),
    /// 删除任务协作者
    TaskCollaboratorDelete(String, String),
    /// 获取任务协作者列表
    TaskCollaboratorList(String),
    /// 批量删除任务协作者
    TaskCollaboratorBatchDelete(String),
    /// 创建任务提醒
    TaskReminderCreate(String),
    /// 删除任务提醒
    TaskReminderDelete(String, String),
    /// 获取任务提醒列表
    TaskReminderList(String),
    /// 创建任务评论
    TaskCommentCreate(String),
    /// 获取任务评论详情
    TaskCommentGet(String, String),
    /// 更新任务评论
    TaskCommentUpdate(String, String),
    /// 删除任务评论
    TaskCommentDelete(String, String),
    /// 获取任务评论列表
    TaskCommentList(String),
}

impl TaskApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            TaskApiV1::TaskCreate => "/open-apis/task/v1/tasks".to_string(),
            TaskApiV1::TaskGet(task_id) => format!("/open-apis/task/v1/tasks/{task_id}"),
            TaskApiV1::TaskUpdate(task_id) => format!("/open-apis/task/v1/tasks/{task_id}"),
            TaskApiV1::TaskDelete(task_id) => format!("/open-apis/task/v1/tasks/{task_id}"),
            TaskApiV1::TaskComplete(task_id) => {
                format!("/open-apis/task/v1/tasks/{task_id}/complete")
            }
            TaskApiV1::TaskUncomplete(task_id) => {
                format!("/open-apis/task/v1/tasks/{task_id}/uncomplete")
            }
            TaskApiV1::TaskList => "/open-apis/task/v1/tasks".to_string(),
            TaskApiV1::TaskFollowerCreate(task_id) => {
                format!("/open-apis/task/v1/tasks/{task_id}/followers")
            }
            TaskApiV1::TaskFollowerDelete(task_id, follower_id) => {
                format!("/open-apis/task/v1/tasks/{task_id}/followers/{follower_id}")
            }
            TaskApiV1::TaskFollowerList(task_id) => {
                format!("/open-apis/task/v1/tasks/{task_id}/followers")
            }
            TaskApiV1::TaskFollowerBatchDelete(task_id) => {
                format!("/open-apis/task/v1/tasks/{task_id}/batch_delete_follower")
            }
            TaskApiV1::TaskCollaboratorCreate(task_id) => {
                format!("/open-apis/task/v1/tasks/{task_id}/collaborators")
            }
            TaskApiV1::TaskCollaboratorDelete(task_id, collaborator_id) => {
                format!("/open-apis/task/v1/tasks/{task_id}/collaborators/{collaborator_id}")
            }
            TaskApiV1::TaskCollaboratorList(task_id) => {
                format!("/open-apis/task/v1/tasks/{task_id}/collaborators")
            }
            TaskApiV1::TaskCollaboratorBatchDelete(task_id) => {
                format!("/open-apis/task/v1/tasks/{task_id}/batch_delete_collaborator")
            }
            TaskApiV1::TaskReminderCreate(task_id) => {
                format!("/open-apis/task/v1/tasks/{task_id}/reminders")
            }
            TaskApiV1::TaskReminderDelete(task_id, reminder_id) => {
                format!("/open-apis/task/v1/tasks/{task_id}/reminders/{reminder_id}")
            }
            TaskApiV1::TaskReminderList(task_id) => {
                format!("/open-apis/task/v1/tasks/{task_id}/reminders")
            }
            TaskApiV1::TaskCommentCreate(task_id) => {
                format!("/open-apis/task/v1/tasks/{task_id}/comments")
            }
            TaskApiV1::TaskCommentGet(task_id, comment_id) => {
                format!("/open-apis/task/v1/tasks/{task_id}/comments/{comment_id}")
            }
            TaskApiV1::TaskCommentUpdate(task_id, comment_id) => {
                format!("/open-apis/task/v1/tasks/{task_id}/comments/{comment_id}")
            }
            TaskApiV1::TaskCommentDelete(task_id, comment_id) => {
                format!("/open-apis/task/v1/tasks/{task_id}/comments/{comment_id}")
            }
            TaskApiV1::TaskCommentList(task_id) => {
                format!("/open-apis/task/v1/tasks/{task_id}/comments")
            }
        }
    }
}

/// 任务 API V2 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum TaskApiV2 {
    /// 创建任务
    TaskCreate,
    /// 获取任务详情
    TaskGet(String),
    /// 更新任务
    TaskUpdate(String),
    /// 删除任务
    TaskDelete(String),
    /// 完成任务
    TaskComplete(String),
    /// 取消完成任务
    TaskUncomplete(String),
    /// 获取任务列表
    TaskList,
    /// 创建任务清单
    TasklistCreate,
    /// 获取任务清单详情
    TasklistGet(String),
    /// 更新任务清单
    TasklistUpdate(String),
    /// 删除任务清单
    TasklistDelete(String),
    /// 获取任务清单列表
    TasklistList,
    /// 创建分组（全局端点，无 tasklist 作用域）
    SectionCreate,
    /// 获取分组详情
    SectionGet(String),
    /// 更新分组
    SectionUpdate(String),
    /// 删除分组
    SectionDelete(String),
    /// 获取分组列表
    SectionList,
    /// 创建自定义字段（全局端点，清单 GUID 通过 body resource_id 传）
    CustomFieldCreate,
    /// 获取自定义字段详情
    CustomFieldGet(String),
    /// 更新自定义字段
    CustomFieldUpdate(String),
    /// 删除自定义字段
    CustomFieldDelete(String),
    /// 获取自定义字段列表
    CustomFieldList,
    /// 创建评论
    CommentCreate(String),
    /// 获取评论详情
    CommentGet(String, String),
    /// 更新评论
    CommentUpdate(String, String),
    /// 删除评论
    CommentDelete(String, String),
    /// 获取评论列表
    CommentList(String),
    /// 上传附件
    AttachmentUpload(String),
    /// 删除附件
    AttachmentDelete(String, String),

    // 子任务相关
    /// 创建子任务
    SubtaskCreate(String),
    /// 获取子任务列表
    SubtaskList(String),

    // 任务相关扩展
    /// 添加任务到任务清单
    TaskAddTasklist(String),
    /// 从任务清单移除任务
    TaskRemoveTasklist(String),
    /// 获取任务所属任务清单列表
    TaskGetTasklists(String),
    /// 添加任务成员
    TaskAddMembers(String),
    /// 移除任务成员
    TaskRemoveMembers(String),
    /// 添加任务提醒
    TaskAddReminders(String),
    /// 移除任务提醒
    TaskRemoveReminders(String),
    /// 添加任务依赖
    TaskAddDependencies(String),
    /// 移除任务依赖
    TaskRemoveDependencies(String),

    // 任务清单相关扩展
    /// 获取任务清单中的任务列表
    TasklistGetTasks(String),
    /// 添加任务清单成员
    TasklistAddMembers(String),
    /// 移除任务清单成员
    TasklistRemoveMembers(String),

    // 分组相关扩展
    /// 获取分组中的任务列表
    SectionGetTasks(String),

    // 活动订阅相关
    /// 创建活动订阅
    ActivitySubscriptionCreate(String),
    /// 获取活动订阅详情
    ActivitySubscriptionGet(String, String),
    /// 更新活动订阅
    ActivitySubscriptionUpdate(String, String),
    /// 删除活动订阅
    ActivitySubscriptionDelete(String, String),
    /// 获取活动订阅列表
    ActivitySubscriptionList(String),

    // 自定义字段选项相关
    /// 创建自定义字段选项
    CustomFieldOptionCreate(String),
    /// 更新自定义字段选项
    CustomFieldOptionUpdate(String, String),

    // 附件相关扩展
    /// 获取附件详情
    AttachmentGet(String),
    /// 获取附件列表
    AttachmentList,

    // 自定义字段 add/remove
    /// 添加自定义字段到任务清单
    CustomFieldAdd(String),
    /// 从任务清单移除自定义字段
    CustomFieldRemove(String),

    // 搜索
    /// 搜索任务
    TaskSearch,
    /// 搜索清单
    TasklistSearch,
}

impl TaskApiV2 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            // 任务相关
            TaskApiV2::TaskCreate => "/open-apis/task/v2/tasks".to_string(),
            TaskApiV2::TaskGet(task_guid) => {
                format!("/open-apis/task/v2/tasks/{task_guid}")
            }
            TaskApiV2::TaskUpdate(task_guid) => {
                format!("/open-apis/task/v2/tasks/{task_guid}")
            }
            TaskApiV2::TaskDelete(task_guid) => {
                format!("/open-apis/task/v2/tasks/{task_guid}")
            }
            TaskApiV2::TaskComplete(task_guid) => {
                format!("/open-apis/task/v2/tasks/{task_guid}/complete")
            }
            TaskApiV2::TaskUncomplete(task_guid) => {
                format!("/open-apis/task/v2/tasks/{task_guid}/uncomplete")
            }
            TaskApiV2::TaskList => "/open-apis/task/v2/tasks".to_string(),

            // 任务清单相关
            TaskApiV2::TasklistCreate => "/open-apis/task/v2/tasklists".to_string(),
            TaskApiV2::TasklistGet(tasklist_guid) => {
                format!("/open-apis/task/v2/tasklists/{tasklist_guid}")
            }
            TaskApiV2::TasklistUpdate(tasklist_guid) => {
                format!("/open-apis/task/v2/tasklists/{tasklist_guid}")
            }
            TaskApiV2::TasklistDelete(tasklist_guid) => {
                format!("/open-apis/task/v2/tasklists/{tasklist_guid}")
            }
            TaskApiV2::TasklistList => "/open-apis/task/v2/tasklists".to_string(),

            // 分组相关（全局端点，无 tasklist 作用域前缀）
            TaskApiV2::SectionCreate => "/open-apis/task/v2/sections".to_string(),
            TaskApiV2::SectionGet(section_guid) => {
                format!("/open-apis/task/v2/sections/{section_guid}")
            }
            TaskApiV2::SectionUpdate(section_guid) => {
                format!("/open-apis/task/v2/sections/{section_guid}")
            }
            TaskApiV2::SectionDelete(section_guid) => {
                format!("/open-apis/task/v2/sections/{section_guid}")
            }
            TaskApiV2::SectionList => "/open-apis/task/v2/sections".to_string(),

            // 自定义字段相关（全局端点）
            TaskApiV2::CustomFieldCreate => "/open-apis/task/v2/custom_fields".to_string(),
            TaskApiV2::CustomFieldGet(field_guid) => {
                format!("/open-apis/task/v2/custom_fields/{field_guid}")
            }
            TaskApiV2::CustomFieldUpdate(field_guid) => {
                format!("/open-apis/task/v2/custom_fields/{field_guid}")
            }
            TaskApiV2::CustomFieldDelete(field_guid) => {
                format!("/open-apis/task/v2/custom_fields/{field_guid}")
            }
            TaskApiV2::CustomFieldList => "/open-apis/task/v2/custom_fields".to_string(),

            // 评论相关
            TaskApiV2::CommentCreate(task_guid) => {
                format!("/open-apis/task/v2/tasks/{task_guid}/comments")
            }
            TaskApiV2::CommentGet(task_guid, comment_guid) => {
                format!("/open-apis/task/v2/tasks/{task_guid}/comments/{comment_guid}")
            }
            TaskApiV2::CommentUpdate(task_guid, comment_guid) => {
                format!("/open-apis/task/v2/tasks/{task_guid}/comments/{comment_guid}")
            }
            TaskApiV2::CommentDelete(task_guid, comment_guid) => {
                format!("/open-apis/task/v2/tasks/{task_guid}/comments/{comment_guid}")
            }
            TaskApiV2::CommentList(task_guid) => {
                format!("/open-apis/task/v2/tasks/{task_guid}/comments")
            }

            // 附件相关
            TaskApiV2::AttachmentUpload(task_guid) => {
                format!("/open-apis/task/v2/tasks/{task_guid}/attachments")
            }
            TaskApiV2::AttachmentDelete(task_guid, attachment_guid) => {
                format!("/open-apis/task/v2/tasks/{task_guid}/attachments/{attachment_guid}")
            }

            // 子任务相关
            TaskApiV2::SubtaskCreate(task_guid) => {
                format!("/open-apis/task/v2/tasks/{task_guid}/subtasks")
            }
            TaskApiV2::SubtaskList(task_guid) => {
                format!("/open-apis/task/v2/tasks/{task_guid}/subtasks")
            }

            // 任务相关扩展
            TaskApiV2::TaskAddTasklist(task_guid) => {
                format!("/open-apis/task/v2/tasks/{task_guid}/add_tasklist")
            }
            TaskApiV2::TaskRemoveTasklist(task_guid) => {
                format!("/open-apis/task/v2/tasks/{task_guid}/remove_tasklist")
            }
            TaskApiV2::TaskGetTasklists(task_guid) => {
                format!("/open-apis/task/v2/tasks/{task_guid}/tasklists")
            }
            TaskApiV2::TaskAddMembers(task_guid) => {
                format!("/open-apis/task/v2/tasks/{task_guid}/add_members")
            }
            TaskApiV2::TaskRemoveMembers(task_guid) => {
                format!("/open-apis/task/v2/tasks/{task_guid}/remove_members")
            }
            TaskApiV2::TaskAddReminders(task_guid) => {
                format!("/open-apis/task/v2/tasks/{task_guid}/add_reminders")
            }
            TaskApiV2::TaskRemoveReminders(task_guid) => {
                format!("/open-apis/task/v2/tasks/{task_guid}/remove_reminders")
            }
            TaskApiV2::TaskAddDependencies(task_guid) => {
                format!("/open-apis/task/v2/tasks/{task_guid}/add_dependencies")
            }
            TaskApiV2::TaskRemoveDependencies(task_guid) => {
                format!("/open-apis/task/v2/tasks/{task_guid}/remove_dependencies")
            }

            // 任务清单相关扩展
            TaskApiV2::TasklistGetTasks(tasklist_guid) => {
                format!("/open-apis/task/v2/tasklists/{tasklist_guid}/tasks")
            }
            TaskApiV2::TasklistAddMembers(tasklist_guid) => {
                format!("/open-apis/task/v2/tasklists/{tasklist_guid}/add_members")
            }
            TaskApiV2::TasklistRemoveMembers(tasklist_guid) => {
                format!("/open-apis/task/v2/tasklists/{tasklist_guid}/remove_members")
            }

            // 分组相关扩展
            TaskApiV2::SectionGetTasks(section_guid) => {
                format!("/open-apis/task/v2/sections/{section_guid}/tasks")
            }

            // 活动订阅相关
            TaskApiV2::ActivitySubscriptionCreate(tasklist_guid) => {
                format!("/open-apis/task/v2/tasklists/{tasklist_guid}/activity_subscriptions")
            }
            TaskApiV2::ActivitySubscriptionGet(tasklist_guid, subscription_guid) => {
                format!(
                    "/open-apis/task/v2/tasklists/{tasklist_guid}/activity_subscriptions/{subscription_guid}"
                )
            }
            TaskApiV2::ActivitySubscriptionUpdate(tasklist_guid, subscription_guid) => {
                format!(
                    "/open-apis/task/v2/tasklists/{tasklist_guid}/activity_subscriptions/{subscription_guid}"
                )
            }
            TaskApiV2::ActivitySubscriptionDelete(tasklist_guid, subscription_guid) => {
                format!(
                    "/open-apis/task/v2/tasklists/{tasklist_guid}/activity_subscriptions/{subscription_guid}"
                )
            }
            TaskApiV2::ActivitySubscriptionList(tasklist_guid) => {
                format!("/open-apis/task/v2/tasklists/{tasklist_guid}/activity_subscriptions")
            }

            // 自定义字段选项相关
            TaskApiV2::CustomFieldOptionCreate(custom_field_guid) => {
                format!("/open-apis/task/v2/custom_fields/{custom_field_guid}/options")
            }
            TaskApiV2::CustomFieldOptionUpdate(custom_field_guid, option_guid) => {
                format!(
                    "/open-apis/task/v2/custom_fields/{custom_field_guid}/options/{option_guid}"
                )
            }

            // 附件相关扩展
            TaskApiV2::AttachmentGet(attachment_guid) => {
                format!("/open-apis/task/v2/attachments/{attachment_guid}")
            }
            TaskApiV2::AttachmentList => "/open-apis/task/v2/attachments".to_string(),

            // 自定义字段 add/remove
            TaskApiV2::CustomFieldAdd(custom_field_guid) => {
                format!("/open-apis/task/v2/custom_fields/{custom_field_guid}/add")
            }
            TaskApiV2::CustomFieldRemove(custom_field_guid) => {
                format!("/open-apis/task/v2/custom_fields/{custom_field_guid}/remove")
            }

            // 搜索
            TaskApiV2::TaskSearch => "/open-apis/task/v2/tasks/search".to_string(),
            TaskApiV2::TasklistSearch => "/open-apis/task/v2/tasklists/search".to_string(),
        }
    }
}

/// 审批 API V4 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum ApprovalApiV4 {
    /// 创建审批定义
    ApprovalCreate,
    /// 查看指定审批定义
    ApprovalGet(String),
    /// 订阅审批事件
    ApprovalSubscribe(String),
    /// 取消订阅审批事件
    ApprovalUnsubscribe(String),
    /// 创建三方审批定义
    ExternalApprovalCreate,
    /// 查看指定三方审批定义
    ExternalApprovalGet(String),
    /// 校验三方审批实例
    ExternalInstanceCheck,
    /// 同步三方审批实例
    ExternalInstanceCreate,
    /// 获取三方审批任务状态
    ExternalTaskList,
    /// 审批任务加签
    InstanceAddSign,
    /// 抄送审批实例（用户级）
    InstanceAddCc,
    /// 撤回审批实例
    InstanceCancel,
    /// 抄送审批实例
    InstanceCc,
    /// 创建审批实例
    InstanceCreate,
    /// 获取单个审批实例详情（用户级）
    InstanceDetail,
    /// 获取单个审批实例详情
    InstanceGet(String),
    /// 查询用户的已发起审批列表（用户级）
    InstanceInitiated,
    /// 批量获取审批实例 ID
    InstanceList(String),
    /// 预览审批流程
    InstancePreview,
    /// 查询实例列表
    InstanceQuery,
    /// 查询抄送列表
    InstanceSearchCc,
    /// 单据催办（用户级）
    InstanceRemind,
    /// 撤回审批实例（用户级）
    InstanceRecall,
    /// 退回审批任务
    InstanceSpecifiedRollback(String),
    /// 创建评论
    InstanceCommentCreate(String),
    /// 删除评论
    InstanceCommentDelete(String, String),
    /// 获取评论列表
    InstanceCommentList(String),
    /// 审批任务加签（用户级）
    TaskAddSign,
    /// 同意审批任务
    TaskApprove,
    /// 转交审批任务（用户级）
    TaskForward,
    /// 查询审批任务列表（用户级）
    TaskList,
    /// 同意审批任务（用户级）
    TaskPass,
    /// 查询用户的任务列表
    TaskQuery,
    /// 拒绝审批任务（用户级）
    TaskRefuse,
    /// 拒绝审批任务
    TaskReject,
    /// 退回审批任务（用户级）
    TaskRollback,
    /// 重新提交审批任务
    TaskResubmit,
    /// 查询任务列表
    TaskSearch,
    /// 转交审批任务
    TaskTransfer,
}

impl ApprovalApiV4 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            // 审批定义相关
            ApprovalApiV4::ApprovalCreate => "/open-apis/approval/v4/approvals".to_string(),
            ApprovalApiV4::ApprovalGet(approval_code) => {
                format!("/open-apis/approval/v4/approvals/{approval_code}")
            }
            ApprovalApiV4::ApprovalSubscribe(approval_code) => {
                format!("/open-apis/approval/v4/approvals/{approval_code}/subscribe")
            }
            ApprovalApiV4::ApprovalUnsubscribe(approval_code) => {
                format!("/open-apis/approval/v4/approvals/{approval_code}/unsubscribe")
            }

            // 三方审批定义相关
            ApprovalApiV4::ExternalApprovalCreate => {
                "/open-apis/approval/v4/external_approvals".to_string()
            }
            ApprovalApiV4::ExternalApprovalGet(approval_code) => {
                format!("/open-apis/approval/v4/external_approvals/{approval_code}")
            }

            // 三方审批实例相关
            ApprovalApiV4::ExternalInstanceCheck => {
                "/open-apis/approval/v4/external_instances/check".to_string()
            }
            ApprovalApiV4::ExternalInstanceCreate => {
                "/open-apis/approval/v4/external_instances".to_string()
            }

            // 三方审批任务相关
            ApprovalApiV4::ExternalTaskList => "/open-apis/approval/v4/external_tasks".to_string(),

            // 审批实例相关
            ApprovalApiV4::InstanceAddSign => {
                "/open-apis/approval/v4/instances/add_sign".to_string()
            }
            ApprovalApiV4::InstanceAddCc => "/open-apis/approval/v4/instances/add_cc".to_string(),
            ApprovalApiV4::InstanceCancel => "/open-apis/approval/v4/instances/cancel".to_string(),
            ApprovalApiV4::InstanceCc => "/open-apis/approval/v4/instances/cc".to_string(),
            ApprovalApiV4::InstanceCreate => "/open-apis/approval/v4/instances".to_string(),
            ApprovalApiV4::InstanceDetail => "/open-apis/approval/v4/instances/detail".to_string(),
            ApprovalApiV4::InstanceGet(instance_id) => {
                format!("/open-apis/approval/v4/instances/{instance_id}")
            }
            ApprovalApiV4::InstanceInitiated => {
                "/open-apis/approval/v4/instances/initiated".to_string()
            }
            ApprovalApiV4::InstanceList(approval_code) => {
                format!("/open-apis/approval/v4/instances?approval_code={approval_code}")
            }
            ApprovalApiV4::InstancePreview => {
                "/open-apis/approval/v4/instances/preview".to_string()
            }
            ApprovalApiV4::InstanceQuery => "/open-apis/approval/v4/instances/query".to_string(),
            ApprovalApiV4::InstanceSearchCc => {
                "/open-apis/approval/v4/instances/search_cc".to_string()
            }
            ApprovalApiV4::InstanceRemind => "/open-apis/approval/v4/instances/remind".to_string(),
            ApprovalApiV4::InstanceRecall => "/open-apis/approval/v4/instances/recall".to_string(),
            ApprovalApiV4::InstanceSpecifiedRollback(_) => {
                "/open-apis/approval/v4/instances/specified_rollback".to_string()
            }

            // 审批实例评论相关
            ApprovalApiV4::InstanceCommentCreate(instance_id) => {
                format!("/open-apis/approval/v4/instances/{instance_id}/comments")
            }
            ApprovalApiV4::InstanceCommentDelete(instance_id, comment_id) => {
                format!("/open-apis/approval/v4/instances/{instance_id}/comments/{comment_id}")
            }
            ApprovalApiV4::InstanceCommentList(instance_id) => {
                format!("/open-apis/approval/v4/instances/{instance_id}/comments")
            }

            // 审批任务相关
            ApprovalApiV4::TaskAddSign => "/open-apis/approval/v4/tasks/add_sign".to_string(),
            ApprovalApiV4::TaskApprove => "/open-apis/approval/v4/tasks/approve".to_string(),
            ApprovalApiV4::TaskForward => "/open-apis/approval/v4/tasks/forward".to_string(),
            ApprovalApiV4::TaskList => "/open-apis/approval/v4/tasks".to_string(),
            ApprovalApiV4::TaskPass => "/open-apis/approval/v4/tasks/pass".to_string(),
            ApprovalApiV4::TaskQuery => "/open-apis/approval/v4/tasks/query".to_string(),
            ApprovalApiV4::TaskRefuse => "/open-apis/approval/v4/tasks/refuse".to_string(),
            ApprovalApiV4::TaskReject => "/open-apis/approval/v4/tasks/reject".to_string(),
            ApprovalApiV4::TaskRollback => "/open-apis/approval/v4/tasks/rollback".to_string(),
            ApprovalApiV4::TaskResubmit => "/open-apis/approval/v4/tasks/resubmit".to_string(),
            ApprovalApiV4::TaskSearch => "/open-apis/approval/v4/tasks/search".to_string(),
            ApprovalApiV4::TaskTransfer => "/open-apis/approval/v4/tasks/transfer".to_string(),
        }
    }
}

/// 白板 API V1 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum BoardApiV1 {
    /// 创建节点
    WhiteboardNodeCreate(String),
    /// 获取节点列表
    WhiteboardNodeList(String),
    /// 更新主题
    WhiteboardUpdateTheme(String),
    /// 获取主题
    WhiteboardTheme(String),
    /// 下载为图片
    WhiteboardDownloadAsImage(String),
    /// 创建 PlantUML 节点
    WhiteboardNodeCreatePlantuml(String),
}

impl BoardApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            BoardApiV1::WhiteboardNodeCreate(board_id) => {
                format!("/open-apis/board/v1/whiteboards/{board_id}/nodes")
            }
            BoardApiV1::WhiteboardNodeList(board_id) => {
                format!("/open-apis/board/v1/whiteboards/{board_id}/nodes")
            }
            BoardApiV1::WhiteboardUpdateTheme(board_id) => {
                format!("/open-apis/board/v1/whiteboards/{board_id}/update_theme")
            }
            BoardApiV1::WhiteboardTheme(board_id) => {
                format!("/open-apis/board/v1/whiteboards/{board_id}/theme")
            }
            BoardApiV1::WhiteboardDownloadAsImage(board_id) => {
                format!("/open-apis/board/v1/whiteboards/{board_id}/download_as_image")
            }
            BoardApiV1::WhiteboardNodeCreatePlantuml(board_id) => {
                format!("/open-apis/board/v1/whiteboards/{board_id}/nodes/plantuml")
            }
        }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_task_api_v1_to_url() {
        let cases = vec![
            (
                TaskApiV1::TaskCreate,
                "/open-apis/task/v1/tasks".to_string(),
            ),
            (
                TaskApiV1::TaskGet("task_1".to_string()),
                "/open-apis/task/v1/tasks/task_1".to_string(),
            ),
            (
                TaskApiV1::TaskUpdate("task_1".to_string()),
                "/open-apis/task/v1/tasks/task_1".to_string(),
            ),
            (
                TaskApiV1::TaskDelete("task_1".to_string()),
                "/open-apis/task/v1/tasks/task_1".to_string(),
            ),
            (
                TaskApiV1::TaskComplete("task_1".to_string()),
                "/open-apis/task/v1/tasks/task_1/complete".to_string(),
            ),
            (
                TaskApiV1::TaskUncomplete("task_1".to_string()),
                "/open-apis/task/v1/tasks/task_1/uncomplete".to_string(),
            ),
            (TaskApiV1::TaskList, "/open-apis/task/v1/tasks".to_string()),
            (
                TaskApiV1::TaskFollowerCreate("task_1".to_string()),
                "/open-apis/task/v1/tasks/task_1/followers".to_string(),
            ),
            (
                TaskApiV1::TaskFollowerDelete("task_1".to_string(), "f1".to_string()),
                "/open-apis/task/v1/tasks/task_1/followers/f1".to_string(),
            ),
            (
                TaskApiV1::TaskFollowerList("task_1".to_string()),
                "/open-apis/task/v1/tasks/task_1/followers".to_string(),
            ),
            (
                TaskApiV1::TaskFollowerBatchDelete("task_1".to_string()),
                "/open-apis/task/v1/tasks/task_1/batch_delete_follower".to_string(),
            ),
            (
                TaskApiV1::TaskCollaboratorCreate("task_1".to_string()),
                "/open-apis/task/v1/tasks/task_1/collaborators".to_string(),
            ),
            (
                TaskApiV1::TaskCollaboratorDelete("task_1".to_string(), "c1".to_string()),
                "/open-apis/task/v1/tasks/task_1/collaborators/c1".to_string(),
            ),
            (
                TaskApiV1::TaskCollaboratorList("task_1".to_string()),
                "/open-apis/task/v1/tasks/task_1/collaborators".to_string(),
            ),
            (
                TaskApiV1::TaskCollaboratorBatchDelete("task_1".to_string()),
                "/open-apis/task/v1/tasks/task_1/batch_delete_collaborator".to_string(),
            ),
            (
                TaskApiV1::TaskReminderCreate("task_1".to_string()),
                "/open-apis/task/v1/tasks/task_1/reminders".to_string(),
            ),
            (
                TaskApiV1::TaskReminderDelete("task_1".to_string(), "r1".to_string()),
                "/open-apis/task/v1/tasks/task_1/reminders/r1".to_string(),
            ),
            (
                TaskApiV1::TaskReminderList("task_1".to_string()),
                "/open-apis/task/v1/tasks/task_1/reminders".to_string(),
            ),
            (
                TaskApiV1::TaskCommentCreate("task_1".to_string()),
                "/open-apis/task/v1/tasks/task_1/comments".to_string(),
            ),
            (
                TaskApiV1::TaskCommentGet("task_1".to_string(), "cm1".to_string()),
                "/open-apis/task/v1/tasks/task_1/comments/cm1".to_string(),
            ),
            (
                TaskApiV1::TaskCommentUpdate("task_1".to_string(), "cm1".to_string()),
                "/open-apis/task/v1/tasks/task_1/comments/cm1".to_string(),
            ),
            (
                TaskApiV1::TaskCommentDelete("task_1".to_string(), "cm1".to_string()),
                "/open-apis/task/v1/tasks/task_1/comments/cm1".to_string(),
            ),
            (
                TaskApiV1::TaskCommentList("task_1".to_string()),
                "/open-apis/task/v1/tasks/task_1/comments".to_string(),
            ),
        ];

        for (api, expected) in cases {
            assert_eq!(api.to_url(), expected);
        }
    }

    #[test]
    fn test_task_api_v2_to_url() {
        let cases = vec![
            (
                TaskApiV2::TaskCreate,
                "/open-apis/task/v2/tasks".to_string(),
            ),
            (
                TaskApiV2::TaskGet("t1".to_string()),
                "/open-apis/task/v2/tasks/t1".to_string(),
            ),
            (
                TaskApiV2::TaskUpdate("t1".to_string()),
                "/open-apis/task/v2/tasks/t1".to_string(),
            ),
            (
                TaskApiV2::TaskDelete("t1".to_string()),
                "/open-apis/task/v2/tasks/t1".to_string(),
            ),
            (
                TaskApiV2::TaskComplete("t1".to_string()),
                "/open-apis/task/v2/tasks/t1/complete".to_string(),
            ),
            (
                TaskApiV2::TaskUncomplete("t1".to_string()),
                "/open-apis/task/v2/tasks/t1/uncomplete".to_string(),
            ),
            (TaskApiV2::TaskList, "/open-apis/task/v2/tasks".to_string()),
            (
                TaskApiV2::TasklistCreate,
                "/open-apis/task/v2/tasklists".to_string(),
            ),
            (
                TaskApiV2::TasklistGet("tl1".to_string()),
                "/open-apis/task/v2/tasklists/tl1".to_string(),
            ),
            (
                TaskApiV2::TasklistUpdate("tl1".to_string()),
                "/open-apis/task/v2/tasklists/tl1".to_string(),
            ),
            (
                TaskApiV2::TasklistDelete("tl1".to_string()),
                "/open-apis/task/v2/tasklists/tl1".to_string(),
            ),
            (
                TaskApiV2::TasklistList,
                "/open-apis/task/v2/tasklists".to_string(),
            ),
            (
                TaskApiV2::SectionCreate,
                "/open-apis/task/v2/sections".to_string(),
            ),
            (
                TaskApiV2::SectionGet("s1".to_string()),
                "/open-apis/task/v2/sections/s1".to_string(),
            ),
            (
                TaskApiV2::SectionUpdate("s1".to_string()),
                "/open-apis/task/v2/sections/s1".to_string(),
            ),
            (
                TaskApiV2::SectionDelete("s1".to_string()),
                "/open-apis/task/v2/sections/s1".to_string(),
            ),
            (
                TaskApiV2::SectionList,
                "/open-apis/task/v2/sections".to_string(),
            ),
            (
                TaskApiV2::CustomFieldCreate,
                "/open-apis/task/v2/custom_fields".to_string(),
            ),
            (
                TaskApiV2::CustomFieldGet("f1".to_string()),
                "/open-apis/task/v2/custom_fields/f1".to_string(),
            ),
            (
                TaskApiV2::CustomFieldUpdate("f1".to_string()),
                "/open-apis/task/v2/custom_fields/f1".to_string(),
            ),
            (
                TaskApiV2::CustomFieldDelete("f1".to_string()),
                "/open-apis/task/v2/custom_fields/f1".to_string(),
            ),
            (
                TaskApiV2::CustomFieldList,
                "/open-apis/task/v2/custom_fields".to_string(),
            ),
            (
                TaskApiV2::CommentCreate("t1".to_string()),
                "/open-apis/task/v2/tasks/t1/comments".to_string(),
            ),
            (
                TaskApiV2::CommentGet("t1".to_string(), "c1".to_string()),
                "/open-apis/task/v2/tasks/t1/comments/c1".to_string(),
            ),
            (
                TaskApiV2::CommentUpdate("t1".to_string(), "c1".to_string()),
                "/open-apis/task/v2/tasks/t1/comments/c1".to_string(),
            ),
            (
                TaskApiV2::CommentDelete("t1".to_string(), "c1".to_string()),
                "/open-apis/task/v2/tasks/t1/comments/c1".to_string(),
            ),
            (
                TaskApiV2::CommentList("t1".to_string()),
                "/open-apis/task/v2/tasks/t1/comments".to_string(),
            ),
            (
                TaskApiV2::AttachmentUpload("t1".to_string()),
                "/open-apis/task/v2/tasks/t1/attachments".to_string(),
            ),
            (
                TaskApiV2::AttachmentDelete("t1".to_string(), "a1".to_string()),
                "/open-apis/task/v2/tasks/t1/attachments/a1".to_string(),
            ),
            (
                TaskApiV2::SubtaskCreate("t1".to_string()),
                "/open-apis/task/v2/tasks/t1/subtasks".to_string(),
            ),
            (
                TaskApiV2::SubtaskList("t1".to_string()),
                "/open-apis/task/v2/tasks/t1/subtasks".to_string(),
            ),
            (
                TaskApiV2::TaskAddTasklist("t1".to_string()),
                "/open-apis/task/v2/tasks/t1/add_tasklist".to_string(),
            ),
            (
                TaskApiV2::TaskRemoveTasklist("t1".to_string()),
                "/open-apis/task/v2/tasks/t1/remove_tasklist".to_string(),
            ),
            (
                TaskApiV2::TaskGetTasklists("t1".to_string()),
                "/open-apis/task/v2/tasks/t1/tasklists".to_string(),
            ),
            (
                TaskApiV2::TaskAddMembers("t1".to_string()),
                "/open-apis/task/v2/tasks/t1/add_members".to_string(),
            ),
            (
                TaskApiV2::TaskRemoveMembers("t1".to_string()),
                "/open-apis/task/v2/tasks/t1/remove_members".to_string(),
            ),
            (
                TaskApiV2::TaskAddReminders("t1".to_string()),
                "/open-apis/task/v2/tasks/t1/add_reminders".to_string(),
            ),
            (
                TaskApiV2::TaskRemoveReminders("t1".to_string()),
                "/open-apis/task/v2/tasks/t1/remove_reminders".to_string(),
            ),
            (
                TaskApiV2::TaskAddDependencies("t1".to_string()),
                "/open-apis/task/v2/tasks/t1/add_dependencies".to_string(),
            ),
            (
                TaskApiV2::TaskRemoveDependencies("t1".to_string()),
                "/open-apis/task/v2/tasks/t1/remove_dependencies".to_string(),
            ),
            (
                TaskApiV2::TasklistGetTasks("tl1".to_string()),
                "/open-apis/task/v2/tasklists/tl1/tasks".to_string(),
            ),
            (
                TaskApiV2::TasklistAddMembers("tl1".to_string()),
                "/open-apis/task/v2/tasklists/tl1/add_members".to_string(),
            ),
            (
                TaskApiV2::TasklistRemoveMembers("tl1".to_string()),
                "/open-apis/task/v2/tasklists/tl1/remove_members".to_string(),
            ),
            (
                TaskApiV2::SectionGetTasks("s1".to_string()),
                "/open-apis/task/v2/sections/s1/tasks".to_string(),
            ),
            (
                TaskApiV2::ActivitySubscriptionCreate("tl1".to_string()),
                "/open-apis/task/v2/tasklists/tl1/activity_subscriptions".to_string(),
            ),
            (
                TaskApiV2::ActivitySubscriptionGet("tl1".to_string(), "as1".to_string()),
                "/open-apis/task/v2/tasklists/tl1/activity_subscriptions/as1".to_string(),
            ),
            (
                TaskApiV2::ActivitySubscriptionUpdate("tl1".to_string(), "as1".to_string()),
                "/open-apis/task/v2/tasklists/tl1/activity_subscriptions/as1".to_string(),
            ),
            (
                TaskApiV2::ActivitySubscriptionDelete("tl1".to_string(), "as1".to_string()),
                "/open-apis/task/v2/tasklists/tl1/activity_subscriptions/as1".to_string(),
            ),
            (
                TaskApiV2::ActivitySubscriptionList("tl1".to_string()),
                "/open-apis/task/v2/tasklists/tl1/activity_subscriptions".to_string(),
            ),
            (
                TaskApiV2::CustomFieldOptionCreate("cf1".to_string()),
                "/open-apis/task/v2/custom_fields/cf1/options".to_string(),
            ),
            (
                TaskApiV2::CustomFieldOptionUpdate("cf1".to_string(), "op1".to_string()),
                "/open-apis/task/v2/custom_fields/cf1/options/op1".to_string(),
            ),
            (
                TaskApiV2::AttachmentGet("a1".to_string()),
                "/open-apis/task/v2/attachments/a1".to_string(),
            ),
            (
                TaskApiV2::AttachmentList,
                "/open-apis/task/v2/attachments".to_string(),
            ),
            (
                TaskApiV2::CustomFieldAdd("cf1".to_string()),
                "/open-apis/task/v2/custom_fields/cf1/add".to_string(),
            ),
            (
                TaskApiV2::CustomFieldRemove("cf1".to_string()),
                "/open-apis/task/v2/custom_fields/cf1/remove".to_string(),
            ),
        ];

        for (api, expected) in cases {
            assert_eq!(api.to_url(), expected);
        }
    }

    #[test]
    fn test_approval_api_v4_to_url() {
        let cases = vec![
            (
                ApprovalApiV4::ApprovalCreate,
                "/open-apis/approval/v4/approvals".to_string(),
            ),
            (
                ApprovalApiV4::ApprovalGet("code1".to_string()),
                "/open-apis/approval/v4/approvals/code1".to_string(),
            ),
            (
                ApprovalApiV4::ApprovalSubscribe("code1".to_string()),
                "/open-apis/approval/v4/approvals/code1/subscribe".to_string(),
            ),
            (
                ApprovalApiV4::ApprovalUnsubscribe("code1".to_string()),
                "/open-apis/approval/v4/approvals/code1/unsubscribe".to_string(),
            ),
            (
                ApprovalApiV4::ExternalApprovalCreate,
                "/open-apis/approval/v4/external_approvals".to_string(),
            ),
            (
                ApprovalApiV4::ExternalApprovalGet("code1".to_string()),
                "/open-apis/approval/v4/external_approvals/code1".to_string(),
            ),
            (
                ApprovalApiV4::ExternalInstanceCheck,
                "/open-apis/approval/v4/external_instances/check".to_string(),
            ),
            (
                ApprovalApiV4::ExternalInstanceCreate,
                "/open-apis/approval/v4/external_instances".to_string(),
            ),
            (
                ApprovalApiV4::ExternalTaskList,
                "/open-apis/approval/v4/external_tasks".to_string(),
            ),
            (
                ApprovalApiV4::InstanceAddSign,
                "/open-apis/approval/v4/instances/add_sign".to_string(),
            ),
            (
                ApprovalApiV4::InstanceCancel,
                "/open-apis/approval/v4/instances/cancel".to_string(),
            ),
            (
                ApprovalApiV4::InstanceCc,
                "/open-apis/approval/v4/instances/cc".to_string(),
            ),
            (
                ApprovalApiV4::InstanceCreate,
                "/open-apis/approval/v4/instances".to_string(),
            ),
            (
                ApprovalApiV4::InstanceGet("ins1".to_string()),
                "/open-apis/approval/v4/instances/ins1".to_string(),
            ),
            (
                ApprovalApiV4::InstanceList("code1".to_string()),
                "/open-apis/approval/v4/instances?approval_code=code1".to_string(),
            ),
            (
                ApprovalApiV4::InstancePreview,
                "/open-apis/approval/v4/instances/preview".to_string(),
            ),
            (
                ApprovalApiV4::InstanceQuery,
                "/open-apis/approval/v4/instances/query".to_string(),
            ),
            (
                ApprovalApiV4::InstanceSearchCc,
                "/open-apis/approval/v4/instances/search_cc".to_string(),
            ),
            (
                ApprovalApiV4::InstanceSpecifiedRollback("ins1".to_string()),
                "/open-apis/approval/v4/instances/specified_rollback".to_string(),
            ),
            (
                ApprovalApiV4::InstanceCommentCreate("ins1".to_string()),
                "/open-apis/approval/v4/instances/ins1/comments".to_string(),
            ),
            (
                ApprovalApiV4::InstanceCommentDelete("ins1".to_string(), "c1".to_string()),
                "/open-apis/approval/v4/instances/ins1/comments/c1".to_string(),
            ),
            (
                ApprovalApiV4::InstanceCommentList("ins1".to_string()),
                "/open-apis/approval/v4/instances/ins1/comments".to_string(),
            ),
            (
                ApprovalApiV4::TaskApprove,
                "/open-apis/approval/v4/tasks/approve".to_string(),
            ),
            (
                ApprovalApiV4::TaskQuery,
                "/open-apis/approval/v4/tasks/query".to_string(),
            ),
            (
                ApprovalApiV4::TaskReject,
                "/open-apis/approval/v4/tasks/reject".to_string(),
            ),
            (
                ApprovalApiV4::TaskResubmit,
                "/open-apis/approval/v4/tasks/resubmit".to_string(),
            ),
            (
                ApprovalApiV4::TaskSearch,
                "/open-apis/approval/v4/tasks/search".to_string(),
            ),
            (
                ApprovalApiV4::TaskTransfer,
                "/open-apis/approval/v4/tasks/transfer".to_string(),
            ),
        ];

        for (api, expected) in cases {
            assert_eq!(api.to_url(), expected);
        }
    }

    #[test]
    fn test_board_api_v1_to_url() {
        let cases = vec![
            (
                BoardApiV1::WhiteboardNodeCreate("b1".to_string()),
                "/open-apis/board/v1/whiteboards/b1/nodes".to_string(),
            ),
            (
                BoardApiV1::WhiteboardNodeList("b1".to_string()),
                "/open-apis/board/v1/whiteboards/b1/nodes".to_string(),
            ),
            (
                BoardApiV1::WhiteboardUpdateTheme("b1".to_string()),
                "/open-apis/board/v1/whiteboards/b1/update_theme".to_string(),
            ),
            (
                BoardApiV1::WhiteboardTheme("b1".to_string()),
                "/open-apis/board/v1/whiteboards/b1/theme".to_string(),
            ),
            (
                BoardApiV1::WhiteboardDownloadAsImage("b1".to_string()),
                "/open-apis/board/v1/whiteboards/b1/download_as_image".to_string(),
            ),
            (
                BoardApiV1::WhiteboardNodeCreatePlantuml("b1".to_string()),
                "/open-apis/board/v1/whiteboards/b1/nodes/plantuml".to_string(),
            ),
        ];

        for (api, expected) in cases {
            assert_eq!(api.to_url(), expected);
        }
    }

    #[test]
    fn issue_194_workflow_new_endpoint_paths() {
        assert_eq!(
            crate::common::BoardV1Endpoint::WhiteboardNodeBatchDelete("board_123".to_string())
                .to_url(),
            "/open-apis/board/v1/whiteboards/board_123/nodes/batch_delete"
        );
        assert_eq!(
            crate::common::TaskV2Endpoint::TaskSetAncestorTask("task_123".to_string()).to_url(),
            "/open-apis/task/v2/tasks/task_123/set_ancestor_task"
        );
        assert_eq!(
            crate::common::TaskV2Endpoint::ListRelatedTask.to_url(),
            "/open-apis/task/v2/task_v2/list_related_task"
        );
        assert_eq!(
            crate::common::TaskV2Endpoint::TaskSubscription.to_url(),
            "/open-apis/task/v2/task_v2/task_subscription"
        );
    }
}
