//! 获取可补卡时间
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_task_remedy/query_user_allowed_remedys>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取可补卡时间请求
#[derive(Debug, Clone)]
pub struct QueryUserAllowedRemedysRequest {
    /// 用户 ID（必填）
    user_id: String,
    /// 配置信息
    config: Config,
}

impl QueryUserAllowedRemedysRequest {
    /// 创建请求
    pub fn new(config: Config, user_id: String) -> Self {
        Self { user_id, config }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryUserAllowedRemedysResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryUserAllowedRemedysResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.user_id.trim(), "user_id");

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::UserTaskRemedyQueryUserAllowedRemedys;
        let mut request = ApiRequest::<QueryUserAllowedRemedysResponse>::get(api_endpoint.to_url());

        // 3. 添加查询参数
        request = request.query("user_id", &self.user_id);

        // 4. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 5. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取可补卡时间响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 获取可补卡时间响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryUserAllowedRemedysResponse {
    /// 可补卡时间段列表
    pub items: Vec<AllowedRemedy>,
}

/// 可补卡时间段
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AllowedRemedy {
    /// 开始时间（Unix 时间戳）
    pub start_time: i64,
    /// 结束时间（Unix 时间戳）
    pub end_time: i64,
    /// 星期几（1-7）
    pub day_of_week: i32,
}

impl ApiResponseTrait for QueryUserAllowedRemedysResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::TestConfigBuilder;

    #[test]
    fn test_query_user_allowed_remedys_request_builder_new() {
        let request = QueryUserAllowedRemedysRequest::new(
            TestConfigBuilder::new().build(),
            "test".to_string(),
        );
        let _ = request;
    }
}
