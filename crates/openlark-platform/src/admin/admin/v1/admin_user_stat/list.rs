//! 获取用户维度的用户活跃和功能使用数据 API
//! docPath: https://open.feishu.cn/document/server-docs/admin-v1/data-report-management/list-2

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 获取用户维度统计数据的请求构建器。
pub struct ListAdminUserStatBuilder {
    start_date: String,
    end_date: String,
    page_size: Option<u32>,
    page_token: Option<String>,
    config: Config,
}

impl ListAdminUserStatBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            start_date: String::new(),
            end_date: String::new(),
            page_size: None,
            page_token: None,
            config,
        }
    }

    /// 设置统计开始日期。
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = start_date.into();
        self
    }

    /// 设置统计结束日期。
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = end_date.into();
        self
    }

    /// 设置分页大小。
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页游标。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<ListAdminUserStatResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListAdminUserStatResponse> {
        let mut url = format!(
            "/open-apis/admin/v1/admin_user_stats?start_date={}&end_date={}",
            self.start_date, self.end_date
        );

        if let Some(size) = self.page_size {
            url.push_str(&format!("&page_size={size}"));
        }
        if let Some(token) = self.page_token {
            url.push_str(&format!("&page_token={token}"));
        }

        let api_request: ApiRequest<ListAdminUserStatResponse> = ApiRequest::get(url);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取用户统计数据", "响应数据为空")
        })
    }
}

/// 获取用户维度统计数据的响应。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListAdminUserStatResponse {
    /// 统计条目列表。
    pub items: Vec<AdminUserStatItem>,
    /// 下一页分页游标。
    pub page_token: Option<String>,
    /// 是否还有更多数据。
    pub has_more: bool,
}

/// 单个用户的统计信息。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AdminUserStatItem {
    /// 用户 ID。
    pub user_id: String,
    /// 即时消息使用次数。
    pub im_count: u32,
    /// 日历使用次数。
    pub calendar_count: u32,
    /// 文档使用次数。
    pub doc_count: u32,
    /// 视频会议使用次数。
    pub vc_count: u32,
    /// 邮件使用次数。
    pub mail_count: u32,
}

impl ApiResponseTrait for ListAdminUserStatResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = ListAdminUserStatBuilder::new(config.clone())
            .start_date("test".to_string())
            .end_date("test".to_string());
        let _ = request;
    }
}
