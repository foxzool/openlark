//! 商店付费 API 端点。

/// Pay v1 端点。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PayApiV1 {
    /// 查询订单详情。
    /// GET /open-apis/pay/v1/order/get
    OrderGet,
    /// 查询租户购买的付费方案。
    /// GET /open-apis/pay/v1/order/list
    OrderList,
    /// 查询用户是否在应用开通范围。
    /// GET /open-apis/pay/v1/paid_scope/check_user
    PaidScopeCheckUser,
}

impl PayApiV1 {
    /// 生成对应 URL。
    pub fn to_url(self) -> &'static str {
        match self {
            Self::OrderGet => "/open-apis/pay/v1/order/get",
            Self::OrderList => "/open-apis/pay/v1/order/list",
            Self::PaidScopeCheckUser => "/open-apis/pay/v1/paid_scope/check_user",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PayApiV1;

    #[test]
    fn pay_api_v1_urls() {
        assert_eq!(PayApiV1::OrderGet.to_url(), "/open-apis/pay/v1/order/get");
        assert_eq!(PayApiV1::OrderList.to_url(), "/open-apis/pay/v1/order/list");
        assert_eq!(
            PayApiV1::PaidScopeCheckUser.to_url(),
            "/open-apis/pay/v1/paid_scope/check_user"
        );
    }
}
