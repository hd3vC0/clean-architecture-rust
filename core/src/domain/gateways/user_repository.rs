use crate::domain::entities::User;
pub trait UserRepository {
    fn save(&self, user: User) -> Result<(), String>;
}