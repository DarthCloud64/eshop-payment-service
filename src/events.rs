use std::sync::Arc;

use amqprs::{callbacks::{DefaultChannelCallback, DefaultConnectionCallback}, channel::{BasicConsumeArguments, Channel, ExchangeDeclareArguments, ExchangeType, QueueBindArguments, QueueDeclareArguments}, connection::{Connection, OpenConnectionArguments}, consumer::AsyncConsumer, BasicProperties, Deliver};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::{event, Level};

use crate::state::AppState;

pub static PRODUCT_CREATED_QUEUE_NAME: &str = "product.created";

pub struct RabbitMqInitializationInfo {
    uri: String,
    port: u16,
    username: String,
    password: String
}

impl RabbitMqInitializationInfo {
    pub fn new(
        uri: String,
        port: u16,
        username: String,
        password: String) -> RabbitMqInitializationInfo{
            RabbitMqInitializationInfo {
                uri: uri,
                port: port,
                username: username,
                password: password
            }
        }
}

#[derive(Serialize, Deserialize)]
pub enum Event {
    ProductCreatedEvent {
        product_id: String
    },
}

pub trait MessageBroker {
    async fn publish_message(&self, event: &Event) -> Result<(), String>;
    async fn consume(&self, source_queue_name: &'static str, state: Arc<AppState>);
}

pub struct RabbitMqMessageBroker {
    connection: Connection,
}

impl RabbitMqMessageBroker {
    pub async fn new(init_info: RabbitMqInitializationInfo) -> Result<RabbitMqMessageBroker, String>{
        match Connection::open(&OpenConnectionArguments::new(&init_info.uri, init_info.port, &init_info.username, &init_info.password)
        ).await {
            Ok(connection) => {
                match connection.register_callback(DefaultConnectionCallback).await {
                    Ok(()) => {
                        Ok(RabbitMqMessageBroker{
                            connection: connection
                        })
                    },
                    Err(e) => {
                        Err(format!("Failed to register connection callback: {}", e))
                    }
                }
            },
            Err(e) => {
                Err(format!("Failed to open RabbitMQ connection: {}", e))
            }
        }
    }

    pub async fn get_channel(&self, destination: &str) -> Result<Channel, String>{
        match self.connection.open_channel(None).await{
            Ok(channel) => {
                channel.register_callback(DefaultChannelCallback).await.unwrap();
                channel.exchange_declare(ExchangeDeclareArguments::new(destination, &ExchangeType::Fanout.to_string())).await.unwrap();
                channel.queue_declare(QueueDeclareArguments::durable_client_named(destination)).await.unwrap();
                channel.queue_bind(QueueBindArguments::new(destination, destination, "")).await.unwrap();

                Ok(channel)
            },
            Err(e) => {
                Err(format!("Failed to get channel: {}", e))
            }
        }
    }
}

impl MessageBroker for RabbitMqMessageBroker {
    async fn publish_message(&self, event: &Event) -> Result<(), String> {
        todo!();
    }

    async fn consume(&self, source_queue_name: &'static str, state: Arc<AppState>) {
        match self.get_channel(source_queue_name).await {
            Ok(channel) => {
                let consume_arguments = BasicConsumeArguments::new(source_queue_name, "eshop-payment-service")
                    .manual_ack(false)
                    .finish();
                
                match source_queue_name {
                    queue_name if queue_name == PRODUCT_CREATED_QUEUE_NAME => {
                        channel.basic_consume(ProductCreatedEventHandler::new(state.clone()), consume_arguments).await.unwrap();
                    },
                    x => event!(Level::INFO, "event {} is not valid to subscribe to", x)
                }
            },
            Err(e) => {
                panic!();
            }
        }
    }
}

pub struct ProductCreatedEventHandler {
    state: Arc<AppState>,
}

impl ProductCreatedEventHandler {
    pub fn new(state: Arc<AppState>) -> Self {
        ProductCreatedEventHandler { 
            state: state,
        }
    }
}

#[async_trait]
impl AsyncConsumer for ProductCreatedEventHandler {
    async fn consume(
        &mut self,
        _: &Channel,
        _: Deliver,
        _: BasicProperties,
        content: Vec<u8>,
    ){
        let raw_event = String::from_utf8(content).unwrap();
        event!(Level::DEBUG, "Received event: {}", raw_event);

        match serde_json::from_str::<Event>(&raw_event) {
            Ok(deserialized_event) => {
                match deserialized_event {
                    Event::ProductCreatedEvent { product_id } => {

                    },
                    _ => event!(Level::INFO, "Event not supported")
                }
            },
            Err(e) => {
                event!(Level::WARN, "Failed to deserialize event {}: {}", raw_event, e);
            }
        }
    }
}