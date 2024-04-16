use payments::click2_pay_client::Click2PayClient;
use payments::Click2PayPaymentRequest;

mod payments {
    tonic::include_proto!("payments");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("payment_descriptor");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Click2PayClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(Click2PayPaymentRequest {
        amount: 35,
        from_addr: "12345".to_owned(),
        to_addr: "54321".to_owned(),
    });

    let response = client.send_payment(request).await?;

    println!("Resposta={:?}", response);

    Ok(())
}
