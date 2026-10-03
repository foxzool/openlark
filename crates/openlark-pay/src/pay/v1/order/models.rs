//! 商店付费订单模型。

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 付费方案类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PricePlanType {
    /// 试用。
    Trial,
    /// 永久。
    Permanent,
    /// 两年。
    TwoYear,
    /// 按年。
    PerYear,
    /// 按月。
    PerMonth,
    /// 按席位两年。
    PerSeatTwoYear,
    /// 按席位按年。
    PerSeatPerYear,
    /// 按席位按月。
    PerSeatPerMonth,
    /// 永久数量。
    PermanentCount,
}

/// 订单状态（响应）。
///
/// 与列表请求过滤值 `refunded` 拼写不同，各自照抄。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    /// 正常。
    Normal,
    /// 已退款。
    Refund,
}

/// 购买类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderBuyType {
    /// 新购。
    Buy,
    /// 升级。
    Upgrade,
    /// 续费。
    Renew,
}

/// 订单列表请求的 status 过滤值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderListStatus {
    /// 正常。
    Normal,
    /// 已退款。
    Refunded,
    /// 全部。
    All,
}

impl OrderListStatus {
    /// 查询参数字面量。
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Refunded => "refunded",
            Self::All => "all",
        }
    }
}

/// 付费订单。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Order {
    /// 订单 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 付费方案 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_plan_id: Option<String>,
    /// 付费方案类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_plan_type: Option<PricePlanType>,
    /// 席位数。仅 `per_seat_per_year` / `per_seat_per_month` 有意义。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seats: Option<i32>,
    /// 购买数量。文档标明恒为 1。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_count: Option<i32>,
    /// 创建时间，秒级字符串时间戳。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 支付时间，秒级字符串时间戳。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_time: Option<String>,
    /// 订单状态。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatus>,
    /// 购买类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_type: Option<OrderBuyType>,
    /// 升级前源订单 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_order_id: Option<String>,
    /// 升级后目标订单 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_order_id: Option<String>,
    /// 实付金额，单位分。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_pay_price: Option<i64>,
    /// 租户 key。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_key: Option<String>,
}

/// 查询订单详情响应 `data`。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GetOrderResponse {
    /// 订单详情。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
}

impl ApiResponseTrait for GetOrderResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询租户购买的付费方案响应 `data`。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ListOrdersResponse {
    /// 订单总数。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// 是否还有更多页。`false` 时 `page_token` 为空。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 下一页标记。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 订单列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_list: Option<Vec<Order>>,
}

impl ApiResponseTrait for ListOrdersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn order_status_keeps_refund_spelling() {
        let status: OrderStatus = serde_json::from_value(json!("refund")).unwrap();
        assert_eq!(status, OrderStatus::Refund);
        assert_eq!(
            serde_json::to_value(OrderStatus::Refund).unwrap(),
            json!("refund")
        );
        assert_eq!(OrderListStatus::Refunded.as_str(), "refunded");
    }

    #[test]
    fn price_plan_type_nine_values() {
        let values = [
            "trial",
            "permanent",
            "two_year",
            "per_year",
            "per_month",
            "per_seat_two_year",
            "per_seat_per_year",
            "per_seat_per_month",
            "permanent_count",
        ];
        for value in values {
            let parsed: PricePlanType = serde_json::from_value(json!(value)).unwrap();
            assert_eq!(serde_json::to_value(parsed).unwrap(), json!(value));
        }
    }
}
