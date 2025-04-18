use std::{env, str::FromStr};

use async_trait::async_trait;
use reqwest::Url;
use tracing::{event, Level};

use crate::{domain::Payment, dtos::{PaymentProcessorCreateCheckoutSessionRequestDto, PaymentProcessorCreatePricingRequestDto, PaymentProcessorCreateProductRequestDto, PaymentProcessorLineItemRequestDto}};

#[async_trait]
pub trait PaymentProcessor {
    async fn create_checkout_session(&self, payment: Payment) -> Result<Payment, String>;
    async fn create_product(&self, product_id: String, name: String) -> Result<(), String>;
    async fn create_product_pricing(&self, product_id: String, currency: String, unit_amount: i32) -> Result<(), String>;
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

#[async_trait]
impl PaymentProcessor for StripePaymentProcessor{
    async fn create_checkout_session(&self, mut payment: Payment) -> Result<Payment, String> {
        let create_checkout_session_request_dto = PaymentProcessorCreateCheckoutSessionRequestDto {
            ui_mode: String::from("custom"),
            mode: String::from("payment"),
            return_url: String::from(format!("{}/return?session_id={{CHECKOUT_SESSION_ID}}", String::from(env::var("PAYMENT_REDIRECT_BASE_URL").unwrap()))),
            line_items: vec![
                PaymentProcessorLineItemRequestDto{
                    price: String::from("10000"),
                    quantity: 10
                }
            ],
        };

        // serde_qs (query string) must be used to manually serialize the object before passing to reqwest
        // reqwest is using serde_urlencoded internally which has unresolved issues with collection
        // https://github.com/wyyerd/stripe-rs/pull/23/commits
        let form_url_encoded_request = serde_qs::to_string(&create_checkout_session_request_dto).unwrap();

        let url = Url::from_str(&format!("{}/v1/checkout/sessions", String::from(env::var("STRIPE_API_BASE_URL").unwrap()))).unwrap();

        let http_client = reqwest::Client::new();
        match http_client.post(url)
            .header(reqwest::header::CONTENT_TYPE, String::from("application/x-www-form-urlencoded"))
            .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", String::from(env::var("STRIPE_API_KEY").unwrap())))
            .body(form_url_encoded_request)
            .send()
            .await {
                Ok(response) => {
                    println!("{}", &response.text().await.unwrap());
                    // match response.json::<PaymentProcessorCreateCheckoutSessionResponseDto>().await {
                    //     Ok(create_checkout_session_response_dto) => {
                    //         payment.payment_processor_checkout_session_id = create_checkout_session_response_dto.session_id;
                    //         payment.payment_processor_checkout_session_url = create_checkout_session_response_dto.session_url;

                    //         return Ok(payment);
                    //     },
                    //     Err(e) => {
                    //         event!(Level::WARN, "Error occurred when deserializing CreateCheckoutResponseDto: {}", e);
                    //         return Err(format!("Error occurred when deserializing CreateCheckoutResponseDto: {}", e));
                    //     }
                    // }
                    return Ok(payment);
                },
                Err(e) => {
                    event!(Level::WARN, "Error occurred when sending CreateCheckoutRequest to Stripe: {}", e);
                    return Err(format!("Error occurred when sending CreateCheckoutRequest to Stripe: {}", e));
                }
            };
    }

    async fn create_product(&self, product_id: String, name: String) -> Result<(), String> {
        let payment_processor_create_product_request_dto = PaymentProcessorCreateProductRequestDto {
            id: product_id,
            name: name,
        };

        let form_url_encoded_request = serde_qs::to_string(&payment_processor_create_product_request_dto).unwrap();

        let url = Url::from_str(&format!("{}/v1/products", String::from(env::var("STRIPE_API_BASE_URL").unwrap()))).unwrap();

        let http_client = reqwest::Client::new();
        match http_client.post(url)
            .header(reqwest::header::CONTENT_TYPE, String::from("application/x-www-form-urlencoded"))
            .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", String::from(env::var("STRIPE_API_KEY").unwrap())))
            .body(form_url_encoded_request)
            .send()
            .await {
                Ok(_) => {
                    Ok(())
                },
                Err(e) => {
                    event!(Level::WARN, "Error occurred when sending CreateProductRequest to Stripe: {}", e);
                    return Err(format!("Error occurred when sending CreateProductRequest to Stripe: {}", e));
                }
            }
    }

    async fn create_product_pricing(&self, product_id: String, currency: String, unit_amount: i32) -> Result<(), String> {
        let payment_processor_create_pricing_request_dto = PaymentProcessorCreatePricingRequestDto {
            product: product_id,
            currency: currency,
            unit_amount: unit_amount * 100, // Stripe's unit amount is in cents
        };

        let form_url_encoded_request = serde_qs::to_string(&payment_processor_create_pricing_request_dto).unwrap();

        let url = Url::from_str(&format!("{}/v1/prices", String::from(env::var("STRIPE_API_BASE_URL").unwrap()))).unwrap();

        let http_client = reqwest::Client::new();
        match http_client.post(url)
            .header(reqwest::header::CONTENT_TYPE, String::from("application/x-www-form-urlencoded"))
            .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", String::from(env::var("STRIPE_API_KEY").unwrap())))
            .body(form_url_encoded_request)
            .send()
            .await {
                Ok(_) => {
                    Ok(())
                },
                Err(e) => {
                    event!(Level::WARN, "Error occurred when sending CreateProductRequest to Stripe: {}", e);
                    return Err(format!("Error occurred when sending CreateProductRequest to Stripe: {}", e));
                }
            }
    }
}
