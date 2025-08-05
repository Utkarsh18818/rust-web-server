pub mod hello_user;
pub use hello_user::hello_user_config;

pub mod home;
pub use home::home_config;


pub mod create_user;
pub use create_user::create_user_config;

fn logging(path: &str){
    println!("{}",path);
}
