pub struct LineItem {
    pub product_id: String,
    pub quantity: u32,
    pub price: f32
}

pub struct Payment {
    pub id: String,
    pub products: Vec<LineItem>,
    pub status: String,
    pub payment_processor: String,
    pub payment_processor_checkout_session_id: String,
    pub payment_processor_checkout_session_url: String,
    pub payment_processor_id: String,
    pub payment_processor_status: String,
}