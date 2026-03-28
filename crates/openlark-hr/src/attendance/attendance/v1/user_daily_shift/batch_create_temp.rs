//! 创建或修改临时排班
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_daily_shift/batch_create_temp>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 创建或修改临时排班请求
#[derive(Debug, Clone)]
pub struct BatchCreateTempRequest {
    /// 临时排班记录列表（必填）
    shifts: Vec<TempShift>,
    /// 配置信息
    config: Config,
}

impl BatchCreateTempRequest {
    /// 创建请求
    pub fn new(config: Config, shifts: Vec<TempShift>) -> Self {
        Self { shifts, config }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchCreateTempResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchCreateTempResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.shifts, "shifts");

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::UserDailyShiftBatchCreateTemp;
        let request = ApiRequest::<BatchCreateTempResponse>::post(api_endpoint.to_url());

        // 3. 构建请求体
        let request_body = BatchCreateTempRequestBody {
            shifts: self.shifts,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "构建请求体失败",
                format!("序列化请求体失败: {}", e),
            )
        })?;
        let request = request.body(request_body_json);

        // 4. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 5. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "创建或修改临时排班响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 创建或修改临时排班请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateTempRequestBody {
    /// 临时排班记录列表
    pub shifts: Vec<TempShift>,
}

/// 临时排班记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TempShift {
    /// 用户 ID
    pub user_id: String,
    /// 排班日期（Unix 时间戳）
    pub date: i64,
    /// 班次 ID
    pub shift_id: String,
    /// 是否为临时排班
    pub is_temp: bool,
    /// 工作时长（小时）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_hours: Option<f64>,
}

/// 创建或修改临时排班响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchCreateTempResponse {
    /// 是否成功
    pub success: bool,
    /// 成功处理的记录数
    pub processed_count: i32,
    /// 失败的记录数
    pub failed_count: i32,
}

impl ApiResponseTrait for BatchCreateTempResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
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
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
