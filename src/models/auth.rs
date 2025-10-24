use log::{debug, error, warn};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, Error, PgPool};

use super::UserAuth;
use crate::{dto::UserAuthCredsDTO, errors::response::ResponseErrors, misc::{argon2_config::verify, auth::{issue_jwt, UserDetails, UserType}}};

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
    ) -> Result<String, ResponseErrors> {
        let resultrow = query_as!(
            UserAuthCredsUid,
            "SELECT uid,email,pwd,role from userauth where email=$1::varchar(64)",
            x.email
        )
        .fetch_optional(dbpool)
        .await?;

        if let Some(UserAuthCredsUid { pwd, uid,role, .. }) = resultrow {
            let is_valid = verify(&pwd, &x.pwd)?;

            if is_valid {
                const DEFAULT_JWT_EXPIRY_DAYS: u8 = 5;
                let res = issue_jwt(jwt_key, UserDetails{ uid,user_type:role}, DEFAULT_JWT_EXPIRY_DAYS)?;
                Ok(res)
            } else {
                warn!("Could not find valid ");
                Err(ResponseErrors::Unauthorized)
            }
        } else {
            warn!("Could not find user {}", x.email);
            Err(ResponseErrors::Unauthorized)
        }
    }
}
