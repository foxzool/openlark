//! compile-fail：当对应 feature 禁用时，Client 上不得存在该 meta 字段。
//!
//! #423 / #435 要求：disabled feature 不产生字段。
//! 使用 trybuild 在 openlark-client 默认无 feature 的 dev-dep 上下文中验证访问失败。

fn main() {
    // 把非法字段访问放在 probe fn 体内，确保字段解析发生（产生 E0609），同时避免主路径 unreachable 警告。
    #[allow(dead_code, unused_variables)]
    fn probe_client_fields(c: openlark_client::Client) {
        // 当 openlark-client 以 default-features=false 且无 feature 编译时，此字段访问必须编译失败。
        let _ = &c.auth;
    }
}
