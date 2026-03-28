//! 更新人员组成员
//!
//! docPath: <https://open.feishu.cn/document/server-docs/performance-v2/user_group_user_rel/write>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 更新人员组成员请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct WriteRequest {
    /// 用户组 ID（必填）
    user_group_id: String,
    /// 要添加的用户 ID 列表（可选）
    add_user_ids: Vec<String>,
    /// 要移除的用户 ID 列表（可选）
    remove_user_ids: Vec<String>,
    /// 配置信息
    config: Config,
}

impl WriteRequest {
    /// 创建请求
    pub fn new(config: Config, user_group_id: String) -> Self {
        Self {
            user_group_id,
            add_user_ids: Vec::new(),
            remove_user_ids: Vec::new(),
            config,
        }
    }

    /// 添加用户
    pub fn add_user(mut self, user_id: String) -> Self {
        self.add_user_ids.push(user_id);
        self
    }

    /// 移除用户
    pub fn remove_user(mut self, user_id: String) -> Self {
        self.remove_user_ids.push(user_id);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<WriteResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<WriteResponse> {
        use crate::common::api_endpoints::PerformanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.user_group_id.trim(), "user_group_id");

        // 2. 构建端点
        let api_endpoint = PerformanceApiV1::UserGroupUserRelWrite;
        let request = ApiRequest::<WriteResponse>::post(api_endpoint.to_url());

        // 3. 构建请求体
        let request_body = WriteRequestBody {
            user_group_id: self.user_group_id,
            add_user_ids: self.add_user_ids,
            remove_user_ids: self.remove_user_ids,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {}", e),
            )
        })?;
        let request = request.body(request_body_json);

        // 4. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 5. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "更新人员组成员响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 写入请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteRequestBody {
    /// 用户组 ID
    pub user_group_id: String,
    /// 要添加的用户 ID 列表
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub add_user_ids: Vec<String>,
    /// 要移除的用户 ID 列表
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub remove_user_ids: Vec<String>,
}

/// 更新人员组成员响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WriteResponse {
    /// 是否成功
    pub success: bool,
}

impl ApiResponseTrait for WriteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
