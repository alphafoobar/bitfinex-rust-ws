extern crate bitfinex;

use std::collections::BTreeSet;

use std::time::{SystemTime, UNIX_EPOCH};
use bitfinex::{ errors::*, events::*, websockets::* };
use bitfinex::{ pairs::*, precision::* };

struct WebSocketHandler {
    counter : u128,
    trades : BTreeSet<i64>
}

impl EventHandler for WebSocketHandler {
    fn on_connect(&mut self, event: NotificationEvent) {
        if let NotificationEvent::Info(info) = event {
            println!("Platform status: {:?}, Version {}", info.platform, info.version);
        }
    }

    fn on_auth(&mut self, _event: NotificationEvent) {}

    fn on_subscribed(&mut self, event: NotificationEvent) {
        if let NotificationEvent::TradingSubscribed(msg) = event {
            println!("Subscribed: {:?}", msg);
        } else if let NotificationEvent::CandlesSubscribed(msg) = event {
            println!("Subscribed: {:?}", msg);
        } else if let NotificationEvent::RawBookSubscribed(msg) = event {
            println!("Subscribed: {:?}", msg);
        }
    }

    fn on_data_event(&mut self, event: DataEvent) {
        let since_the_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Clock may have gone backwards")
            .as_millis();
        let now = since_the_epoch as i64;
        if let DataEvent::TradesTradingUpdateEvent(channel, pair, trading) = event {
            if !self.trades.contains(&trading.id) {
                println!("{} map.size: {} Trade update ({}) - Lag: {} (ms), Id: {}, Price: {}, Amount: {}", self.counter, self.trades.len(), channel, (now - trading.mts), trading.id, trading.price, trading.amount);
                self.counter = self.counter + 1;
                self.trades.insert(trading.id);
            }/* else {
                println!("{} Trades already contains update ({}) - Lag: {} (ms), Id: {}, Price: {}, Amount: {}", self.counter, channel, (now - trading.mts), trading.id, trading.price, trading.amount);
            }*/
        }
    }

    fn on_error(&mut self, message: Error) {
        println!("{:?}", message);
    }
}

fn main() {
    let mut web_socket: WebSockets = WebSockets::new();

    web_socket.add_event_handler(WebSocketHandler { counter: 0 , trades: BTreeSet::new()});
    web_socket.connect().unwrap(); // check error

    // TRADES
    web_socket.subscribe_trades(BTCUSD, EventType::Trading);
    web_socket.event_loop().unwrap(); // check error
}
