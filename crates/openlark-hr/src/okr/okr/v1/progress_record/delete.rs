//! 删除 OKR 进展记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v1/progress_record/delete>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 删除 OKR 进展记录请求
#[derive(Debug, Clone)]
pub struct DeleteRequest {
    /// 进展记录 ID（必填）
    progress_id: String,
    /// 配置信息
    config: Config,
}

impl DeleteRequest {
    /// 创建请求
    pub fn new(config: Config, progress_id: String) -> Self {
        Self {
            progress_id,
            config,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DeleteResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteResponse> {
        use crate::common::api_endpoints::OkrApiV1;

        // 1. 验证必填字段
        validate_required!(self.progress_id.trim(), "进展记录 ID 不能为空");

        // 2. 构建端点
        let api_endpoint = OkrApiV1::ProgressRecordDelete(self.progress_id.clone());
        let request = ApiRequest::<DeleteResponse>::delete(api_endpoint.to_url());

        // 3. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 4. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "删除 OKR 进展记录响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 删除 OKR 进展记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteResponse {
    /// 删除结果
    pub result: bool,
}

impl ApiResponseTrait for DeleteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_okr_v1_progress_record_delete_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value = serde_json::from_str(r#"{"result": false}"#).unwrap();
        Mock::given(method("DELETE"))
            .and(path("/open-apis/okr/v1/progress_records/progress_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": data_body
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = DeleteRequest::new(config, "progress_001".to_string())
            .execute()
            .await
            .expect("okr_v1_progress_record_delete 应成功");

        let _ = data.result;

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/okr/v1/progress_records/progress_001"
        );
    }
}
