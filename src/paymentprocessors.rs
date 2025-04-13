use std::{env, str::FromStr};

use reqwest::Url;
use tracing::{event, Level};

use crate::{domain::Payment, dtos::{CreateCheckoutSessionRequestDto, CreateCheckoutSessionResponseDto}};

pub trait PaymentProcessor {
    async fn create_checkout_session(payment: &mut Payment) -> Result<&Payment, String>;
}

pub struct StripePaymentProcessor {
    base_redirect_url: String,
}

impl StripePaymentProcessor {
    pub fn new(base_redirect_url: String) -> Self {
        StripePaymentProcessor { 
            base_redirect_url: base_redirect_url
        }
    }
}

impl PaymentProcessor for StripePaymentProcessor{
    async fn create_checkout_session(payment: &mut Payment) -> Result<&Payment, String> {
        let create_checkout_session_request_dto = CreateCheckoutSessionRequestDto {
            ui_mode: String::from("custom"),
            mode: String::from("payment"),
            return_url: String::from(format!("{}/return?session_id={{CHECKOUT_SESSION_ID}}", String::from(env::var("PAYMENT_REDIRECT_BASE_URL").unwrap()))),
            line_items: Vec::new(),
        };

        let url = Url::from_str(&format!("{}/v1/checkout/sessions", String::from(env::var("STRIPE_API_BASE_URL").unwrap()))).unwrap();

        let http_client = reqwest::Client::new();
        match http_client.post(url)
            .header("Bearer", String::from(env::var("STRIPE_API_KEY").unwrap()))
            .json(&create_checkout_session_request_dto)
            .send()
            .await {
                Ok(response) => {
                    match response.json::<CreateCheckoutSessionResponseDto>().await {
                        Ok(create_checkout_session_response_dto) => {
                            payment.payment_processor_checkout_session_id = create_checkout_session_response_dto.session_id;
                            payment.payment_processor_checkout_session_url = create_checkout_session_response_dto.session_url;

                            return Ok(payment);
                        },
                        Err(e) => {
                            event!(Level::WARN, "Error occurred when deserializing CreateCheckoutResponseDto: {}", e);
                            return Err(format!("Error occurred when deserializing CreateCheckoutResponseDto: {}", e));
                        }
                    }
                },
                Err(e) => {
                    event!(Level::WARN, "Error occurred when sending CreateCheckoutRequest to Stripe: {}", e);
                    return Err(format!("Error occurred when sending CreateCheckoutRequest to Stripe: {}", e));
                }
            };
    }
}
