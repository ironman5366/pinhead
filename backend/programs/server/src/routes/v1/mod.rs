pub mod document;
pub mod health_check;
pub mod user;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CreateUserSerializer {
    pub email: String,
    pub password: String,
}
