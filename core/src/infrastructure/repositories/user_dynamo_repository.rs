use aws_config::SdkConfig;
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use tokio::runtime::Handle;
use crate::domain::{entities::User, gateways::UserRepository};
pub struct UserDynamoRepository {
    client: Client,
}

impl UserDynamoRepository {
    pub fn new(config: SdkConfig) -> Self {
        let client = Client::new(&config);
        UserDynamoRepository { client }
    }
}

impl  UserRepository for UserDynamoRepository {
    fn save(&self, user: User) -> Result<(), String> {
        let id = AttributeValue::S(user.id);
        let name = AttributeValue::S(user.name);
        let last_name = AttributeValue::S(user.last_name);
        let email = AttributeValue::S(user.email);
        let request = self
            .client
            .put_item()
            .table_name("user")
            .item("id", id)
            .item("name", name)
            .item("last_name", last_name)
            .item("email", email);
        
        let handle = Handle::current();
        
        let _ = futures::executor::block_on(async {
            handle.spawn(async {
                let resp = request.send().await;
                match resp {
                    Err(msg) => panic!("{}", msg),
                    Ok(_) => println!("Todo OK")
                }
            }).await
        });
        
        Ok(())
    }
}