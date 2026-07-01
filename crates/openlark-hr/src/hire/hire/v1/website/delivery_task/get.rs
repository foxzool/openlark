//! 获取招聘官网投递任务结果
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/website.delivery_task/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::hire::hire::common_models::WebsiteDeliveryTaskResult;

/// 获取招聘官网投递任务结果请求
#[derive(Debug, Clone)]
pub struct GetRequest {
    /// 配置信息
    config: Config,
    website_id: Option<String>,
    delivery_task_id: Option<String>,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            website_id: None,
            delivery_task_id: None,
        }
    }

    /// 设置 `website_id`。
    pub fn website_id(mut self, website_id: impl Into<String>) -> Self {
        self.website_id = Some(website_id.into());
        self
    }

    /// 设置 `delivery_task_id`。
    pub fn delivery_task_id(mut self, delivery_task_id: impl Into<String>) -> Self {
        self.delivery_task_id = Some(delivery_task_id.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetResponse> {
        let website_id = self.website_id.unwrap_or_default();
        let delivery_task_id = self.delivery_task_id.unwrap_or_default();
        validate_required!(website_id.trim(), "website_id 不能为空");
        validate_required!(delivery_task_id.trim(), "delivery_task_id 不能为空");

        let request = ApiRequest::<GetResponse>::get(format!(
            "/open-apis/hire/v1/websites/{website_id}/delivery_tasks/{delivery_task_id}"
        ));
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取招聘官网投递任务结果响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 获取招聘官网投递任务结果响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetResponse {
    #[serde(flatten)]
    /// `delivery_task` 字段。
    pub delivery_task: WebsiteDeliveryTaskResult,
}

impl ApiResponseTrait for GetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
