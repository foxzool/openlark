//! 应用票据（app_ticket）恢复（ADR-0002）：请求遇 app_ticket 失效时触发“重新推送”。
//!
//! `recover_app_ticket_if_needed` 由 `Transport::do_request` 在收到响应后按名委托调用；
//! 条件（code==ERR_CODE_APP_TICKET_INVALID）+ 动作（resend）+ 出口（UnifiedRequestBuilder
//! bootstrap 旁路）三者同居 auth/，`Transport` 不再编码业务错误码。

use crate::{
    SDKResult,
    api::{ApiRequest, RequestData},
    config::Config,
    constants::{APPLY_APP_TICKET_PATH, AccessTokenType, ERR_CODE_APP_TICKET_INVALID},
    req_option::RequestOption,
};

/// 请求遇 app_ticket 失效时触发重推（副作用操作，不返回业务数据）。
///
/// `Transport::do_request` 在收到响应后调用：
/// `auth::app_ticket::recover_app_ticket_if_needed(resp.is_success(), resp.raw_response.code, config)`。
pub(crate) async fn recover_app_ticket_if_needed(
    is_success: bool,
    code: i32,
    config: &Config,
) -> SDKResult<()> {
    if !is_success && code == ERR_CODE_APP_TICKET_INVALID {
        resend_app_ticket(config).await?;
    }
    Ok(())
}

/// 重新推送 app_ticket（bootstrap 旁路，ADR-0002 Q2）。
///
/// 经 `UnifiedRequestBuilder` 获得标准 header/timeout，但以 None-token bootstrap 身份
/// 发送，绕过 `Transport::request` 的 policy + recovery 管线——故构造上不可能递归
/// （resend 端点本身不被 app_ticket 门控，亦不会回到本恢复路径）。
pub(crate) async fn resend_app_ticket(config: &Config) -> SDKResult<()> {
    let url = format!("{}{}", config.base_url(), APPLY_APP_TICKET_PATH);
    let body = serde_json::json!({
        "app_id": config.app_id(),
        "app_secret": config.app_secret(),
    });
    let mut req = ApiRequest::<()>::post(url).body(RequestData::Json(body));
    let req_builder = crate::request_execution::UnifiedRequestBuilder::build(
        &mut req,
        AccessTokenType::None,
        config,
        &RequestOption::default(),
    )
    .await?;
    // build() 只声明 Content-Type；body 字节由此附加（镜像 Transport::do_send 的非 multipart 分支）。
    let _ = req_builder.body(req.to_bytes()).send().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::APPLY_APP_TICKET_PATH;
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn cfg(base_url: &str, app_id: &str, app_secret: &str) -> Config {
        Config::builder()
            .app_id(app_id)
            .app_secret(app_secret)
            .base_url(base_url)
            .enable_token_cache(false)
            .build()
    }

    #[tokio::test]
    async fn recover_noop_on_success() {
        // is_success=true → 不触发 resend → 不触网 → Ok
        let config = cfg("http://unreachable.invalid", "id", "sec");
        assert!(
            recover_app_ticket_if_needed(true, ERR_CODE_APP_TICKET_INVALID, &config)
                .await
                .is_ok()
        );
    }

    #[tokio::test]
    async fn recover_noop_on_unrelated_error_code() {
        let config = cfg("http://unreachable.invalid", "id", "sec");
        assert!(
            recover_app_ticket_if_needed(false, 99999, &config)
                .await
                .is_ok()
        );
    }

    #[tokio::test]
    async fn recover_triggers_resend_on_app_ticket_invalid() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(APPLY_APP_TICKET_PATH))
            .and(body_json(
                serde_json::json!({"app_id":"id","app_secret":"sec"}),
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"code":0})))
            .expect(1)
            .mount(&server)
            .await;

        let config = cfg(&server.uri(), "id", "sec");
        recover_app_ticket_if_needed(false, ERR_CODE_APP_TICKET_INVALID, &config)
            .await
            .expect("resend should succeed");
        // expect(1) 在 server drop 时校验：resend 必须被命中恰好一次。
    }
}
