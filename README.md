# helloworld-tonic

A Rust-based protobuf/gRPC example using buf.Build BSR.
This example is based upon the helloworld-tonic sample at https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md.

## Example grpcurl commands:

### SayHello:

grpcurl -plaintext -import-path "./proto/helloworld/v1" -proto helloworld.proto -d '{"name": "Tonic"}' '[::1]:50051'
helloworld.v1.GreeterService/SayHello

{ "message": "Hello Tonic!" }

### Describe GreeterService:

grpcurl --plaintext --import-path "./proto/helloworld/v1" -proto helloworld.proto describe helloworld.v1.GreeterService

helloworld.v1.GreeterService is a service:

service GreeterService {

rpc SayHello ( .helloworld.v1.SayHelloRequest ) returns ( .helloworld.v1.SayHelloResponse );

}

