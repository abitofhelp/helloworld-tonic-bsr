mod gen;

pub mod helloworld {
    pub mod v1 {
        include!("gen/helloworld.v1.rs");
    }
}

use helloworld::v1::greeter_service_client::GreeterServiceClient;
use helloworld::v1::SayHelloRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(SayHelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
