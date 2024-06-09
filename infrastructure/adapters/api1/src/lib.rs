use model::{entity::Username, gateways::UserServiceGateway};

pub struct AWSApiUserService{}

impl UserServiceGateway for  AWSApiUserService {
    fn get_user(&self, id: u32) -> model::entity::Username {
        print!("Consulta usuario id: {}", id);
        Username {
            name: String::from("Juan"),
            last_name: String::from("Perez"),
            email: String::from("jp@h.com"),
        }
    }
}