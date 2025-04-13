use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LineItemRequestDto {
    pub price: String,
    pub quantity: u32,
}

#[derive(Serialize, Deserialize)]
pub struct CreateCheckoutSessionRequestDto {
    pub ui_mode: String,
    pub line_items: Vec<LineItemRequestDto>,
    pub mode: String,
    pub return_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateCheckoutSessionResponseDto {
    pub session_id: String,
    pub session_url: String,
}