use log::{debug, warn};
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
    pub role: UserType
}


impl UserAuth {
    pub async fn login(
        x: &UserAuthCredsDTO,
        dbpool: &PgPool,
        jwt_key: &String,
    ) -> Result<Option<String>, sqlx::Error> {
        let resultrow = query_as!(
            UserAuthCredsUid,
            "SELECT uid,email,pwd,role from userauth where email=$1::varchar(64)",
            x.email
        )
        .fetch_optional(dbpool)
        .await?;

        if let Some(UserAuthCredsUid { pwd, uid,role, .. }) = resultrow {

            let is_valid = argon2::verify_encoded(
                pwd.as_str(),
                x.pwd.as_bytes(),
            );
            let is_valid = is_valid.unwrap_or_else(|x| {
                warn!("The user was not valid! {:?}", x);
                false
            });
            if is_valid {
                const DEFAULT_JWT_EXPIRY_DAYS: u8 = 5;
                let res = issue_jwt(jwt_key, UserDetails{ uid,user_type:role}, DEFAULT_JWT_EXPIRY_DAYS).map_or(None, |v| Some(v));
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
