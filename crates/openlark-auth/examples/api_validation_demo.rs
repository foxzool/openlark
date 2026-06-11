//! OpenLark Auth模块API验证示例
//!
//! 本示例演示如何使用openlark-auth模块的所有11个认证API

use openlark_auth::{AuthService, AuthenService, OAuthService};
use openlark_core::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建配置（实际使用时需要配置正确的参数）
    let config = Config::default();

    println!("🔐 OpenLark Auth API 验证演示");
    println!("================================");

    // 1. Auth v3 APIs (5个)
    println!("\n📋 Auth v3 APIs (5个):");
    let auth_service = AuthService::new(config.clone()).v3();

    println!("✅ 1. 商店应用获取app_access_token");
    let _builder1 = auth_service
        .app_access_token()
        .app_id("app_id")
        .app_secret("app_secret")
        .app_ticket("app_ticket");

    println!("✅ 2. 自建应用获取app_access_token_internal");
    let _builder2 = auth_service
        .app_access_token_internal()
        .app_id("app_id")
        .app_secret("app_secret");

    println!("✅ 3. 商店应用获取tenant_access_token");
    let _builder3 = auth_service
        .tenant_access_token()
        .app_access_token("app_access_token")
        .tenant_key("tenant_key");

    println!("✅ 4. 自建应用获取tenant_access_token_internal");
    let _builder4 = auth_service
        .tenant_access_token_internal()
        .app_id("app_id")
        .app_secret("app_secret");

    println!("✅ 5. 重新获取app_ticket");
    let _builder5 = auth_service
        .app_ticket_resend()
        .app_id("app_id")
        .app_secret("app_secret");

    // 2. Authen v1 APIs (5个)
    println!("\n👤 Authen v1 APIs (5个):");
    let authen_service = AuthenService::new(config.clone()).v1();

    println!("✅ 1. 获取用户信息 (user_info.get)");
    let _builder6 = authen_service
        .user_info()
        .get()
        .user_access_token("user_token")
        .user_id_type("open_id");

    println!("✅ 2. 获取用户访问令牌 (access_token.create)");
    let _builder7 = authen_service
        .access_token()
        .grant_code("grant_code")
        .app_id("app_id")
        .app_secret("app_secret");

    println!("✅ 3. 刷新用户访问令牌 (refresh_access_token.create)");
    let _builder8 = authen_service
        .refresh_access_token()
        .refresh_token("refresh_token")
        .app_id("app_id")
        .app_secret("app_secret");

    println!("✅ 4. 获取OIDC用户访问令牌 (oidc.access_token.create)");
    let _builder9 = authen_service
        .oidc()
        .access_token()
        .code("auth_code")
        .code_verifier("code_verifier")
        .redirect_uri("https://example.com/callback")
        .client_id("client_id")
        .client_secret("client_secret");

    println!("✅ 5. 刷新OIDC用户访问令牌 (oidc.refresh_access_token.create)");
    let _builder10 = authen_service
        .oidc()
        .refresh_access_token()
        .refresh_token("refresh_token")
        .client_id("client_id")
        .client_secret("client_secret");

    // 3. OAuth APIs (1个)
    println!("\n🔗 OAuth APIs (1个):");
    let oauth_service = OAuthService::new(config.clone()).old();

    println!("✅ 1. 获取登录预授权码 (v1/index)");
    let auth_url = oauth_service
        .authorization()
        .app_id("app_id")
        .redirect_uri("https://example.com/callback")
        .scope("user_info:read")
        .state("random_state")
        .build_url();

    println!("   授权URL: {auth_url}");

    // 总结
    println!("\n🎯 API验证总结:");
    println!("   ✅ Auth APIs: 5/5");
    println!("   ✅ Authen APIs: 5/5");
    println!("   ✅ OAuth APIs: 1/1");
    println!("   🎉 总计: 11/11 API 构建器创建成功!");

    println!("\n📝 注意: 本示例仅验证API构建器是否可以正确创建，不执行实际的网络请求。");
    println!("   要执行实际API调用，请配置有效的app_id、app_secret等参数。");

    Ok(())
}
