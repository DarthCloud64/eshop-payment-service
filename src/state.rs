use std::sync::Arc;

use crate::cqrs::{CreateCheckoutSessionCommandHandler, CreateProductPricingCommandHandler};

#[derive(Clone)]
pub struct AppState {
    pub create_checkout_session_command_handler: Arc<CreateCheckoutSessionCommandHandler>,
    pub create_product_pricing_command_handler: Arc<CreateProductPricingCommandHandler>,
    pub auth0_domain: String,
    pub auth0_audience: String,
}