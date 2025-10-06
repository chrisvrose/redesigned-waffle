use log::debug;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, Error, PgPool};

use super::UserAuth;
use crate::{dto::UserAuthCredsDTO, misc::auth::{issue_jwt, UserDetails, UserType}};

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
        jwt_key: &String,
    ) -> Result<Option<String>, sqlx::Error> {
        let resultrow = query_as!(
            UserAuthCredsUid,
            "SELECT uid,email,pwd from userauth where email=$1::varchar(64)",
            x.email
        )
        .fetch_optional(dbpool)
        .await?;

        if let Some(UserAuthCredsUid { pwd, uid, .. }) = resultrow {

            let is_valid = argon2::verify_encoded(
                pwd.as_str(),
                x.pwd.as_bytes(),
            );
            let is_valid = is_valid.unwrap_or_else(|x| {
                debug!("The user was not valid! {:?}", x);
                false
            });
            if is_valid {
                let res = issue_jwt(jwt_key, UserDetails{uid,user_type:UserType::Student}, 5).map_or(None, |v| Some(v));
                Ok(res)
            } else {
                // Ok(None)
                Err(Error::RowNotFound)
            }
        } else {
            debug!("Could not find user {}", x.email);
            Err(Error::RowNotFound)
        }
    }
}
