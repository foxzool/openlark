/// HelpdeskApiV1 枚举。
#[derive(Debug, Clone, PartialEq)]
pub enum HelpdeskApiV1 {
    // Ticket APIs
    /// 创建工单端点。
    TicketCreate,
    /// 获取工单详情端点。
    TicketGet(String),
    /// 更新工单端点。
    TicketUpdate(String),
    /// 工单列表端点。
    TicketList,
    // Agent APIs
    /// 坐席邮箱端点。
    AgentEmail,
    /// 更新坐席端点。
    AgentPatch(String),
    /// 坐席排班列表端点。
    AgentScheduleList,
    /// 创建坐席排班端点。
    AgentScheduleCreate,
    /// 获取坐席排班端点。
    AgentScheduleGet(String),
    /// 更新坐席排班端点。
    AgentSchedulePatch(String),
    /// 删除坐席排班端点。
    AgentScheduleDelete(String),
    /// 坐席技能列表端点。
    AgentSkillList,
    /// 获取坐席技能端点。
    AgentSkillGet(String),
    /// 创建坐席技能端点。
    AgentSkillCreate,
    /// 更新坐席技能端点。
    AgentSkillPatch(String),
    /// 删除坐席技能端点。
    AgentSkillDelete(String),
    /// 坐席技能规则列表端点。
    AgentSkillRuleList,
    // Category APIs
    /// 分类列表端点。
    CategoryList,
    /// 获取分类端点。
    CategoryGet(String),
    /// 创建分类端点。
    CategoryCreate,
    /// 更新分类端点。
    CategoryPatch(String),
    /// 删除分类端点。
    CategoryDelete(String),
    // FAQ APIs
    /// FAQ 列表端点。
    FaqList,
    /// 获取 FAQ 端点。
    FaqGet(String),
    /// 创建 FAQ 端点。
    FaqCreate,
    /// 更新 FAQ 端点。
    FaqPatch(String),
    /// 删除 FAQ 端点。
    FaqDelete(String),
    /// FAQ 搜索端点。
    FaqSearch,
    /// FAQ 图片端点。
    FaqImage(String, String),
    // Notification APIs
    /// 获取通知端点。
    NotificationGet(String),
    /// 创建通知端点。
    NotificationCreate,
    /// 更新通知端点。
    NotificationPatch(String),
    /// 提交通知审批端点。
    NotificationSubmitApprove(String),
    /// 取消通知审批端点。
    NotificationCancelApprove(String),
    /// 执行通知发送端点。
    NotificationExecuteSend(String),
    /// 通知预览端点。
    NotificationPreview(String),
    /// 取消通知发送端点。
    NotificationCancelSend(String),
    // Ticket Message APIs
    /// 工单消息列表端点。
    TicketMessageList(String),
    /// 创建工单消息端点。
    TicketMessageCreate(String),
    // Ticket Customized Field APIs
    /// 工单自定义字段列表端点。
    TicketCustomizedFieldList,
    /// 获取工单自定义字段端点。
    TicketCustomizedFieldGet(String),
    /// 创建工单自定义字段端点。
    TicketCustomizedFieldCreate,
    /// 更新工单自定义字段端点。
    TicketCustomizedFieldPatch(String),
    /// 删除工单自定义字段端点。
    TicketCustomizedFieldDelete(String),
    // Event APIs
    /// 事件订阅端点。
    EventSubscribe,
    /// 事件取消订阅端点。
    EventUnsubscribe,
    // Bot Message
    /// 机器人消息端点。
    BotMessageCreate,
    // Other Ticket APIs
    /// 回复用户提问端点。
    TicketAnswerUserQuery(String),
    /// 工单自定义字段端点。
    TicketCustomizedFields,
    /// 拉起服务端点。
    TicketStartService,
    /// 工单图片端点。
    TicketImage,
}

