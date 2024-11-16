use tonic::{async_trait, transport::Server, Request, Response};

use crm::pb::{
    user_service_server::{UserService, UserServiceServer},
    CreateUserRequest, GetUserRequest, User,
};

#[derive(Default)]
pub struct UserServer {}

#[async_trait]
impl UserService for UserServer {
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<User>, tonic::Status> {
        let input = request.into_inner();
        println!("Got a request: {:?}", input);
        Ok(Response::new(User::new(1, "fafa", "fafa@qq.com")))
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<User>, tonic::Status> {
        let input = request.into_inner();
        println!("Got a request: {:?}", input);
        Ok(Response::new(User::new(1, &input.name, &input.email)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let user_server = UserServer::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(UserServiceServer::new(user_server))
        .serve(addr)
        .await?;

    Ok(())
}
