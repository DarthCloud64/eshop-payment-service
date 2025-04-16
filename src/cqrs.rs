use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tracing::{event, Level};

use crate::{domain::{Payment, PaymentStatus}, dtos::{CreateCheckoutSessionResponseDto, LineItemRequestDto, Response}, paymentprocessors::PaymentProcessor};

// traits
pub trait Command{}
pub trait Query{}

pub trait CommandHandler<C: Command, R: Response>{
    async fn handle(&self, input: &C) -> Result<R, String>;
}

pub trait QueryHandler<Q: Query, R: Response>{
    async fn handle(&self, input: Option<Q>) -> Result<R, String>;
}

#[derive(Serialize, Deserialize)]
pub struct CreateCheckoutSessionCommand {
    pub line_items: Vec<LineItemRequestDto>
}
impl Command for CreateCheckoutSessionCommand{}

pub struct CreateCheckoutSessionCommandHandler {
    payment_processor: Arc<dyn PaymentProcessor + Send + Sync>
}

impl CreateCheckoutSessionCommandHandler {
    pub fn new(payment_processor: Arc<dyn PaymentProcessor + Send + Sync>) -> Self {
        CreateCheckoutSessionCommandHandler { 
            payment_processor: payment_processor 
        }
    }
}

impl CommandHandler<CreateCheckoutSessionCommand, CreateCheckoutSessionResponseDto> for CreateCheckoutSessionCommandHandler {
    async fn handle(&self, input: &CreateCheckoutSessionCommand) -> Result<CreateCheckoutSessionResponseDto, String> {
        let payment = Payment {
            id: uuid::Uuid::new_v4().to_string(),
            line_items: Vec::new(),
            status: PaymentStatus::NEW.to_string(),
            payment_processor: String::new(),
            payment_processor_checkout_session_id: String::new(),
            payment_processor_checkout_session_url: String::new(),
            payment_processor_id: String::new(),
            payment_processor_status: String::new(),
        };

        match self.payment_processor.as_ref().create_checkout_session(payment).await {
            Ok(payment_with_session_info) => {
                Ok(CreateCheckoutSessionResponseDto {
                    payment_id: payment_with_session_info.id,
                    checkout_session_id: payment_with_session_info.payment_processor_checkout_session_id,
                    checkout_session_url: payment_with_session_info.payment_processor_checkout_session_url,
                })
            },
            Err(e) => {
                event!(Level::WARN, "Error occurred when creating checkout session: {}", e);
                return Err(format!("Error occurred when creating checkout session: {}", e));
            }
        }
    }
}