use payments::click2_pay_server::{Click2Pay, Click2PayServer};
use payments::{Click2PayPaymentRequest, Click2PayPaymentResponse};

use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};

mod payments {
    tonic::include_proto!("payments");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("payment_descriptor");
}

#[derive(Debug, Default)]
pub struct Click2PayService {}

#[tonic::async_trait]
impl Click2Pay for Click2PayService {
    async fn send_payment(
        &self,
        request: Request<Click2PayPaymentRequest>,
    ) -> Result<Response<Click2PayPaymentResponse>, Status> {
        let req = request.into_inner();

        let reply = Click2PayPaymentResponse {
            successful: true,
            message: format!("Enviado R$ {}  para {}.", req.amount, req.to_addr).into(),
        };

        Ok(Response::new(reply))
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "[::1]:50051".parse()?;

    let click2pay_service: Click2PayService = Click2PayService::default();
    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(payments::FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .add_service(service)
        .add_service(Click2PayServer::new(click2pay_service))
        .serve(addr)
        .await?;

    Ok(())
}
