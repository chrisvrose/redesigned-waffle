use jsonwebtoken::{DecodingKey, Validation, decode, encode, errors::Error as JWTError};
use serde::{Deserialize, Serialize};
use std::time::UNIX_EPOCH;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]

pub struct UserDetails {
    pub uid: i32,
    pub user_type: UserType,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum UserType {
    Admin,
    Student,
}

/// parts of the authtoken we are concerned about
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub struct AuthTokenData {
    pub user_details: UserDetails,
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
pub fn issue_jwt(key: &String, user_details: UserDetails, expdays: u8) -> Result<String, JWTError> {
    let key = key.as_bytes();

    let exp_time = add_days_to_epoch(expdays);
    let data = AuthTokenData {
        user_details,
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
        const UID_PROVIDED: i32 = 23;
        let jwt = issue_jwt(
            &secret,
            UserDetails {
                uid: UID_PROVIDED,
                user_type: UserType::Student,
            },
            5,
        )?;

        let user_details_given = validate_jwt(&secret, &jwt)?;

        assert_eq!(user_details_given.user_details.user_type, UserType::Student,);

        Ok(())
    }
    #[test]
    pub fn token_serialization_symmetric() -> Result<(), serde_json::Error> {
        let token_data = AuthTokenData {
            user_details: UserDetails {
                uid: 32,
                user_type: UserType::Student,
            },
            exp: 1000,
        };

        let token_string = serde_json::to_string(&token_data)?;

        let token_reencoded = serde_json::from_str::<AuthTokenData>(token_string.as_str())?;

        assert_eq!(token_data, token_reencoded);
        Ok(())
    }
}
