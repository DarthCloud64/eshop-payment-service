pub struct LineItem {
    pub product_id: String,
    pub quantity: u32,
    pub price: f32
}

pub struct Payment {
    pub id: String,
    pub line_items: Vec<LineItem>,
    pub status: String,
    pub payment_processor: String,
    pub payment_processor_checkout_session_id: String,
    pub payment_processor_checkout_session_url: String,
    pub payment_processor_id: String,
    pub payment_processor_status: String,
}

#[derive(Debug)]
pub enum PaymentStatus {
    NEW,
}

impl ToString for PaymentStatus {
    fn to_string(&self) -> String {
        match self {
            PaymentStatus::NEW => String::from("New")
        }
    }
}