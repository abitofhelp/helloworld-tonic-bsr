mod gen;

pub mod helloworld {
    pub mod v1 {
        include!("gen/helloworld.v1.rs");
    }
}

use tonic::{transport::Server, Request, Response, Status};

use helloworld::v1::greeter_service_server::{GreeterService, GreeterServiceServer};
use helloworld::v1::{SayHelloRequest, SayHelloResponse};

use crate::helloworld::v1::greeter_service_server;

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl GreeterService for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<SayHelloRequest>,
    ) -> Result<Response<SayHelloResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = helloworld::v1::SayHelloResponse {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    println!("\n>> Starting the GreeterService server: {}", addr);
    Server::builder()
        .add_service(greeter_service_server::GreeterServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
