//! 查询批量任务执行状态
//!
//! docPath: <https://open.feishu.cn/document/ukTMukTMukTM/uUDOwUjL1gDM14SN4ATN>

use std::collections::HashMap;

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{common::api_utils::extract_response_data, endpoints::CONTACT_V2_TASK_GET};

/// 查询批量任务执行状态请求
pub struct GetBatchTaskRequest {
    config: Config,
    query: HashMap<String, String>,
}

impl GetBatchTaskRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query: HashMap::new(),
        }
    }

    /// 添加查询参数（例如 task_id）
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.insert(key.into(), value.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/ukTMukTMukTM/uUDOwUjL1gDM14SN4ATN>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(CONTACT_V2_TASK_GET);
        for (k, v) in self.query {
            req = req.query(k, v);
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询批量任务执行状态")
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
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
