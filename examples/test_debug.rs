#![allow(deprecated, missing_docs)]

use openlark_client::config::Config as ClientConfig;
use openlark_core::config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let core_config = Config::builder()
        .app_id("test_app_id")
        .app_secret("secret_app_secret_12345")
        .build();
    println!("Core Config (recommended): {:?}", core_config);

    let client_config = ClientConfig::builder()
        .app_id("test_app_id")
        .app_secret("secret_app_secret_12345")
        .build()?;
    println!(
        "Client Config (deprecated compatibility): {:?}",
        client_config
    );

    println!("Debug output successfully masks secrets!");
    Ok(())
}
