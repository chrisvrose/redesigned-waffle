use jsonwebtoken::{decode, encode, errors::Error as JWTError, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::time::UNIX_EPOCH;

#[derive(Debug, Serialize, Deserialize,PartialEq, Eq,Clone,Copy)]

pub enum UserType {
    Admin(i32),
    Student(i32),
}

/// parts of the authtoken we are concerned about
#[derive(Debug, Serialize, Deserialize,PartialEq, Eq,Clone, Copy)]
pub struct AuthTokenData {
    pub uid: UserType,
    pub exp: usize,
}

/// add days to current timestamp
fn add_days_to_epoch(expdays: u8) -> u64 {
    let origin = UNIX_EPOCH;
    const SECS_IN_DAYS: u64 = 24 * 60 * 60;
    let duration = origin.elapsed().expect("We're still living in the past");

    duration.as_secs() + (expdays as u64) * SECS_IN_DAYS
}

/// issue a jwt, expiring in the future
pub fn issue_jwt(key: &String, uid: UserType, expdays: u8) -> Result<String, JWTError> {
    let key = key.as_bytes();

    let exp_time = add_days_to_epoch(expdays);
    let data = AuthTokenData {
        uid,
        exp: exp_time as usize,
    };

    let claims = encode(
        &jsonwebtoken::Header::default(),
        &data,
        &jsonwebtoken::EncodingKey::from_secret(key),
    );

    claims
}
/// validate a jwt and return the uid
pub fn validate_jwt(key: &String, token: &String) -> Result<AuthTokenData, JWTError> {
    // let key = key.into();
    // let key:&String = & (*key).into();
    let key = key.as_bytes();

    // let token:&String = &(*token).into();

    let res = decode::<AuthTokenData>(
        token,
        &DecodingKey::from_secret(key),
        &Validation::default(),
    )?;

    Ok(res.claims)
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::errors::Error as JWTError;
    #[test]
    pub fn test_jwt() -> Result<(), JWTError> {
        let secret = String::from("mySecret");
        let jwt = issue_jwt(&secret, UserType::Student(23), 5)?;

        let uid = validate_jwt(&secret, &jwt)?;

        assert_eq!(
            uid.uid, UserType::Student(23),
            "Testing that we get back same student uid, had expected {} but got {:?}",
            23, uid.uid
        );

        Ok(())
    }
    #[test]
    pub fn how_does_enum_ser_look_like()->Result<(),serde_json::Error> {
        let xold= AuthTokenData {
            uid: UserType::Student(32),
            exp: 1000,
        };
        let x = serde_json::to_string(&xold)?;

        let x= serde_json::from_str::<AuthTokenData>(x.as_str())?;

        assert_eq!(xold,x);
        Ok(())
    }
}
