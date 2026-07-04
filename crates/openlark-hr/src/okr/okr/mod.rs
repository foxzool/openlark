/// okr 项目模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织

/// v1 子模块。
pub mod v1;
/// v2 子模块。
pub mod v2;

/// okr 项目 v2 版本服务
/// OkrV2 服务入口。
pub type OkrV2 = v2::OkrV2;
