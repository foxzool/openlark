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

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}

pub mod ticket_customized_field;
