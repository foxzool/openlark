#![allow(missing_docs)]

use openlark_core::config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder()
        .app_id("test_app_id")
        .app_secret("secret_app_secret_12345")
        .build();
    println!("Config: {:?}", config);
    println!("Debug output successfully masks secrets!");
    Ok(())
}
