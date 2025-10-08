use log::debug;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, Error as SqlxError, PgPool};

use crate::misc::argon2_config;
use crate::dto::userauth::OutUserDTO;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUserDTO {
    pub name: String,
    pub email: String,
    pub pwd: String,
    pub semester: i32,
    pub deptid: String,
}



#[derive(Serialize, Deserialize, Debug)]
pub struct UserAuth {
    pub uid: i32,
    pub name: String,
    pub email: String,
    pub pwd: String,
    pub semester: Option<i32>,
    pub deptid: String,
}

impl UserAuth {
    /// convert to a simpler output model
    pub fn _out(self) -> OutUserDTO {
        OutUserDTO {
            uid: self.uid,
            email: self.email,
            semester: self.semester,
            name: self.name,
            deptid: self.deptid,
        }
    }
    /// get all bodies
    pub async fn get_all(db: &PgPool) -> Result<Vec<OutUserDTO>, SqlxError> {
        let resp = query_as!(
            OutUserDTO,
            "select uid,email,semester,name,deptid from userauth"
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
    ) -> Result<i32, SqlxError> {
        debug!("Attempting to add user for {}", user.email);


        let mut tx = db.begin().await?;
        let pwdref = &(user.pwd);

        let response = query!(
                "INSERT INTO userauth(name,email,pwd,semester,deptid) values($1,$2,$3,$4,$5) returning uid",
                user.name,
                user.email,
                // put blank password first, refer below
                "",
                user.semester,
                user.deptid
            )
            .fetch_one(&mut *tx)
            .await?;

        // only after inserting the user, actually generate a password. Otherwise, not worth the effort of hashing

        let cfg = argon2_config::get_config();
        let hashed_pwd = argon2::hash_encoded(pwdref.as_bytes(), salt.as_bytes(), &cfg).unwrap();

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

    // pub async fn remove_user(uid:&i32,db:&PgPool)->Result<u64,SqlxError> {
    //     let x = query!("delete from userauth where uid=$1",uid).execute(db).await?;

    //     Ok(x.rows_affected())
    // }
}
