//! 主数据管理模块（flat-by-design，ADR 0001）。
//!
//! 叶子 `new(Config)` 无路径参数 → 直路径访问（`crate::mdm::v1::*` / `crate::mdm::v3::*`），
//! 无 Service 壳。Service 层会是纯转发 shell（反 ADR），同 analytics 裁决；
//! `PlatformService` 故意不暴露 `mdm()` accessor。

pub mod v1;
pub mod v3;
