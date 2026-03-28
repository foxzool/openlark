//! 查询席位分配详情
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/seat_assignment/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 查询席位分配详情 Builder
#[derive(Debug, Clone)]
pub struct SeatAssignmentListBuilder {
    config: Config,
    /// 页码
    page: Option<u32>,
    /// 每页数量
    page_size: Option<u32>,
}

impl SeatAssignmentListBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page: None,
            page_size: None,
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

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SeatAssignmentListResponse> {
        let url = "/open-apis/apaas/v1/seat_assignments";

        let mut req: ApiRequest<SeatAssignmentListResponse> = ApiRequest::get(url);
        if let Some(page) = self.page {
            req = req.query("page", page.to_string());
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        let resp = Transport::request(req, &self.config, None).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("查询席位分配详情", "响应数据为空")
        })
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SeatAssignmentListResponse> {
        let url = "/open-apis/apaas/v1/seat_assignments";

        let mut req: ApiRequest<SeatAssignmentListResponse> = ApiRequest::get(url);
        if let Some(page) = self.page {
            req = req.query("page", page.to_string());
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("查询席位分配详情", "响应数据为空")
        })
    }
}

/// 席位分配信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SeatAssignment {
    /// 席位 ID
    #[serde(rename = "seat_id")]
    seat_id: String,
    /// 用户 ID
    #[serde(rename = "user_id")]
    user_id: String,
    /// 席位类型
    #[serde(rename = "seat_type")]
    seat_type: String,
    /// 分配时间
    #[serde(rename = "assigned_time")]
    assigned_time: i64,
    /// 到期时间
    #[serde(rename = "expire_time", skip_serializing_if = "Option::is_none")]
    expire_time: Option<i64>,
}

/// 查询席位分配详情响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SeatAssignmentListResponse {
    /// 席位分配列表
    #[serde(rename = "items")]
    items: Vec<SeatAssignment>,
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

impl ApiResponseTrait for SeatAssignmentListResponse {
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
        let request = SeatAssignmentListBuilder::new(config.clone())
            .page(1)
            .page_size(1);
        let _ = request;
    }
}
