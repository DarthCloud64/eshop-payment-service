use std::sync::Arc;

use crate::cqrs::CreateCheckoutSessionCommandHandler;

#[derive(Clone)]
pub struct AppState {
    pub create_checkout_session_command_handler: Arc<CreateCheckoutSessionCommandHandler>,
    pub auth0_domain: String,
    pub auth0_audience: String,
}