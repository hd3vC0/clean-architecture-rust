#[derive(Clone, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub last_name: String,
    pub email: String
}

impl User {
    pub fn new(id: String, name: String, last_name: String, email: String) -> Self {
        User { id, name, last_name, email }
    }
}