impl HelpdeskApiV1 {
    /// 返回对应的端点 URL。
    pub fn to_url(&self) -> String {
        match self {
            // Ticket APIs
            HelpdeskApiV1::TicketCreate => "/open-apis/helpdesk/v1/tickets".to_string(),
            HelpdeskApiV1::TicketGet(id) => format!("/open-apis/helpdesk/v1/tickets/{id}"),
            HelpdeskApiV1::TicketUpdate(id) => format!("/open-apis/helpdesk/v1/tickets/{id}"),
            HelpdeskApiV1::TicketList => "/open-apis/helpdesk/v1/tickets".to_string(),
            // Agent APIs
            HelpdeskApiV1::AgentEmail => "/open-apis/helpdesk/v1/agent_emails".to_string(),
            HelpdeskApiV1::AgentPatch(id) => format!("/open-apis/helpdesk/v1/agents/{id}"),
            HelpdeskApiV1::AgentScheduleList => {
                "/open-apis/helpdesk/v1/agent_schedules".to_string()
            }
            HelpdeskApiV1::AgentScheduleCreate => {
                "/open-apis/helpdesk/v1/agent_schedules".to_string()
            }
            HelpdeskApiV1::AgentScheduleGet(id) => {
                format!("/open-apis/helpdesk/v1/agents/{id}/schedules")
            }
            HelpdeskApiV1::AgentSchedulePatch(id) => {
                format!("/open-apis/helpdesk/v1/agents/{id}/schedules")
            }
            HelpdeskApiV1::AgentScheduleDelete(id) => {
                format!("/open-apis/helpdesk/v1/agents/{id}/schedules")
            }
            HelpdeskApiV1::AgentSkillList => "/open-apis/helpdesk/v1/agent_skills".to_string(),
            HelpdeskApiV1::AgentSkillGet(id) => {
                format!("/open-apis/helpdesk/v1/agent_skills/{id}")
            }
            HelpdeskApiV1::AgentSkillCreate => "/open-apis/helpdesk/v1/agent_skills".to_string(),
            HelpdeskApiV1::AgentSkillPatch(id) => {
                format!("/open-apis/helpdesk/v1/agent_skills/{id}")
            }
            HelpdeskApiV1::AgentSkillDelete(id) => {
                format!("/open-apis/helpdesk/v1/agent_skills/{id}")
            }
            HelpdeskApiV1::AgentSkillRuleList => {
                "/open-apis/helpdesk/v1/agent_skill_rules".to_string()
            }
            // Category APIs
            HelpdeskApiV1::CategoryList => "/open-apis/helpdesk/v1/categories".to_string(),
            HelpdeskApiV1::CategoryGet(id) => format!("/open-apis/helpdesk/v1/categories/{id}"),
            HelpdeskApiV1::CategoryCreate => "/open-apis/helpdesk/v1/categories".to_string(),
            HelpdeskApiV1::CategoryPatch(id) => format!("/open-apis/helpdesk/v1/categories/{id}"),
            HelpdeskApiV1::CategoryDelete(id) => {
                format!("/open-apis/helpdesk/v1/categories/{id}")
            }
            // FAQ APIs
            HelpdeskApiV1::FaqList => "/open-apis/helpdesk/v1/faqs".to_string(),
            HelpdeskApiV1::FaqGet(id) => format!("/open-apis/helpdesk/v1/faqs/{id}"),
            HelpdeskApiV1::FaqCreate => "/open-apis/helpdesk/v1/faqs".to_string(),
            HelpdeskApiV1::FaqPatch(id) => format!("/open-apis/helpdesk/v1/faqs/{id}"),
            HelpdeskApiV1::FaqDelete(id) => format!("/open-apis/helpdesk/v1/faqs/{id}"),
            HelpdeskApiV1::FaqSearch => "/open-apis/helpdesk/v1/faqs/search".to_string(),
            HelpdeskApiV1::FaqImage(id, image_key) => {
                format!("/open-apis/helpdesk/v1/faqs/{id}/image/{image_key}")
            }
            // Notification APIs
            HelpdeskApiV1::NotificationGet(id) => {
                format!("/open-apis/helpdesk/v1/notifications/{id}")
            }
            HelpdeskApiV1::NotificationCreate => "/open-apis/helpdesk/v1/notifications".to_string(),
            HelpdeskApiV1::NotificationPatch(id) => {
                format!("/open-apis/helpdesk/v1/notifications/{id}")
            }
            HelpdeskApiV1::NotificationSubmitApprove(id) => {
                format!("/open-apis/helpdesk/v1/notifications/{id}/submit_approve")
            }
            HelpdeskApiV1::NotificationCancelApprove(id) => {
                format!("/open-apis/helpdesk/v1/notifications/{id}/cancel_approve")
            }
            HelpdeskApiV1::NotificationExecuteSend(id) => {
                format!("/open-apis/helpdesk/v1/notifications/{id}/execute_send")
            }
            HelpdeskApiV1::NotificationPreview(id) => {
                format!("/open-apis/helpdesk/v1/notifications/{id}/preview")
            }
            HelpdeskApiV1::NotificationCancelSend(id) => {
                format!("/open-apis/helpdesk/v1/notifications/{id}/cancel_send")
            }
            // Ticket Message APIs
            HelpdeskApiV1::TicketMessageList(ticket_id) => {
                format!("/open-apis/helpdesk/v1/tickets/{ticket_id}/messages")
            }
            HelpdeskApiV1::TicketMessageCreate(ticket_id) => {
                format!("/open-apis/helpdesk/v1/tickets/{ticket_id}/messages")
            }
            // Ticket Customized Field APIs
            HelpdeskApiV1::TicketCustomizedFieldList => {
                "/open-apis/helpdesk/v1/ticket_customized_fields".to_string()
            }
            HelpdeskApiV1::TicketCustomizedFieldGet(id) => {
                format!("/open-apis/helpdesk/v1/ticket_customized_fields/{id}")
            }
            HelpdeskApiV1::TicketCustomizedFieldCreate => {
                "/open-apis/helpdesk/v1/ticket_customized_fields".to_string()
            }
            HelpdeskApiV1::TicketCustomizedFieldPatch(id) => {
                format!("/open-apis/helpdesk/v1/ticket_customized_fields/{id}")
            }
            HelpdeskApiV1::TicketCustomizedFieldDelete(id) => {
                format!("/open-apis/helpdesk/v1/ticket_customized_fields/{id}")
            }
            // Event APIs
            HelpdeskApiV1::EventSubscribe => "/open-apis/helpdesk/v1/events/subscribe".to_string(),
            HelpdeskApiV1::EventUnsubscribe => {
                "/open-apis/helpdesk/v1/events/unsubscribe".to_string()
            }
            // Bot Message
            HelpdeskApiV1::BotMessageCreate => "/open-apis/helpdesk/v1/message".to_string(),
            // Other Ticket APIs
            HelpdeskApiV1::TicketAnswerUserQuery(id) => {
                format!("/open-apis/helpdesk/v1/tickets/{id}/answer_user_query")
            }
            HelpdeskApiV1::TicketCustomizedFields => {
                "/open-apis/helpdesk/v1/customized_fields".to_string()
            }
            HelpdeskApiV1::TicketStartService => "/open-apis/helpdesk/v1/start_service".to_string(),
            HelpdeskApiV1::TicketImage => "/open-apis/helpdesk/v1/ticket_images".to_string(),
        }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_helpdeskapiv1_to_url_coverage() {
        let urls = vec![
            HelpdeskApiV1::TicketCreate.to_url(),
            HelpdeskApiV1::TicketGet("id1".to_string()).to_url(),
            HelpdeskApiV1::TicketUpdate("id1".to_string()).to_url(),
            HelpdeskApiV1::TicketList.to_url(),
            HelpdeskApiV1::AgentEmail.to_url(),
            HelpdeskApiV1::AgentPatch("id1".to_string()).to_url(),
            HelpdeskApiV1::AgentScheduleList.to_url(),
            HelpdeskApiV1::AgentScheduleCreate.to_url(),
            HelpdeskApiV1::AgentScheduleGet("id1".to_string()).to_url(),
            HelpdeskApiV1::AgentSchedulePatch("id1".to_string()).to_url(),
            HelpdeskApiV1::AgentScheduleDelete("id1".to_string()).to_url(),
            HelpdeskApiV1::AgentSkillList.to_url(),
            HelpdeskApiV1::AgentSkillGet("id1".to_string()).to_url(),
            HelpdeskApiV1::AgentSkillCreate.to_url(),
            HelpdeskApiV1::AgentSkillPatch("id1".to_string()).to_url(),
            HelpdeskApiV1::AgentSkillDelete("id1".to_string()).to_url(),
            HelpdeskApiV1::AgentSkillRuleList.to_url(),
            HelpdeskApiV1::CategoryList.to_url(),
            HelpdeskApiV1::CategoryGet("id1".to_string()).to_url(),
            HelpdeskApiV1::CategoryCreate.to_url(),
            HelpdeskApiV1::CategoryPatch("id1".to_string()).to_url(),
            HelpdeskApiV1::CategoryDelete("id1".to_string()).to_url(),
            HelpdeskApiV1::FaqList.to_url(),
            HelpdeskApiV1::FaqGet("id1".to_string()).to_url(),
            HelpdeskApiV1::FaqCreate.to_url(),
            HelpdeskApiV1::FaqPatch("id1".to_string()).to_url(),
            HelpdeskApiV1::FaqDelete("id1".to_string()).to_url(),
            HelpdeskApiV1::FaqSearch.to_url(),
            HelpdeskApiV1::FaqImage("id1".to_string(), "id2".to_string()).to_url(),
            HelpdeskApiV1::NotificationGet("id1".to_string()).to_url(),
            HelpdeskApiV1::NotificationCreate.to_url(),
            HelpdeskApiV1::NotificationPatch("id1".to_string()).to_url(),
            HelpdeskApiV1::NotificationSubmitApprove("id1".to_string()).to_url(),
            HelpdeskApiV1::NotificationCancelApprove("id1".to_string()).to_url(),
            HelpdeskApiV1::NotificationExecuteSend("id1".to_string()).to_url(),
            HelpdeskApiV1::NotificationPreview("id1".to_string()).to_url(),
            HelpdeskApiV1::NotificationCancelSend("id1".to_string()).to_url(),
            HelpdeskApiV1::TicketMessageList("id1".to_string()).to_url(),
            HelpdeskApiV1::TicketMessageCreate("id1".to_string()).to_url(),
            HelpdeskApiV1::TicketCustomizedFieldList.to_url(),
            HelpdeskApiV1::TicketCustomizedFieldGet("id1".to_string()).to_url(),
            HelpdeskApiV1::TicketCustomizedFieldCreate.to_url(),
            HelpdeskApiV1::TicketCustomizedFieldPatch("id1".to_string()).to_url(),
            HelpdeskApiV1::TicketCustomizedFieldDelete("id1".to_string()).to_url(),
            HelpdeskApiV1::EventSubscribe.to_url(),
            HelpdeskApiV1::EventUnsubscribe.to_url(),
            HelpdeskApiV1::BotMessageCreate.to_url(),
            HelpdeskApiV1::TicketAnswerUserQuery("id1".to_string()).to_url(),
            HelpdeskApiV1::TicketCustomizedFields.to_url(),
            HelpdeskApiV1::TicketStartService.to_url(),
            HelpdeskApiV1::TicketImage.to_url(),
        ];
        assert!(urls.iter().all(|url| url.starts_with("/open-apis/")));
    }
}
