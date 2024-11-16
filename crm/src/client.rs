use crm::pb::{user_service_client::UserServiceClient, GetUserRequest};
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = UserServiceClient::connect("http://[::1]:50051").await?;

    let request = Request::new(GetUserRequest { id: 1 });
    let response = client.get_user(request).await?;

    println!("Response: {:?}", response);

    Ok(())
}
