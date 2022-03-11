#[derive(Debug, PartialEq, Eq, Clone)]
pub enum OrderResultType {
    Queued,
    Executed,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum OrderType {
    Buy,
    Sell,
}
#[derive(Debug)]
pub struct OrderResult {
    pub result_type: OrderResultType,
    pub order_type: OrderType,
    pub quantity: u64,
    pub price: f32,
}

impl OrderResult {
    pub fn new(
        result_type: OrderResultType,
        order_type: OrderType,
        quantity: u64,
        price: f32,
    ) -> OrderResult {
        OrderResult {
            result_type,
            order_type,
            quantity,
            price,
        }
    }
}
