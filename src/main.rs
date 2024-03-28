use crate::http::httplistener::HttpListener;
use crate::http::listenerhandler::ListenerHandler;

mod discord;
mod http;
mod listeners;
mod utils;

#[tokio::main]
async fn main() {
    println!("{}", 1 << 6);

    let mut listener_handler = ListenerHandler::new();

    listener_handler.add_listener(Box::new(listeners::basiclistener::BasicListener {}));

    let listener = HttpListener { listener_handler };

    listener.start().await.unwrap();
}
