/// Wiki API 数据模型
///
/// 注意：这些结构体对应「知识库」相关接口响应体中的 data 字段结构。
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 知识空间基础信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiSpace {
    /// 知识空间ID
    pub space_id: String,
    /// 知识空间名称
    pub name: String,
    /// 知识空间描述
    pub description: Option<String>,
}

/// 知识空间节点信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiSpaceNode {
    /// 知识空间ID
    pub space_id: String,
    /// 节点Token
    pub node_token: String,
    /// 实际文档 token（如果要获取或编辑节点内容，需要使用此 token 调用对应的接口）
    pub obj_token: Option<String>,
    /// 实际文档类型（例如 doc、docx 等）
    pub obj_type: Option<String>,
    /// 父节点Token
    pub parent_node_token: Option<String>,
    /// 节点标题
    pub title: Option<String>,
    /// 节点URL
    pub url: Option<String>,
}

/// 知识空间成员信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiSpaceMember {
    /// 成员ID
    pub member_id: Option<String>,
    /// 成员标识类型（例如 openid、userid 等）
    pub member_type: Option<String>,
    /// 成员角色（例如 admin、member）
    pub member_role: Option<String>,
    /// 成员主体类型（例如 user）
    #[serde(rename = "type")]
    pub member_kind: Option<String>,
}

/// 知识空间设置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiSpaceSetting {
    /// 创建设置（例如 admin / admin_and_member）
    pub create_setting: String,
    /// 安全设置（例如 allow / not_allow）
    pub security_setting: String,
    /// 评论设置（例如 allow / not_allow）
    pub comment_setting: String,
}

/// 任务信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WikiTask {
    /// 任务ID
    pub task_id: Option<String>,
    /// 任务状态
    pub status: Option<String>,
    /// 任务类型（部分接口会透传）
    pub task_type: Option<String>,
    /// 迁移结果（结构较复杂，暂以 JSON 透传）
    pub move_result: Option<Vec<serde_json::Value>>,
    /// 其它字段透传
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 搜索结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiSearchResult {
    /// wiki 节点 token
    pub node_id: String,
    /// wiki 所属知识空间 Id
    pub space_id: String,
    /// wiki 类型（参考文档类型表）
    pub obj_type: i32,
    /// 节点真实文档 token
    pub obj_token: String,
    /// 暂未生效，一律返回空
    pub parent_id: Option<String>,
    /// 序号，从 1 开始计数
    pub sort_id: Option<i32>,
    /// wiki 标题
    pub title: Option<String>,
    /// wiki 访问 url
    pub url: Option<String>,
    /// wiki 对应图标的 url
    pub icon: Option<String>,
}
