use core::application::use_cases::CreateUser;
use core::domain::entities::User;
use core::infrastructure::repositories::UserDynamoRepository;

fn main() {
    let repository: UserDynamoRepository = UserDynamoRepository::configure();
    let use_case = CreateUser::new(&repository);
    let result: Result<(), String> = use_case.execute(&User::new(
        "123456".to_string(), 
        "Humberto A".to_string(), 
        "Monterrosa".to_string(),
        "h@h.com".to_string()));
    
    match result {
        Ok(()) => print!("Todo OK"),
        Err(msg) => print!("Error: {}", msg.to_string())
    }

    let resultado = mifn();

}
