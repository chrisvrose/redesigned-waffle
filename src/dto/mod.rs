use serde::{Deserialize, Serialize};
pub mod userauth;
// login credentials wrapper
#[derive(Deserialize, Serialize, Debug)]
pub struct UserAuthCredsDTO {
    pub email: String,
    pub pwd: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct NewBookDTO{
    pub course_code:String
}
