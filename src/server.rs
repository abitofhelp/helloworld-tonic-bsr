mod gen;

use tonic::{transport::Server, Request, Response, Status};

use helloworld::helloworld::v1::{
    greeter_service_server::{ GreeterService, GreeterServiceServer },
    { SayHelloRequest, SayHelloResponse }
};

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl GreeterService for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<SayHelloRequest>,
    ) -> Result<Response<SayHelloResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = SayHelloResponse {
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
        .add_service(GreeterServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
