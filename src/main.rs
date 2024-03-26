use crate::http::httplistener::HttpListener;
use crate::http::listenerhandler::ListenerHandler;

mod http;
mod listeners;

#[tokio::main]
async fn main() {
    let mut listener_handler = ListenerHandler::new();

    listener_handler.add_listener(Box::new(listeners::basiclistener::BasicListener {}));

    let listener = HttpListener {
        listener_handler
    };
    
    
    listener.start().await.unwrap();
}
