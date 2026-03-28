//! 查询归档报表表头
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/archive_rule/user_stats_fields_query>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 查询归档报表表头请求
#[derive(Debug, Clone)]
pub struct UserStatsFieldsQueryRequest {
    /// 归档规则 ID（可选）
    archive_rule_id: Option<String>,
    /// 配置信息
    config: Config,
}

impl UserStatsFieldsQueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            archive_rule_id: None,
            config,
        }
    }

    /// 设置归档规则 ID（可选）
    pub fn archive_rule_id(mut self, archive_rule_id: String) -> Self {
        self.archive_rule_id = Some(archive_rule_id);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UserStatsFieldsQueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UserStatsFieldsQueryResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 构建端点
        let api_endpoint = AttendanceApiV1::ArchiveRuleUserStatsFieldsQuery;
        let mut request = ApiRequest::<UserStatsFieldsQueryResponse>::get(api_endpoint.to_url());

        // 2. 添加查询参数（可选）
        if let Some(ref archive_rule_id) = self.archive_rule_id {
            request = request.query("archive_rule_id", archive_rule_id);
        }

        // 3. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 4. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "查询归档报表表头响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 查询归档报表表头响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserStatsFieldsQueryResponse {
    /// 字段列表
    pub fields: Vec<UserStatsField>,
}

/// 统计字段信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserStatsField {
    /// 字段 ID
    pub field_id: String,
    /// 字段名称
    pub field_name: String,
    /// 字段类型
    pub field_type: String,
    /// 是否显示
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_display: Option<bool>,
    /// 排序权重
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
}

impl ApiResponseTrait for UserStatsFieldsQueryResponse {
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
    fn test_user_stats_fields_query_request_builder_new() {
        let request = UserStatsFieldsQueryRequest::new(TestConfigBuilder::new().build())
            .archive_rule_id("test".to_string());
        let _ = request;
    }
}
