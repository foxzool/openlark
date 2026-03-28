//! 批量查询打卡流水
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_flow/query>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, validate_required_list, SDKResult,
};

use super::models::{QueryUserFlowRequestBody, QueryUserFlowResponse};

/// 批量查询打卡流水请求
#[derive(Debug, Clone)]
pub struct QueryUserFlowRequest {
    /// 查询的起始日期，格式为 yyyy-MM-dd
    start_date: String,
    /// 查询的结束日期，格式为 yyyy-MM-dd
    end_date: String,
    /// 查询的用户 ID 列表，最多支持 50 个用户
    user_ids: Vec<String>,
    /// 用户 ID 类型，可选值：open_id、union_id、user_id
    user_id_type: Option<String>,
    /// 查询的打卡类型，可选值：1-上班打卡、2-下班打卡、3-外出打卡
    punch_type: Option<i32>,
    /// 配置信息
    config: Config,
}

impl QueryUserFlowRequest {
    /// 创建批量查询打卡流水请求
    pub fn new(config: Config) -> Self {
        Self {
            start_date: String::new(),
            end_date: String::new(),
            user_ids: Vec::new(),
            user_id_type: None,
            punch_type: None,
            config,
        }
    }

    /// 设置查询起始日期（必填）
    pub fn start_date(mut self, start_date: String) -> Self {
        self.start_date = start_date;
        self
    }

    /// 设置查询结束日期（必填）
    pub fn end_date(mut self, end_date: String) -> Self {
        self.end_date = end_date;
        self
    }

    /// 设置查询用户 ID 列表（必填）
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = user_ids;
        self
    }

    /// 设置用户 ID 类型（可选）
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 设置打卡类型（可选）
    pub fn punch_type(mut self, punch_type: i32) -> Self {
        self.punch_type = Some(punch_type);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryUserFlowResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryUserFlowResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.start_date.trim(), "查询起始日期不能为空");
        validate_required!(self.end_date.trim(), "查询结束日期不能为空");
        validate_required_list!(self.user_ids, 50, "用户 ID 列表不能为空且不能超过 50 个");

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::UserFlowQuery;
        let mut request = ApiRequest::<QueryUserFlowResponse>::post(api_endpoint.to_url());

        // 3. 添加查询参数（可选）
        if let Some(ref user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        // 4. 序列化请求体
        let request_body = QueryUserFlowRequestBody {
            start_date: self.start_date,
            end_date: self.end_date,
            user_ids: self.user_ids,
            user_id_type: self.user_id_type,
            punch_type: self.punch_type,
        };
        request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {}", e),
            )
        })?);

        // 5. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 6. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "批量查询打卡流水响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for QueryUserFlowResponse {
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
    fn test_query_user_flow_request_builder_new() {
        let request = QueryUserFlowRequest::new(TestConfigBuilder::new().build())
            .start_date("test".to_string());
        let _ = request;
    }

    #[test]
    fn test_query_user_flow_request_validation_fails_on_default_request() {
        let request = QueryUserFlowRequest::new(TestConfigBuilder::new().build());
        let rt = tokio::runtime::Runtime::new().expect("创建 tokio runtime 失败");
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }
}
