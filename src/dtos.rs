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

#[derive(Deserialize, Serialize)]
pub struct ProductResponse{
    pub id: String,
    pub name: String,
    pub price: f32,
    pub description: String,
    pub inventory: u32,
    pub stars: u8,
    pub number_of_reviews: u32,
}

#[derive(Deserialize, Serialize)]
pub struct GetProductsResponse{
    pub products: Vec<ProductResponse>
}
impl Response for GetProductsResponse{}

#[derive(Deserialize, Serialize)]
pub struct EmptyResponse{}
impl Response for EmptyResponse{}

#[derive(Deserialize, Serialize)]
pub struct PaymentProcessorCreateProductRequestDto {
    pub id: String,
    pub name: String
}

#[derive(Deserialize, Serialize)]
pub struct PaymentProcessorCreatePricingRequestDto {
    pub product: String,
    pub currency: String,
    pub unit_amount: i32,
}