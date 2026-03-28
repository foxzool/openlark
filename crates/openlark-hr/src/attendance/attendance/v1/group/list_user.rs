//! 查询考勤组下所有成员
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/group/list_user>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 查询考勤组下所有成员请求
#[derive(Debug, Clone)]
pub struct ListUserRequest {
    /// 考勤组 ID（必填）
    group_id: String,
    /// 分页大小（可选，默认 50，最大 100）
    page_size: Option<i32>,
    /// 分页标记（可选）
    page_token: Option<String>,
    /// 配置信息
    config: Config,
}

impl ListUserRequest {
    /// 创建请求
    pub fn new(config: Config, group_id: String) -> Self {
        Self {
            group_id,
            page_size: None,
            page_token: None,
            config,
        }
    }

    /// 设置分页大小（可选，默认 50，最大 100）
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
    pub async fn execute(self) -> SDKResult<ListUserResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListUserResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.group_id.trim(), "group_id");

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::GroupListUser(self.group_id.clone());
        let mut request = ApiRequest::<ListUserResponse>::get(api_endpoint.to_url());

        // 3. 添加查询参数（可选）
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        // 4. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 5. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "查询考勤组下所有成员响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 查询考勤组下所有成员响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListUserResponse {
    /// 成员列表
    pub items: Vec<GroupMember>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 考勤组成员
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupMember {
    /// 用户 ID
    pub user_id: String,
    /// 用户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// 加入时间（Unix 时间戳）
    pub join_time: i64,
}

impl ApiResponseTrait for ListUserResponse {
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
    fn test_list_user_request_builder_new() {
        let request = ListUserRequest::new(TestConfigBuilder::new().build(), "test".to_string());
        let _ = request;
    }
}
