use crate::entity::Username;

pub trait UserServiceGateway {
    fn get_user(&self, id: u32) -> Username;
}