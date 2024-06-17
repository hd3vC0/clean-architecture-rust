use crate::domain::{entities::User, gateways::UserRepository};
pub struct CreateUser<'a> {
    pub repository: &'a dyn UserRepository
}

impl <'a> CreateUser<'a> {
    pub fn new(repository: &'a dyn UserRepository) -> Self {
        CreateUser { repository }
    }

    pub fn execute(&self, user: User) -> Result<(), String> {
        self.repository.save(user)
    }
}