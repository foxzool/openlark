pub mod create;
pub mod query;
pub mod query_user_allowed_remedys;

use serde::{Deserialize, Serialize};

/// 补卡记录（user_task_remedy）
///
/// 字段表对齐官方文档；`time`/`time_zone`/`create_time`/`update_time` 在
/// 「通知补卡审批发起」(create) 响应中为无效字段（官方示例恒为 "-"），在
/// 「获取补卡记录」(query) 响应中为秒级时间戳/考勤组时区。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserTaskRemedy {
    /// 用户 ID，对应 employee_type
    pub user_id: String,
    /// 补卡日期，格式为 yyyyMMdd
    pub remedy_date: i32,
    /// 第几次上下班：0-第 1 次，1-第 2 次，2-第 3 次；自由班制填 0
    pub punch_no: i32,
    /// 上班/下班：1-上班，2-下班；自由班制填 0
    pub work_type: i32,
    /// 审批 ID，可用于通知审批状态更新
    pub approval_id: String,
    /// 补卡时间，格式为 yyyy-MM-dd HH:mm
    pub remedy_time: String,
    /// 补卡状态：0-审批中，1-未通过，2-已通过，3-已取消，4-通过后撤回
    /// （create 响应中为无效字段，默认返回审批中）
    pub status: i32,
    /// 补卡原因
    pub reason: String,
    /// 补卡时间，精确到秒的时间戳（create 响应中为无效字段）
    pub time: String,
    /// 补卡时考勤组时区（create 响应中为无效字段）
    pub time_zone: String,
    /// 补卡发起时间，精确到秒的时间戳（create 响应中为无效字段）
    pub create_time: String,
    /// 补卡状态更新时间，精确到秒的时间戳（create 响应中为无效字段）
    pub update_time: String,
}
