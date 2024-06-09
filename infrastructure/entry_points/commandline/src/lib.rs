use use_case::GetUserUseCase;

pub struct EntryPoint {
    pub get_user_inst: GetUserUseCase
}

pub fn entry_point(entrypoint: &EntryPoint) {
    println!("En el entrypoint");
    entrypoint.get_user_inst.get_user(5);
}