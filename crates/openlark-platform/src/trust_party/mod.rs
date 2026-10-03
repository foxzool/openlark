//! 关联组织模块（flat-by-design，ADR 0001）。
//!
//! 叶子 `new(Config)` 无路径参数 → 直路径访问（`crate::trust_party::v1::*`），无 Service 壳
//! （同 analytics / mdm 裁决）。`PlatformService` 故意不暴露 `trust_party()` accessor。

pub mod v1;
