use log::debug;
use serde::{Deserialize, Serialize};
use sqlx::{Error as SqlxError, PgPool, query, query_as};

use crate::dto::userauth::OutUserDTO;
use crate::errors::response::ResponseErrors;
use crate::misc::argon2_config::hash_password_with_config;
use crate::misc::auth::UserType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUserDTO {
    pub name: String,
    pub email: String,
    pub pwd: String,
    pub semester: i32,
    pub deptid: String,
    pub role: Option<UserType>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserAuth {
    pub uid: i32,
    pub name: String,
    pub email: String,
    pub pwd: String,
    pub semester: Option<i32>,
    pub deptid: String,
    pub role: UserType,
}

impl UserAuth {
    /// get all bodies
    pub async fn get_all(db: &PgPool) -> Result<Vec<OutUserDTO>, SqlxError> {
        let resp = query_as!(
            OutUserDTO,
            "select uid,email,semester,name,deptid,role from userauth"
        )
        .fetch_all(db)
        .await;
        resp
    }
    /// add a user
    pub async fn add_user(
        user: &NewUserDTO,
        db: &PgPool,
        salt: &String,
    ) -> Result<i32, ResponseErrors> {
        debug!("Attempting to add user for {}", user.email);

        let mut tx = db.begin().await?;
        let pwdref = &(user.pwd);

        let response = query!(
                "INSERT INTO userauth(name,email,pwd,semester,deptid,role) values($1,$2,$3,$4,$5,$6) returning uid",
                user.name,
                user.email,
                // put blank password first, refer below
                "",
                user.semester,
                user.deptid,
                i32::from(user.role.unwrap_or_default())
            )
            .fetch_one(&mut *tx)
            .await?
            ;

        // only after inserting the user, actually generate a password. Otherwise, not worth the effort of hashing
        let hashed_pwd = hash_password_with_config(pwdref, salt)?;

        // get this data type
        let inserteduid = response.uid;

        query!(
            "update userauth set pwd=$1 where uid=$2",
            hashed_pwd,
            inserteduid
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(inserteduid)
    }
}
