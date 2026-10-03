//! 打卡结果查询模型
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_task/query>

use serde::{Deserialize, Serialize};

/// 查询打卡结果请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryUserTaskRequestBody {
    /// 查询的起始日期，格式为 yyyy-MM-dd
    pub start_date: String,
    /// 查询的结束日期，格式为 yyyy-MM-dd
    pub end_date: String,
    /// 查询的用户 ID 列表，最多支持 50 个用户
    pub user_ids: Vec<String>,
    /// 用户 ID 类型，可选值：open_id、union_id、user_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 查询的打卡类型，可选值：
    /// - 1: 上班打卡
    /// - 2: 下班打卡
    /// - 3: 外出打卡
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_in_type: Option<i32>,
}

/// 打卡记录
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserTaskRecord {
    /// 用户 ID
    pub user_id: String,
    /// 打卡日期，格式为 yyyy-MM-dd
    pub date: String,
    /// 打卡类型
    /// - 1: 上班打卡
    /// - 2: 下班打卡
    /// - 3: 外出打卡
    pub check_in_type: i32,
    /// 打卡时间，格式为 yyyy-MM-dd HH:mm:ss
    pub check_in_time: String,
    /// 打卡地点名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_in_place_name: Option<String>,
    /// 打卡地点 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_in_place_id: Option<String>,
    /// 打卡结果
    /// - 1: 正常
    /// - 2: 迟到
    /// - 3: 早退
    /// - 4: 缺卡
    /// - 5: 地点异常
    /// - 6: Wi-Fi 异常
    /// - 7: 设备异常
    pub check_in_result: i32,
    /// 打卡方式
    /// - 1: 手机打卡
    /// - 2: 考勤机打卡
    /// - 3: 手动导入
    /// - 4: 补卡
    pub check_in_method: i32,
    /// 打卡设备 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// 打卡设备名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// 打卡 Wi-Fi 名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi_name: Option<String>,
    /// 打卡 Wi-Fi MAC 地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi_mac: Option<String>,
    /// 打卡备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 打卡照片列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_list: Option<Vec<String>>,
    /// 经度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    /// 纬度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    /// 外勤打卡地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_address: Option<String>,
    /// 外勤打卡备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_remark: Option<String>,
}

/// 查询打卡结果响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryUserTaskResponse {
    /// 打卡记录列表
    pub records: Vec<UserTaskRecord>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_user_task_record_serialization_roundtrip() {
        let record = UserTaskRecord {
            user_id: "u_1".to_string(),
            date: "2024-01-01".to_string(),
            check_in_type: 1,
            check_in_time: "2024-01-01 09:00:00".to_string(),
            check_in_place_name: Some("总部".to_string()),
            check_in_place_id: Some("p_1".to_string()),
            check_in_result: 1,
            check_in_method: 1,
            device_id: Some("dev_1".to_string()),
            device_name: Some("iPhone".to_string()),
            wifi_name: Some("Office".to_string()),
            wifi_mac: Some("AA:BB:CC:DD:EE:FF".to_string()),
            remark: Some("正常".to_string()),
            photo_list: Some(vec!["url_1".to_string()]),
            longitude: Some(121.0),
            latitude: Some(31.0),
            out_address: None,
            out_remark: None,
        };

        let text = serde_json::to_string(&record).expect("序列化失败");
        let parsed: UserTaskRecord = serde_json::from_str(&text).expect("反序列化失败");
        assert_eq!(parsed.user_id, "u_1");
        assert_eq!(parsed.check_in_result, 1);
    }
}
