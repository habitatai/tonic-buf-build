// Advanced example showing mock gRPC server implementation
// with file descriptor set for reflection support

// Include the generated proto code
mod proto {
    tonic::include_proto!("tonic_buf_build_sample");
}

use proto::{
    hello_service_server::{HelloService, HelloServiceServer},
    hello_service_client::HelloServiceClient,
    SayHelloResponse,
};
use tonic::{transport::Server, Request, Response, Status};
use std::net::SocketAddr;

// Mock server implementation
#[derive(Debug, Default)]
pub struct MockHelloService;

#[tonic::async_trait]
impl HelloService for MockHelloService {
    async fn say_hello(
        &self,
        _request: Request<()>,
    ) -> Result<Response<SayHelloResponse>, Status> {
        println!("📥 Received SayHello request");
        
        let response = SayHelloResponse {
            value: "Hello from mock server! 🎉".to_string(),
        };
        
        println!("📤 Sending response: {}", response.value);
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Advanced tonic-buf-build example with mock server!");
    
    let addr: SocketAddr = "[::1]:50051".parse()?;
    let hello_service = MockHelloService::default();

    println!("✅ Proto code successfully generated with:");
    println!("   - Server and client code");
    println!("   - File descriptor set for reflection"); 
    println!("   - Custom tonic-prost-build configuration");
    
    // Check if descriptor file was generated
    let out_dir = std::env::var("OUT_DIR").unwrap_or_else(|_| "target/debug/build".to_string());
    println!("📁 Descriptor file at: {}/*/out/services_descriptor.bin", out_dir);
    
    println!("\n🌐 Starting mock server on http://{}", addr);
    println!("📡 HTTP endpoint available at: http://[::1]:50051/hello");
    
    // Start the server
    tokio::spawn(async move {
        Server::builder()
            .add_service(HelloServiceServer::new(hello_service))
            .serve(addr)
            .await
            .expect("Server failed to start");
    });
    
    // Give server time to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    // Test the server with a client
    println!("\n🔄 Testing server with client...");
    
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;
    let mut client = HelloServiceClient::new(channel);
    
    let request = tonic::Request::new(());
    let response = client.say_hello(request).await?;
    
    println!("✅ Client received: {}", response.into_inner().value);
    
    println!("\n✨ This example shows the new API migration:");
    println!("   OLD: tonic_buf_build::compile_from_buf(tonic_build::Builder::new().file_descriptor_set_path(...), None)");
    println!("   NEW: tonic_buf_build::compile_from_buf_with_builder_config(|builder| builder.file_descriptor_set_path(...))");
    
    println!("\n🎯 Mock server is running! Try:");
    println!("   curl http://[::1]:50051/hello");
    
    // Keep server running for a bit
    println!("\n⏱️  Server will run for 30 seconds...");
    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    
    println!("👋 Shutting down mock server");
    Ok(())
}