use use_case::GetUserUseCase;

pub struct EntryPoint {
    pub get_user_inst: GetUserUseCase
}

pub fn entry_point(entrypoint: &EntryPoint) {
    println!("En el entrypoint");
    let get_user = entrypoint.get_user_inst.get_user(5);
    println!("Usuario {}", get_user.name)
}