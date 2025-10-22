use argon2::{Config, Result};

use crate::misc::argon2_config;

fn get_config<'a>()->Config<'a>{
    return Config::default();
}

pub fn hash_password_with_config(pwdref:&String, salt: &String)->Result<String>{
    let cfg = argon2_config::get_config();
    argon2::hash_encoded(pwdref.as_bytes(), salt.as_bytes(), &cfg)
}
