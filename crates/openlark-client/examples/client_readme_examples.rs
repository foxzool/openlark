//! Compile-check companion for the public `openlark-client` README snippets.

use openlark_client::prelude::*;
use std::time::Duration;

fn builder_example() -> Result<()> {
    let _client = Client::builder()
        .app_id("your_app_id")
        .app_secret("your_app_secret")
        .base_url("https://open.feishu.cn")
        .timeout(Duration::from_secs(30))
        .build()?;
    Ok(())
}

fn from_env_example() -> Result<()> {
    let _client = Client::from_env()?;
    Ok(())
}

fn core_config_example() -> Result<()> {
    let config = CoreConfig::builder()
        .app_id("your_app_id")
        .app_secret("your_app_secret")
        .base_url("https://open.feishu.cn")
        .build();

    let _client = Client::with_core_config(config)?;
    Ok(())
}

fn service_access_example() -> Result<()> {
    let client = Client::builder()
        .app_id("your_app_id")
        .app_secret("your_app_secret")
        .build()?;
    let _config = client.config();

    #[cfg(feature = "docs")]
    let _docs_config = client.docs.config();

    #[cfg(feature = "communication")]
    let _communication = &client.communication;

    Ok(())
}

fn registry_example() -> Result<()> {
    let client = Client::builder()
        .app_id("your_app_id")
        .app_secret("your_app_secret")
        .build()?;

    for entry in client.registry().list_services() {
        let _ = &entry.metadata.name;
    }

    let _docs_enabled = client.registry().has_service("docs");
    Ok(())
}

fn main() -> Result<()> {
    builder_example()?;
    if std::env::var_os("OPENLARK_APP_ID").is_some()
        && std::env::var_os("OPENLARK_APP_SECRET").is_some()
    {
        from_env_example()?;
    }
    core_config_example()?;
    service_access_example()?;
    registry_example()?;
    Ok(())
}
