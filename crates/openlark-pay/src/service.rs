use openlark_core::config::Config;

/// 商店付费服务统一入口。
///
/// 开启 `v1` feature 后提供订单详情、订单列表与开通范围查询。
#[derive(Clone, Debug)]
pub struct PayService {
    // config 仅在 v1 feature 开启时被 accessor 读取；feature 关闭时受控标注为预期死代码。
    #[cfg_attr(not(feature = "v1"), expect(dead_code))]
    config: Config,
}

impl PayService {
    /// 创建新的商店付费服务实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 查询订单详情。
    #[cfg(feature = "v1")]
    pub fn get_order(&self) -> crate::pay::v1::order::GetOrderRequest {
        crate::pay::v1::order::GetOrderRequest::new(self.config.clone())
    }

    /// 查询租户购买的付费方案。
    #[cfg(feature = "v1")]
    pub fn list_orders(&self) -> crate::pay::v1::order::ListOrdersRequest {
        crate::pay::v1::order::ListOrdersRequest::new(self.config.clone())
    }

    /// 查询用户是否在应用开通范围。
    #[cfg(feature = "v1")]
    pub fn check_user(&self) -> crate::pay::v1::paid_scope::CheckUserPaidScopeRequest {
        crate::pay::v1::paid_scope::CheckUserPaidScopeRequest::new(self.config.clone())
    }
}

#[cfg(all(test, feature = "v1"))]
mod tests {
    use super::*;

    #[test]
    fn service_constructs() {
        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let service = PayService::new(config);
        let _ = service.get_order();
        let _ = service.list_orders();
        let _ = service.check_user();
    }
}
