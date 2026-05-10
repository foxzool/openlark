//! 查询统计字段定义
//!
//! docPath: https://open.feishu.cn/document/attendance-v1/user_stats_field/query

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required_list,
};

use super::models::{QueryRequestBody, QueryResponse};

/// 查询统计字段定义请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 考勤组 ID
    unit_id: Option<String>,
    /// 统计类型，可选值：daily（日度统计）、monthly（月度统计）
    stat_type: Option<String>,
    /// 用户 ID 列表
    user_ids: Vec<String>,
    /// 查询的起始日期，格式为 yyyy-MM-dd
    start_date: Option<String>,
    /// 查询的结束日期，格式为 yyyy-MM-dd
    end_date: Option<String>,
    /// 是否包含下属，默认值为 false
    is_include_subordinate: Option<bool>,
    /// 分页标记，用于获取下一页数据
    page_token: Option<String>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建查询统计字段定义请求
    pub fn new(config: Config) -> Self {
        Self {
            unit_id: None,
            stat_type: None,
            user_ids: Vec::new(),
            start_date: None,
            end_date: None,
            is_include_subordinate: None,
            page_token: None,
            config,
        }
    }

    /// 设置考勤组 ID（可选）
    pub fn unit_id(mut self, unit_id: String) -> Self {
        self.unit_id = Some(unit_id);
        self
    }

    /// 设置统计类型（可选）
    pub fn stat_type(mut self, stat_type: String) -> Self {
        self.stat_type = Some(stat_type);
        self
    }

    /// 设置用户 ID 列表（可选）
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = user_ids;
        self
    }

    /// 添加单个用户 ID
    pub fn add_user_id(mut self, user_id: String) -> Self {
        self.user_ids.push(user_id);
        self
    }

    /// 设置查询起始日期（可选）
    pub fn start_date(mut self, start_date: String) -> Self {
        self.start_date = Some(start_date);
        self
    }

    /// 设置查询结束日期（可选）
    pub fn end_date(mut self, end_date: String) -> Self {
        self.end_date = Some(end_date);
        self
    }

    /// 设置是否包含下属（可选）
    pub fn is_include_subordinate(mut self, is_include_subordinate: bool) -> Self {
        self.is_include_subordinate = Some(is_include_subordinate);
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
        // 至少需要指定考勤组 ID 或用户 ID
        validate_required_list!(self.user_ids, 50, "查询条件不能为空");

        // 验证用户 ID 数量
        if self.user_ids.len() > 50 {
            return Err(openlark_core::error::validation_error(
                "用户 ID 列表超出限制",
                "最多支持 50 个用户",
            ));
        }

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::UserStatsFieldQuery;
        let mut request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url());

        // 3. 添加查询参数（如果有）
        // 暂无需要添加的查询参数

        // 4. 序列化请求体
        let request_body = QueryRequestBody {
            unit_id: self.unit_id,
            stat_type: self.stat_type,
            user_ids: if self.user_ids.is_empty() {
                None
            } else {
                Some(self.user_ids)
            },
            start_date: self.start_date,
            end_date: self.end_date,
            is_include_subordinate: self.is_include_subordinate,
            page_token: self.page_token,
        };
        request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);

        // 5. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 6. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "查询统计字段定义响应数据为空",
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
            QueryRequest::new(TestConfigBuilder::new().build()).unit_id("test".to_string());
        let _ = request;
    }
}
