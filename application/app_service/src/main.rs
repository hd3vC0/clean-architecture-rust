use api1::AWSApiUserService;
use commandline::EntryPoint;
use use_case::GetUserUseCase;

fn main() {
    let api: AWSApiUserService = AWSApiUserService{};
    let obj = GetUserUseCase{service:Box::new(api)}; 
    let entry = EntryPoint{get_user_inst: obj};
    commandline::entry_point(&entry);
   
}
