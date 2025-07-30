// Custom directory example showing how to use buf files from a subdirectory

// Include the generated proto code
mod proto {
    tonic::include_proto!("tonic_buf_build_custom_dir");
}

use proto::{
    custom_dir_service_server::{CustomDirService, CustomDirServiceServer},
    HelloRequest, HelloResponse, ConfigRequest, ConfigResponse,
};
use tonic::{transport::Server, Request, Response, Status};
use std::net::SocketAddr;

// Implementation showing custom directory usage
#[derive(Debug, Default)]
pub struct CustomDirServiceImpl;

#[tonic::async_trait]
impl CustomDirService for CustomDirServiceImpl {
    async fn say_hello_from_custom_dir(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let req = request.into_inner();
        println!("📍 Received hello request from: {} at {}", req.name, req.location);
        
        let response = HelloResponse {
            message: format!("Hello {}! 👋 This service was built from proto files in a custom directory (./proto/)", req.name),
            timestamp: chrono::Utc::now().to_rfc3339(),
        };
        
        println!("📤 Sending response: {}", response.message);
        Ok(Response::new(response))
    }
    
    async fn get_config(
        &self,
        _request: Request<ConfigRequest>,
    ) -> Result<Response<ConfigResponse>, Status> {
        println!("⚙️  Configuration request received");
        
        let response = ConfigResponse {
            buf_directory: "./proto/".to_string(),
            proto_files_count: "1".to_string(),
            available_services: vec!["CustomDirService".to_string()],
        };
        
        println!("📊 Sending config: buf_dir={}, services={:?}", 
                response.buf_directory, response.available_services);
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Custom Directory tonic-buf-build example!");
    println!("📁 This example demonstrates using proto files from ./proto/ directory");
    
    let addr: SocketAddr = "[::1]:50052".parse()?;
    let service = CustomDirServiceImpl::default();

    println!("🎯 Server listening on {}", addr);
    println!("💡 This shows how tonic-buf-build can compile proto files from custom directories");
    println!("🔧 Check build.rs to see how TonicBufConfig is used");

    Server::builder()
        .add_service(CustomDirServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}