use serde::{Deserialize, Serialize};
use sqlx::{query_as, Error, PgPool};

use crate::misc::auth::{issue_jwt,UserType};
use super::UserAuth;

// login credentials wrapper
#[derive(Deserialize, Serialize, Debug)]
pub struct UserAuthCredsDTO {
    pub email: String,
    pub pwd: String,
}

/// result from db -> dont include pwd
#[derive(Deserialize, Serialize, Debug)]

struct UserAuthCredsUid {
    pub uid: i32,
    pub email: String,
    pub pwd: String,
}

impl UserAuth {
    pub async fn login_student(
        x: &UserAuthCredsDTO,
        dbpool: &PgPool,
        pepper: &String,
        jwt_key: &String,
    ) -> Result<Option<String>, sqlx::Error> {
        let resultrow = query_as!(
            UserAuthCredsUid,
            "SELECT uid,email,pwd from userauth where email=$1::varchar(64)",
            x.email
        )
        .fetch_optional(dbpool)
        .await?;

        if let Some(UserAuthCredsUid{pwd,uid,..}) = resultrow {
            let mut verifier = argonautica::Verifier::default();
            let is_valid = verifier
                .with_hash(&(pwd))
                .with_password(&(x.pwd))
                .with_secret_key(pepper.as_bytes())
                .verify();
            let is_valid = is_valid.unwrap_or(false);
            if is_valid {
                let res = issue_jwt(jwt_key, UserType::Student(uid), 5).map_or(None, |v| Some(v));
                Ok(res)
            } else {
                // Ok(None)
                Err(Error::RowNotFound)
            }
        } else {
            Err(Error::RowNotFound)
        }
    }
}
