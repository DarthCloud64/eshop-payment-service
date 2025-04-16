use serde::{Deserialize, Serialize};

pub trait Response{}

#[derive(Serialize, Deserialize)]
pub struct LineItemRequestDto {
    pub product_id: String,
    pub quantity: u32,
    pub price: f32
}

#[derive(Serialize, Deserialize)]
pub struct PaymentProcessorLineItemRequestDto {
    pub price: String,
    pub quantity: u32,
}

#[derive(Serialize, Deserialize)]
pub struct CreateCheckoutSessionResponseDto {
    pub payment_id: String,
    pub checkout_session_id: String,
    pub checkout_session_url: String,
}
impl Response for CreateCheckoutSessionResponseDto{}

#[derive(Serialize, Deserialize)]
pub struct PaymentProcessorCreateCheckoutSessionRequestDto {
    pub ui_mode: String,
    pub line_items: Vec<PaymentProcessorLineItemRequestDto>,
    pub mode: String,
    pub return_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct PaymentProcessorCreateCheckoutSessionResponseDto {
    pub session_id: String,
    pub session_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiError {
    pub error: String
}
impl Response for ApiError{}