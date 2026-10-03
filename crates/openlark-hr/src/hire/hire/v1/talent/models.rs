//! Hire v1 talent 资源相关模型
//!
//! 包含获取候选人列表、获取候选人信息、综合创建/更新候选人、批量获取候选人ID等API的请求和响应结构体

use serde::{Deserialize, Serialize};

// 候选人基础数据结构

/// 候选人信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Talent {
    /// 候选人 ID
    pub talent_id: String,
    /// 候选人姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 简历内容（HTML格式）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume: Option<String>,
    /// 邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 手机号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 候选人状态
    /// - 1: 新候选人
    /// - 2: 在流程中
    /// - 3: 已归档
    /// - 4: 已入职
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 创建时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 更新时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
}

/// 候选人联系方式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TalentContact {
    /// 邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 手机号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

/// 候选人教育经历
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TalentEducation {
    /// 学校名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub school: Option<String>,
    /// 专业
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<String>,
    /// 学历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub degree: Option<String>,
    /// 开始时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 结束时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
}

/// 候选人工作经历
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TalentWorkExperience {
    /// 公司名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// 职位
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 开始时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 结束时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 工作描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

// 获取候选人列表相关模型

/// 获取候选人列表请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRequestBody {
    /// 分页大小（1-100）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 查询关键词（姓名、邮箱、手机号）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// 候选人状态列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_list: Option<Vec<i32>>,
}

/// 获取候选人列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 候选人列表
    pub talent_list: Vec<Talent>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

// 获取候选人信息相关模型

/// 获取候选人信息请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRequestBody {
    /// 是否需要详细信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_detail: Option<bool>,
}

/// 获取候选人信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 候选人信息
    pub talent: Talent,
    /// 教育经历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub education_list: Option<Vec<TalentEducation>>,
    /// 工作经历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_experience_list: Option<Vec<TalentWorkExperience>>,
}

// 综合创建候选人相关模型

/// 综合创建候选人请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombinedCreateRequestBody {
    /// 候选人姓名（必填）
    pub name: String,
    /// 邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 手机号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 简历内容（HTML格式）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume: Option<String>,
    /// 教育经历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub education_list: Option<Vec<TalentEducation>>,
    /// 工作经历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_experience_list: Option<Vec<TalentWorkExperience>>,
}

/// 综合创建候选人响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CombinedCreateResponse {
    /// 候选人 ID
    pub talent_id: String,
}

// 综合更新候选人相关模型

/// 综合更新候选人请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombinedUpdateRequestBody {
    /// 候选人 ID（必填）
    pub talent_id: String,
    /// 候选人姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 手机号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 简历内容（HTML格式）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume: Option<String>,
    /// 教育经历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub education_list: Option<Vec<TalentEducation>>,
    /// 工作经历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_experience_list: Option<Vec<TalentWorkExperience>>,
}

/// 综合更新候选人响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CombinedUpdateResponse {
    /// 更新结果
    pub result: bool,
}

// 批量获取候选人ID相关模型

/// 批量获取候选人ID请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetIdRequestBody {
    /// 候选人 ID 列表（必填，最多100个）
    pub talent_ids: Vec<String>,
}

/// 候选人ID映射信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TalentIdInfo {
    /// 候选人 ID
    pub talent_id: String,
    /// 是否有效
    pub is_valid: bool,
}

/// 批量获取候选人ID响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetIdResponse {
    /// 候选人ID列表
    pub talent_id_list: Vec<TalentIdInfo>,
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_talent_serialization_roundtrip() {
        let talent = Talent {
            talent_id: "talent_1".to_string(),
            name: Some("张三".to_string()),
            resume: Some("<p>简历</p>".to_string()),
            email: Some("zhangsan@example.com".to_string()),
            phone: Some("13800000000".to_string()),
            status: Some(2),
            create_time: Some(1700000000000),
            update_time: Some(1700000001000),
        };

        let text = serde_json::to_string(&talent).expect("序列化失败");
        let parsed: Talent = serde_json::from_str(&text).expect("反序列化失败");
        assert_eq!(parsed.talent_id, "talent_1");
        assert_eq!(parsed.status, Some(2));
    }

    #[test]
    fn test_talent_response_deserialization() {
        let response: BatchGetIdResponse = serde_json::from_str(
            r#"{"talent_id_list":[{"talent_id":"talent_2","is_valid":true}]}"#,
        )
        .expect("反序列化失败");

        assert_eq!(response.talent_id_list.len(), 1);
        assert!(response.talent_id_list[0].is_valid);
    }
}
