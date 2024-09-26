use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
struct Delivery {
    name: String,
    phone: String,
    zip: String,
    city: String,
    address: String,
    region: String,
    email: String
}
#[derive(Deserialize, Serialize)]
struct Payment {
    transaction: String,
    request_id: String,
    currency: String,
    provider: String,
    amount: u64,
    payment_dt: u64,
    bank: String,
    delivery_cost: u64,
    goods_total: u64,
    custom_fee: u64
}
#[derive(Deserialize, Serialize)]
struct Item {
    chrt_id: u64,
    track_number: String,
    price: u64,
    rid: String,
    name: String,
    sale: u64,
    size: String,
    total_price: u64,
    nm_id: u64,
    brand: String,
    status: u64
}
#[derive(Deserialize, Serialize)]
pub struct WBmodel {
    order_uid: String,
    track_number: String,
    entry: String,
    delivery: Delivery,
    payment: Payment,
    items: Vec<Item>,
    locale: String,
    internal_signature: String,
    customer_id: String,
    delivery_service: String,
    shardkey: String,
    sm_id: u64,
    date_created: String,
    oof_shard: String
}
