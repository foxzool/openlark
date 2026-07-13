use super::super::DefaultServiceRegistry;
use crate::Result;

// #436 待迁：hr / ai / workflow / platform / application / helpdesk / mail /
// analytics / user 仍走 legacy 双声明。foundational + bot 已迁入 capability catalog。
compiled_services! {
    {
        feature: "hr",
        name: "hr",
        description: "飞书人力资源服务，提供员工、考勤、薪酬等功能",
        dependencies: ["auth"],
        provides: ["attendance", "corehr", "ehr"],
        priority: 4,
    },
    {
        feature: "ai",
        name: "ai",
        description: "飞书AI服务，提供智能助手、AI分析等功能",
        dependencies: ["auth", "communication"],
        provides: ["chatbot", "smart-analysis"],
        priority: 4,
    },
    {
        feature: "workflow",
        name: "workflow",
        description: "飞书工作流服务，提供审批、任务、看板等功能",
        dependencies: ["auth"],
        provides: ["approval", "task", "board"],
        priority: 4,
    },
    {
        feature: "platform",
        name: "platform",
        description: "飞书平台服务，提供应用平台相关功能",
        dependencies: ["auth"],
        provides: ["app-platform"],
        priority: 4,
    },
    {
        feature: "application",
        name: "application",
        description: "飞书应用服务，提供应用管理相关功能",
        dependencies: ["auth"],
        provides: ["app-management"],
        priority: 4,
    },
    {
        feature: "helpdesk",
        name: "helpdesk",
        description: "飞书帮助台服务，提供工单管理相关功能",
        dependencies: ["auth"],
        provides: ["ticket"],
        priority: 4,
    },
    {
        feature: "mail",
        name: "mail",
        description: "飞书邮件服务，提供邮件相关功能",
        dependencies: ["auth"],
        provides: ["email"],
        priority: 4,
    },
    {
        feature: "analytics",
        name: "analytics",
        description: "飞书分析服务，提供数据分析相关功能",
        dependencies: ["auth"],
        provides: ["report"],
        priority: 4,
    },
    {
        feature: "user",
        name: "user",
        description: "飞书用户服务，提供用户设置相关功能",
        dependencies: ["auth"],
        provides: ["system_status"],
        priority: 4,
    },
}
