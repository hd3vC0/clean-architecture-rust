use model::entity::Username;
use model::gateways::UserServiceGateway;

pub struct GetUserUseCase{
    pub service: Box<dyn UserServiceGateway>,
}

impl GetUserUseCase {
   pub fn get_user(&self, id: u32) -> Username{
        return self.service.get_user(id);
    }
}