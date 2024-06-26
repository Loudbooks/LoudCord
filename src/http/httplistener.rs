use tiny_http::Request;
use crate::discord::mapping::interactiontype::InteractionType;

use crate::discord::objects::interaction::incominginteraction::IncomingInteraction;
use crate::http::authorization;
use crate::http::listenerhandler::ListenerHandler;

pub struct HttpListener {
    pub(crate) listener_handler: ListenerHandler,
    pub(crate) public_key: String,
}

impl HttpListener {
    pub async fn start(&self, port: i16) -> std::io::Result<()> {
        let server = {
            let this = tiny_http::Server::http(format!("127.0.0.1:{}", port));
            match this {
                Ok(t) => t,
                Err(e) => panic!("{:?}", e),
            }
        };

        println!("Listening on port {}!", port);

        for request in server.incoming_requests() {
            self.handle_connection(request).await?;
        }

        Ok(())
    }

    async fn handle_connection(&self, mut request: Request) -> std::io::Result<()> {
        let mut input = String::new();
        request.as_reader().read_to_string(&mut input)?;
        
        let is_valid = authorization::verify_message(self.public_key.as_str(), request.headers(), input.as_str());
        
        println!("{}", input);
        
        let message = serde_json::from_str::<IncomingInteraction>(input.as_str());
        
        if message.is_err() {
            request.respond(tiny_http::Response::new(
                tiny_http::StatusCode(400),
                vec![],
                "Bad Request".as_bytes(),
                None,
                None
            )).unwrap();
            
            println!("Bad request!");
            
            return Ok(());
        }
        
        let message = message.unwrap();
        
        if is_valid.is_err() {
            request.respond(tiny_http::Response::new(
                tiny_http::StatusCode(401),
                vec![],
                "Unauthorized".as_bytes(),
                None,
                None
            )).unwrap();
            
            println!("Invalid message!");
            
            return Ok(());
        } else {
            println!("Valid message!")
        }
        
        if message.r#type.clone().unwrap() == InteractionType::Ping {
            request.respond(tiny_http::Response::from_string("{\"type\": 1}")).unwrap();
            println!("Pong!");
            return Ok(());
        }

        self.listener_handler.handle_message(&message).await;
        
        Ok(())
    }
}
