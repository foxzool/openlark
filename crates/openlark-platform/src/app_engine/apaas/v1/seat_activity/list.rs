//! 查询席位活跃详情
//!
//! 文档: https://open.feishu.cn/document/apaas-v1/seat_activity/list
//! docPath: https://open.feishu.cn/document/apaas-v1/seat_activity/list

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 查询席位活跃详情 Builder
#[derive(Debug, Clone)]
pub struct SeatActivityListBuilder {
    config: Config,
    /// 页码
    page: Option<u32>,
    /// 每页数量
    page_size: Option<u32>,
    /// 开始时间
    start_time: Option<i64>,
    /// 结束时间
    end_time: Option<i64>,
}

impl SeatActivityListBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page: None,
            page_size: None,
            start_time: None,
            end_time: None,
        }
    }

    /// 设置页码
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// 设置每页数量
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置开始时间
    pub fn start_time(mut self, start_time: i64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// 设置结束时间
    pub fn end_time(mut self, end_time: i64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SeatActivityListResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SeatActivityListResponse> {
        let url = "/open-apis/apaas/v1/seat_activities";

        let mut req: ApiRequest<SeatActivityListResponse> = ApiRequest::get(url);
        if let Some(page) = self.page {
            req = req.query("page", page.to_string());
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        if let Some(start_time) = self.start_time {
            req = req.query("start_time", start_time.to_string());
        }
        if let Some(end_time) = self.end_time {
            req = req.query("end_time", end_time.to_string());
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("查询席位活跃详情", "响应数据为空")
        })
    }
}

/// 席位活跃信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SeatActivity {
    /// 席位 ID
    #[serde(rename = "seat_id")]
    seat_id: String,
    /// 用户 ID
    #[serde(rename = "user_id")]
    user_id: String,
    /// 活跃时间
    #[serde(rename = "active_time")]
    active_time: i64,
    /// 活跃类型
    #[serde(rename = "activity_type")]
    activity_type: String,
}

/// 查询席位活跃详情响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SeatActivityListResponse {
    /// 席位活跃列表
    #[serde(rename = "items")]
    items: Vec<SeatActivity>,
    /// 是否有更多
    #[serde(rename = "has_more")]
    has_more: bool,
    /// 页码
    #[serde(rename = "page")]
    page: u32,
    /// 每页数量
    #[serde(rename = "page_size")]
    page_size: u32,
}

impl ApiResponseTrait for SeatActivityListResponse {
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
        let request = SeatActivityListBuilder::new(config.clone())
            .page(1)
            .page_size(1);
        let _ = request;
    }
}
