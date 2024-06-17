use core::{application::use_cases::CreateUser, domain::entities::User, infrastructure::{repositories::UserDynamoRepository, utils::aws_config}};
use uuid::Uuid;

use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};

async fn function_handler(_: Request) -> Result<Response<Body>, Error> {
    let config = aws_config().await;
    let repository = UserDynamoRepository::new(config);
    let create_user = CreateUser::new(&repository);
    let uid = Uuid::new_v4().to_string();

    let user = User::new(
        uid,
        "Foo".to_string(),
        "Bar".to_string(),
        "f@b.com".to_string()
    );
    
    // execute
    let _ = create_user.execute(user);

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(Body::Text("Ok".to_string()))
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
