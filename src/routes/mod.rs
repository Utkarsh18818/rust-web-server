pub mod hello_user;
pub use hello_user::hello_user_config;

pub mod home;
pub use home::home_config;

#[warn(unused_imports)]
pub mod create_user;
pub use create_user::create_user_config;

pub mod todos;
pub use todos::*;

fn logging(path: &str){
    println!("{}",path);
}
