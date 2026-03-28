//! 查询统计数据
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_stats_data/query>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};

use super::models::{QueryRequestBody, QueryResponse};

/// 查询统计数据请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 查询的起始日期，格式为 yyyy-MM-dd
    start_date: String,
    /// 查询的结束日期，格式为 yyyy-MM-dd
    end_date: String,
    /// 查询的用户 ID 列表，最多支持 50 个用户
    user_ids: Vec<String>,
    /// 查询的考勤组 ID 列表
    group_ids: Vec<String>,
    /// 用户 ID 类型，可选值：open_id、union_id、user_id
    user_id_type: Option<String>,
    /// 统计类型，可选值：day（日度统计）、month（月度统计）
    stats_type: Option<String>,
    /// 分页大小，默认值为 100，最大值为 200
    page_size: Option<i32>,
    /// 分页标记，用于获取下一页数据
    page_token: Option<String>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建查询统计数据请求
    pub fn new(config: Config) -> Self {
        Self {
            start_date: String::new(),
            end_date: String::new(),
            user_ids: Vec::new(),
            group_ids: Vec::new(),
            user_id_type: None,
            stats_type: None,
            page_size: None,
            page_token: None,
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

    /// 设置查询用户 ID 列表
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = user_ids;
        self
    }

    /// 设置考勤组 ID 列表
    pub fn group_ids(mut self, group_ids: Vec<String>) -> Self {
        self.group_ids = group_ids;
        self
    }

    /// 设置用户 ID 类型（可选）
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 设置统计类型（可选）
    pub fn stats_type(mut self, stats_type: String) -> Self {
        self.stats_type = Some(stats_type);
        self
    }

    /// 设置分页大小（可选）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（可选）
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
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

        // 1. 验证必填字段
        validate_required!(self.start_date.trim(), "查询起始日期不能为空");
        validate_required!(self.end_date.trim(), "查询结束日期不能为空");

        // 至少需要指定用户 ID 或考勤组 ID
        if self.user_ids.is_empty() && self.group_ids.is_empty() {
            return Err(openlark_core::error::validation_error(
                "查询条件不能为空",
                "至少需要指定用户 ID 或考勤组 ID",
            ));
        }

        // 验证用户 ID 数量
        if self.user_ids.len() > 50 {
            return Err(openlark_core::error::validation_error(
                "查询用户 ID 列表超出限制",
                "最多支持 50 个用户",
            ));
        }

        // 验证分页大小
        if let Some(page_size) = self.page_size {
            if !(1..=200).contains(&page_size) {
                return Err(openlark_core::error::validation_error(
                    "分页大小超出范围",
                    "分页大小必须在 1-200 之间",
                ));
            }
        }

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::UserStatsDataQuery;
        let mut request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url());

        // 3. 添加查询参数（可选）
        if let Some(ref user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        // 4. 序列化请求体
        let request_body = QueryRequestBody {
            start_date: self.start_date,
            end_date: self.end_date,
            user_ids: if self.user_ids.is_empty() {
                None
            } else {
                Some(self.user_ids)
            },
            group_ids: if self.group_ids.is_empty() {
                None
            } else {
                Some(self.group_ids)
            },
            user_id_type: self.user_id_type,
            stats_type: self.stats_type,
            page_size: self.page_size,
            page_token: self.page_token,
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
                "查询统计数据响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
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
            QueryRequest::new(TestConfigBuilder::new().build()).start_date("test".to_string());
        let _ = request;
    }

    #[test]
    fn test_query_request_validation_fails_on_default_request() {
        let request = QueryRequest::new(TestConfigBuilder::new().build());
        let rt = tokio::runtime::Runtime::new().expect("创建 tokio runtime 失败");
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }
}
