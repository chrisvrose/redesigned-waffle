use argon2::{Config, Result};

use crate::misc::argon2_config;

fn get_config<'a>() -> Config<'a> {
    return Config::default();
}

pub fn hash_password_with_config(pwdref: &String, salt: &str) -> Result<String> {
    let cfg = argon2_config::get_config();
    argon2::hash_encoded(pwdref.as_bytes(), salt.as_bytes(), &cfg)
}
pub fn verify(hash: &str, password: &str) -> argon2::Result<bool> {
    argon2::verify_encoded(hash, password.as_bytes())
}

#[cfg(test)]
mod tests {
    use crate::misc::argon2_config::{hash_password_with_config, verify};
    const SALT_STR: &str = "Hello world";
    #[test]
    pub fn test_hash() {
        let pwdref = String::from("hello");
        let hashed = hash_password_with_config(&pwdref, SALT_STR);
        assert!(hashed.is_ok())
    }
    #[test]
    pub fn test_hash_verify(){
        let pwdref = String::from("hello");
        let hashed_res = hash_password_with_config(&pwdref, SALT_STR);
        assert!(hashed_res.is_ok());
        let hashed_pwd = hashed_res.unwrap();
        let verify_result = verify(&hashed_pwd, &pwdref);

        assert!(verify_result.is_ok());
        assert!(verify_result.unwrap());
    }
}
