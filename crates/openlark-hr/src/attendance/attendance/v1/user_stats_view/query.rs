//! 查询统计设置
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_stats_view/query>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 查询统计设置请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 考勤组 ID（可选）
    group_id: Option<String>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            group_id: None,
            config,
        }
    }

    /// 设置考勤组 ID（可选）
    pub fn group_id(mut self, group_id: String) -> Self {
        self.group_id = Some(group_id);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 构建端点
        let api_endpoint = AttendanceApiV1::UserStatsViewQuery;
        let mut request = ApiRequest::<QueryResponse>::get(api_endpoint.to_url());

        // 2. 添加查询参数（可选）
        if let Some(ref group_id) = self.group_id {
            request = request.query("group_id", group_id);
        }

        // 3. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 4. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "查询统计设置响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 查询统计设置响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 统计视图列表
    pub items: Vec<UserStatsView>,
}

/// 统计视图设置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserStatsView {
    /// 视图 ID
    pub view_id: String,
    /// 视图名称
    pub name: String,
    /// 考勤组 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// 是否为默认视图
    pub is_default: bool,
    /// 显示字段 ID 列表
    pub field_ids: Vec<String>,
    /// 创建时间（Unix 时间戳）
    pub created_at: i64,
    /// 更新时间（Unix 时间戳）
    pub updated_at: i64,
}

impl ApiResponseTrait for QueryResponse {
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
    fn test_query_request_builder_new() {
        let request =
            QueryRequest::new(TestConfigBuilder::new().build()).group_id("test".to_string());
        let _ = request;
    }
}
