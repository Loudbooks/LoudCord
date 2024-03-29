use tiny_http::Request;

use crate::discord::objects::interaction::incominginteraction::IncomingInteraction;
use crate::http::listenerhandler::ListenerHandler;

pub struct HttpListener {
    pub(crate) listener_handler: ListenerHandler,
}

impl HttpListener {
    pub async fn start(&self) -> std::io::Result<()> {
        let server = {
            let this = tiny_http::Server::http("127.0.0.1:3000");
            match this {
                Ok(t) => t,
                Err(e) => panic!("{:?}", e),
            }
        };

        println!("Listening on port 3000!");

        for request in server.incoming_requests() {
            self.handle_connection(request).await?;
        }

        Ok(())
    }

    async fn handle_connection(&self, mut request: Request) -> std::io::Result<()> {
        let mut input = String::new();
        request.as_reader().read_to_string(&mut input)?;
        
        println!("Received: {}", input);
        println!("{}", request.method());
        
        let message = serde_json::from_str::<IncomingInteraction>(input.as_str()).unwrap_or_else(|e| {
            panic!("{:?}", e);
        });

        self.listener_handler.handle_message(&message).await;
        
        Ok(())
    }
}
