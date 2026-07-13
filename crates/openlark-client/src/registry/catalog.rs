//! Legacy compiled-services 表（已清空）。
//!
//! #434–#436 后全部业务域元数据由 `crate::capability` catalog 生成。
//! 本模块保留 no-op 注册，供 bootstrap 在 #437 收缩 legacy 路径前保持调用形状稳定。

use super::super::DefaultServiceRegistry;
use crate::Result;

pub(crate) fn register_compiled_services(
    _registry: &mut DefaultServiceRegistry,
) -> Result<()> {
    Ok(())
}

#[cfg(test)]
pub(super) fn compiled_service_names() -> Vec<&'static str> {
    Vec::new()
}
