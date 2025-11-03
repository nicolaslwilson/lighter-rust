use crate::{
    config::LighterConfig,
    error::{LighterError, Result},
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

pub type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsRequest {
    pub method: String,
    pub params: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsResponse {
    pub id: Option<String>,
    pub result: Option<Value>,
    pub error: Option<WsError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsError {
    pub code: i32,
    pub message: String,
    pub data: Option<Value>,
}

#[derive(Debug, strum::Display, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum WsMessage {
    Connected,
    #[serde(rename = "subscribed/order_book")]
    SubscribedOrderBook,
    #[serde(rename = "update/order_book")]
    UpdateOrderBook,
    #[serde(rename = "subscribed/account_all")]
    SubscribedAccountAll,
    #[serde(rename = "update/account_all")]
    UpdateAccountAll,
    Ping,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum WsSubscriptionType {
    OrderBooks,
    Accounts,
}

// #[derive(Debug)]
// struct WsSubscription {
//     id: uuid::Uuid,
//     pub state: Option<HashMap<String, Value>>,
// }

#[derive(Debug)]
pub struct WsClient<F1, F2>
where
    F1: Fn(String, Value) + Send + Sync + 'static,
    F2: Fn(String, Value) + Send + Sync + 'static,
{
    stream: WsStream,
    subscriptions: HashMap<WsSubscriptionType, HashMap<String, Option<Value>>>,
    on_order_book_update: Option<F1>,
    on_account_update: Option<F2>,
    // {OrderBook: {'1': {id, state}}}}
}

pub struct WsClientBuilder<F1, F2>
where
    F1: Fn(String, Value) + Send + Sync + 'static,
    F2: Fn(String, Value) + Send + Sync + 'static,
{
    config: Option<LighterConfig>,
    order_books_subs: Option<(Vec<String>, F1)>,
    accounts_subs: Option<(Vec<String>, F2)>,
}

impl<F1, F2> WsClientBuilder<F1, F2>
where
    F1: Fn(String, Value) + Send + Sync + 'static,
    F2: Fn(String, Value) + Send + Sync + 'static,
{
    pub fn with_config(mut self, config: LighterConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn with_order_books_subs(
        mut self,
        order_book_subs: Vec<String>,
        order_book_update_fn: F1,
    ) -> Self {
        self.order_books_subs = Some((order_book_subs, order_book_update_fn));
        self
    }

    pub fn with_accounts_subs(mut self, account_subs: Vec<String>, account_update_fn: F2) -> Self {
        self.accounts_subs = Some((account_subs, account_update_fn));
        self
    }

    pub async fn build(self) -> Result<WsClient<F1, F2>> {
        let config = self.config.unwrap_or_default();

        if self.accounts_subs.is_none() && self.order_books_subs.is_none() {
            return Err(LighterError::Generic("No subscriptions provided".into()));
        }

        // create client
        let (ws_stream, _) = connect_async(&config.ws_url)
            .await
            .map_err(|e| LighterError::WebSocket(Box::new(e)))?;

        let mut subs = HashMap::new();

        let mut on_order_book_update = None;
        if let Some((order_books_subs, handler)) = self.order_books_subs {
            let order_books_subs = order_books_subs
                .iter()
                .map(|v| (v.to_string(), None))
                .collect::<HashMap<_, _>>();
            subs.insert(WsSubscriptionType::OrderBooks, order_books_subs);
            on_order_book_update = Some(handler);
        }

        let mut on_account_update = None;
        if let Some((account_subs, handler)) = self.accounts_subs {
            let account_subs = account_subs
                .iter()
                .map(|v| (v.to_string(), None))
                .collect::<HashMap<_, _>>();
            subs.insert(WsSubscriptionType::Accounts, account_subs);
            on_account_update = Some(handler);
        }

        Ok(WsClient {
            stream: ws_stream,
            subscriptions: subs,
            on_order_book_update,
            on_account_update,
        })
    }
}

impl<F1, F2> WsClient<F1, F2>
where
    F1: Fn(String, Value) + Send + Sync + 'static,
    F2: Fn(String, Value) + Send + Sync + 'static,
{
    pub async fn run(&mut self) -> Result<()> {
        while let Some(msg) = self.stream.next().await {
            let msg = msg.map_err(|e| {
                tracing::error!("unable to get message: {e}");
                LighterError::Generic("Unable to handle message".into())
            })?;

            self.handle_message(msg).await?;
        }

        Ok(())
    }

    // TODO: complete
    async fn handle_message(&mut self, msg: Message) -> Result<()> {
        match msg {
            Message::Text(data) => {
                let msg = serde_json::from_str::<Value>(&data).map_err(|e| {
                    tracing::error!("unable to deserialize msg: {e}");
                    LighterError::Generic("Unable to deserialize json message".into())
                })?;

                if let Some(msg_type) = msg.get("type") {
                    let api_msg = serde_json::from_value::<WsMessage>(msg_type.to_owned())
                        .map_err(|e| {
                            tracing::error!("unable to deserialize api msg: {e}");
                            LighterError::Generic("Unable to deserialize api message".into())
                        })?;

                    match api_msg {
                        WsMessage::Connected => self.handle_connected().await?,
                        WsMessage::SubscribedOrderBook => {
                            self.handle_subscribed_order_book(msg).await?
                        }
                        WsMessage::UpdateOrderBook => self.handle_update_order_book(msg).await?,
                        WsMessage::SubscribedAccountAll => {
                            self.handle_subscribed_account_all().await?
                        }
                        WsMessage::UpdateAccountAll => self.handle_update_account_all().await?,
                        WsMessage::Ping => self
                            .stream
                            .send(Message::text(json!({"type": "pong"}).to_string()))
                            .await
                            .map_err(|e| {
                                tracing::error!("unable to send `pong`");
                                LighterError::Generic("Unable to send `pong`".into())
                            })?,
                    }
                }

                Ok(())
            }
            _ => Ok(()),
        }
    }

    async fn handle_connected(&mut self) -> Result<()> {
        let order_book_subs = self.subscriptions.get(&WsSubscriptionType::OrderBooks);
        let accounts_subs = self.subscriptions.get(&WsSubscriptionType::Accounts);

        if let Some(order_books_subs) = order_book_subs {
            for market_id in order_books_subs.keys() {
                let resp =
                    json!({"type": "subscribe", "channel": format!("order_book/{market_id}")});
                self.stream
                    .send(Message::text(resp.to_string()))
                    .await
                    .map_err(|e| {
                        tracing::error!("unable to send `connected` response: {e}");
                        LighterError::Generic("Unable to send `connected` response: {e}".into())
                    })?;
            }
        }

        if let Some(accounts_subs) = accounts_subs {
            for account_id in accounts_subs.keys() {
                let resp =
                    json!({"type": "subscribe", "channel": format!("account_all/{account_id}")});
                self.stream
                    .send(Message::text(resp.to_string()))
                    .await
                    .map_err(|e| {
                        tracing::error!("unable to send `connected` response: {e}");
                        LighterError::Generic("Unable to send `connected` response: {e}".into())
                    })?;
            }
        }

        Ok(())
    }

    async fn handle_subscribed_order_book(&mut self, msg: Value) -> Result<()> {
        let channel = msg["channel"].to_string();
        let parts: Vec<&str> = channel.split(':').collect();

        let market_id = parts.get(1).ok_or_else(|| {
            tracing::error!("Unable to get market_id");
            LighterError::Generic("Unable to get market_id".into())
        })?;

        if let Some(subs) = self.subscriptions.get_mut(&WsSubscriptionType::OrderBooks) {
            // set the state
            let state_data = msg
                .get("order_book")
                .ok_or_else(|| {
                    tracing::error!("unable to get order_book from message");
                    LighterError::Generic("Unable to get `order_book` from message".into())
                })?
                .to_owned();
            subs.insert(market_id.to_string(), Some(state_data.clone()));
            if let Some(handler) = &self.on_order_book_update {
                handler(market_id.to_string(), state_data)
            }
        }
        todo!()
    }

    async fn handle_update_order_book(&mut self, msg: Value) -> Result<()> {
        todo!()
    }

    async fn handle_subscribed_account_all(&mut self) -> Result<()> {
        todo!()
    }

    async fn handle_update_account_all(&mut self) -> Result<()> {
        todo!()
    }

    async fn handle_ping(&mut self) -> Result<()> {
        todo!()
    }

    fn update_order_book_state(&mut self, market_id: String, order_book: Value) -> Result<()> {
        let existing_asks = self
            .subscriptions
            .get(&WsSubscriptionType::OrderBooks)
            .unwrap();
        let existing_asks = existing_asks[&market_id].clone().unwrap();
        let existing_asks = existing_asks.as_array().unwrap();
        self.update_orders(
            order_book["asks"].clone().as_array().unwrap(),
            existing_asks,
        );

        let existing_bids = self
            .subscriptions
            .get(&WsSubscriptionType::OrderBooks)
            .unwrap();
        let existing_bids = existing_bids[&market_id].clone().unwrap();
        let existing_bids = existing_bids.as_array().unwrap();
        self.update_orders(
            order_book["bids"].clone().as_array().unwrap(),
            existing_bids,
        );

        Ok(())
    }

    fn update_orders(
        &mut self,
        new_orders: &Vec<Value>,
        existing_orders: &Vec<Value>,
    ) -> Result<()> {
        todo!()
    }
}
