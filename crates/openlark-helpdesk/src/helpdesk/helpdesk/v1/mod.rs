//! Helpdesk v1 API 模块

/// 坐席接口。
pub mod agent;
/// 坐席排班接口。
pub mod agent_schedule;
/// 坐席技能接口。
pub mod agent_skill;
/// 坐席技能规则接口。
pub mod agent_skill_rule;
/// 机器人消息接口。
pub mod bot;
/// 分类接口。
pub mod category;
/// 事件订阅接口。
pub mod event;
/// FAQ 接口。
pub mod faq;
/// 通知接口。
pub mod notification;
/// 工单接口。
pub mod ticket;
/// 工单自定义字段接口。
pub mod ticket_customized_field;

use openlark_core::config::Config;
use std::sync::Arc;

/// Helpdesk v1 API 入口。
#[derive(Clone)]
pub struct HelpdeskV1 {
    config: Arc<Config>,
}

impl HelpdeskV1 {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 访问工单 API。
    pub fn ticket(&self) -> ticket::Ticket {
        ticket::Ticket::new(self.config.clone())
    }

    /// 访问工单自定义字段 API。
    pub fn ticket_customized_field(&self) -> ticket_customized_field::TicketCustomizedField {
        ticket_customized_field::TicketCustomizedField::new(self.config.clone())
    }

    /// agent。
    pub fn agent(&self) -> agent::Agent {
        agent::Agent::new(self.config.clone())
    }

    /// agent_schedule。
    pub fn agent_schedule(&self) -> agent_schedule::AgentSchedule {
        agent_schedule::AgentSchedule::new(self.config.clone())
    }

    /// agent_skill。
    pub fn agent_skill(&self) -> agent_skill::AgentSkill {
        agent_skill::AgentSkill::new(self.config.clone())
    }

    /// agent_skill_rule。
    pub fn agent_skill_rule(&self) -> agent_skill_rule::AgentSkillRule {
        agent_skill_rule::AgentSkillRule::new(self.config.clone())
    }

    /// category。
    pub fn category(&self) -> category::Category {
        category::Category::new(self.config.clone())
    }

    /// faq。
    pub fn faq(&self) -> faq::Faq {
        faq::Faq::new(self.config.clone())
    }

    /// notification。
    pub fn notification(&self) -> notification::Notification {
        notification::Notification::new(self.config.clone())
    }

    /// event。
    pub fn event(&self) -> event::Event {
        event::Event::new(self.config.clone())
    }

    /// bot。
    pub fn bot(&self) -> bot::Bot {
        bot::Bot::new(self.config.clone())
    }
}
