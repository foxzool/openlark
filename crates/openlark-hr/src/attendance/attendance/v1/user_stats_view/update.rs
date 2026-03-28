//! 更新统计设置
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_stats_view/update>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 更新统计设置请求
#[derive(Debug, Clone)]
pub struct UpdateRequest {
    /// 视图 ID（必填）
    view_id: String,
    /// 视图名称（必填）
    name: String,
    /// 显示字段 ID 列表（必填）
    field_ids: Vec<String>,
    /// 是否设为默认视图（可选）
    is_default: Option<bool>,
    /// 配置信息
    config: Config,
}

impl UpdateRequest {
    /// 创建请求
    pub fn new(config: Config, view_id: String, name: String, field_ids: Vec<String>) -> Self {
        Self {
            view_id,
            name,
            field_ids,
            is_default: None,
            config,
        }
    }

    /// 设置是否设为默认视图（可选）
    pub fn is_default(mut self, is_default: bool) -> Self {
        self.is_default = Some(is_default);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.view_id.trim(), "view_id");
        validate_required!(self.name.trim(), "name");
        validate_required!(self.field_ids, "field_ids");

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::UserStatsViewUpdate;
        let request = ApiRequest::<UpdateResponse>::post(api_endpoint.to_url());

        // 3. 构建请求体
        let request_body = UpdateRequestBody {
            view_id: self.view_id,
            name: self.name,
            field_ids: self.field_ids,
            is_default: self.is_default,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "构建请求体失败",
                format!("序列化请求体失败: {}", e),
            )
        })?;
        let request = request.body(request_body_json);

        // 4. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 5. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "更新统计设置响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 更新统计设置请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRequestBody {
    /// 视图 ID
    pub view_id: String,
    /// 视图名称
    pub name: String,
    /// 显示字段 ID 列表
    pub field_ids: Vec<String>,
    /// 是否设为默认视图
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

/// 更新统计设置响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateResponse {
    /// 是否成功
    pub success: bool,
    /// 视图 ID
    pub view_id: String,
}

impl ApiResponseTrait for UpdateResponse {
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
    fn test_update_request_builder_new() {
        let request = UpdateRequest::new(
            TestConfigBuilder::new().build(),
            "test".to_string(),
            "test".to_string(),
            vec!["test".to_string()],
        );
        let _ = request;
    }
}
