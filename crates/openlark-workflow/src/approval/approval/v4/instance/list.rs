//! 批量获取审批实例 ID（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/instance/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::Deserialize;
use std::sync::Arc;

/// 批量获取审批实例 ID 响应（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct ListInstanceResponseV4 {
    /// 审批实例 Code 列表，默认以审批实例创建时间的先后顺序排列
    #[serde(default)]
    pub instance_code_list: Vec<String>,

    /// 分页标记，当 has_more 为 true 时返回新的 page_token
    #[serde(default)]
    pub page_token: Option<String>,

    /// 是否还有更多项
    #[serde(default)]
    pub has_more: bool,
}

/// 批量获取审批实例 ID 请求（v4）
#[derive(Debug, Clone)]
pub struct ListInstanceRequestV4 {
    config: Arc<Config>,
    /// 审批定义 Code（必填）
    approval_code: String,
    /// 审批实例创建时间的开始区间，毫秒时间戳（必填；与 end_time 组成时间区间，单次查询不超过 10 小时）
    start_time: String,
    /// 审批实例创建时间的结束区间，毫秒时间戳（必填）
    end_time: String,
    /// 分页大小（1-100，默认 100）
    page_size: Option<i32>,
    /// 分页标记
    page_token: Option<String>,
}

impl ListInstanceRequestV4 {
    /// 创建请求实例。
    pub fn new(
        config: Arc<Config>,
        approval_code: impl Into<String>,
        start_time: impl Into<String>,
        end_time: impl Into<String>,
    ) -> Self {
        Self {
            config,
            approval_code: approval_code.into(),
            start_time: start_time.into(),
            end_time: end_time.into(),
            page_size: None,
            page_token: None,
        }
    }

    /// 设置分页大小（1-100）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListInstanceResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListInstanceResponseV4> {
        validate_required!(self.approval_code, "审批定义 Code 不能为空");
        validate_required!(self.start_time, "start_time 不能为空");
        validate_required!(self.end_time, "end_time 不能为空");

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceList;
        let mut request = ApiRequest::<ListInstanceResponseV4>::get(api_endpoint.to_url())
            .query("approval_code", self.approval_code)
            .query("start_time", self.start_time)
            .query("end_time", self.end_time);
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "批量获取审批实例 ID",
        )
        .await
    }
}

impl ApiResponseTrait for ListInstanceResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_instance_list_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceList;
        assert_eq!(endpoint.to_url(), "/open-apis/approval/v4/instances");
    }
}
