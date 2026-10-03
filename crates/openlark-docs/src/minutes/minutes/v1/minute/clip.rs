//! 创建妙记片段
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/minutes-v1/minute/clip>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::minutes::MinutesExtraApiV1;
use crate::common::api_utils::serialize_params;

/// 片段时间区间。
///
/// `start_time` / `end_time` 官方标为非必填；区间合并与最短时长由服务端校验。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MinuteTimeRange {
    /// 起始时间（毫秒）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 截止时间（毫秒）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

/// 创建妙记片段请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MinuteClipBody {
    /// 片段时间区间（1–50 条）。
    pub time_ranges: Vec<MinuteTimeRange>,
    /// 片段标题。不填则生成默认标题。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// 创建妙记片段响应 data。
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct MinuteClipResponse {
    /// 妙记链接。成功只表示片段创建已提交，不表示转写/媒体已生成。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minute_url: Option<String>,
}

impl ApiResponseTrait for MinuteClipResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建妙记片段请求。
#[derive(Debug, Clone)]
pub struct MinuteClipRequest {
    config: Config,
    minute_token: String,
}

impl MinuteClipRequest {
    /// 创建请求。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            minute_token: String::new(),
        }
    }

    /// 设置路径参数 `minute_token`。
    pub fn minute_token(mut self, minute_token: impl Into<String>) -> Self {
        self.minute_token = minute_token.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self, body: MinuteClipBody) -> SDKResult<MinuteClipResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: MinuteClipBody,
        option: RequestOption,
    ) -> SDKResult<MinuteClipResponse> {
        validate_required!(self.minute_token, "minute_token 不能为空");
        validate_required_list!(body.time_ranges, 50, "time_ranges 不能为空且不能超过 50 个");
        let req: ApiRequest<MinuteClipResponse> = MinutesExtraApiV1::Clip(self.minute_token)
            .to_request()
            .body(serialize_params(&body, "创建妙记片段")?);
        Transport::request_typed(req, &self.config, Some(option), "创建妙记片段").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::api_endpoints::CatalogEndpoint;
    use crate::common::test_utils::user_test_transport;
    use openlark_core::constants::AccessTokenType;
    use serde_json::json;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, ResponseTemplate};

    fn sample_range() -> MinuteTimeRange {
        MinuteTimeRange {
            start_time: Some("1000".to_string()),
            end_time: Some("20000".to_string()),
        }
    }

    #[test]
    fn extra_catalog_clip_is_user_only_post() {
        let endpoint = MinutesExtraApiV1::Clip("obcnq3b9jl72l83w4f14xxxx".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/minutes/v1/minutes/obcnq3b9jl72l83w4f14xxxx/clip"
        );
        let req: ApiRequest<()> = endpoint.to_request();
        assert_eq!(
            req.supported_access_token_types(),
            vec![AccessTokenType::User]
        );
    }

    #[tokio::test]
    async fn empty_minute_token_fails_before_request() {
        let err = MinuteClipRequest::new(Config::default())
            .minute_token("   ")
            .execute(MinuteClipBody {
                time_ranges: vec![sample_range()],
                title: None,
            })
            .await
            .expect_err("空白 minute_token 应校验失败");
        assert!(err.to_string().contains("minute_token"));
    }

    #[tokio::test]
    async fn empty_time_ranges_fails_before_request() {
        let err = MinuteClipRequest::new(Config::default())
            .minute_token("obcnq3b9jl72l83w4f14xxxx")
            .execute(MinuteClipBody {
                time_ranges: vec![],
                title: None,
            })
            .await
            .expect_err("空 time_ranges 应校验失败");
        assert!(err.to_string().contains("time_ranges"));
    }

    #[tokio::test]
    async fn too_many_time_ranges_fails_before_request() {
        let err = MinuteClipRequest::new(Config::default())
            .minute_token("obcnq3b9jl72l83w4f14xxxx")
            .execute(MinuteClipBody {
                time_ranges: vec![MinuteTimeRange::default(); 51],
                title: None,
            })
            .await
            .expect_err("超过 50 条 time_ranges 应校验失败");
        assert!(err.to_string().contains("time_ranges"));
    }

    #[tokio::test]
    async fn clip_uses_user_token_and_typed_body() {
        let (server, config, option) = user_test_transport().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/minutes/v1/minutes/obcnq3b9jl72l83w4f14xxxx/clip",
            ))
            .and(header("Authorization", "Bearer test-user-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "minute_url": "https://sample.feishu.cn/minutes/clip-1" }
            })))
            .mount(&server)
            .await;

        let response = MinuteClipRequest::new(config)
            .minute_token("obcnq3b9jl72l83w4f14xxxx")
            .execute_with_options(
                MinuteClipBody {
                    time_ranges: vec![sample_range()],
                    title: Some("快速上手飞书妙记-片段".to_string()),
                },
                option,
            )
            .await
            .expect("创建妙记片段应成功");
        assert_eq!(
            response.minute_url.as_deref(),
            Some("https://sample.feishu.cn/minutes/clip-1")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为合法 JSON");
        assert_eq!(body["time_ranges"][0]["start_time"], "1000");
        assert_eq!(body["time_ranges"][0]["end_time"], "20000");
        assert_eq!(body["title"], "快速上手飞书妙记-片段");
    }
}
