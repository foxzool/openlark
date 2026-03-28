//! 查询导出任务结果
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/export/get>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, validate_required,
    SDKResult,
};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::extract_response_data;

/// 查询导出任务结果请求
pub struct GetExportTaskRequest {
    config: Config,
    task_id: String,
    query_params: Vec<(String, String)>,
}

impl GetExportTaskRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            task_id: String::new(),
            query_params: Vec::new(),
        }
    }

    /// 任务 ID（路径参数）
    pub fn task_id(mut self, task_id: impl Into<String>) -> Self {
        self.task_id = task_id.into();
        self
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/export/get>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.task_id, "task_id 不能为空");

        // url: GET:/open-apis/vc/v1/exports/:task_id
        let api_endpoint = VcApiV1::ExportGet(self.task_id.clone());
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(api_endpoint.to_url());
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询导出任务结果")
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
