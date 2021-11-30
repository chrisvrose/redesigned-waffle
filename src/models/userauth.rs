use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, Error as SqlxError, PgPool};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserAuth {
    pub uid: i32,
    pub name: String,
    pub pwd: String,
    pub semester: i32,
    pub dept: String,
}

impl UserAuth {
    pub async fn get_all(db: &PgPool) -> Result<Vec<UserAuth>, SqlxError> {
        let resp = query_as!(UserAuth, "select * from userauth")
            .fetch_all(db)
            .await;
        resp
    }

    pub async fn add_user(user: &UserAuth, db: &PgPool) -> Result<i32, SqlxError> {
        // TODO user.pwd should be hashed
        
        let mut hasher = argonautica::Hasher::default();
        // let pwdhash = hasher.with
        let response = query!(
            "INSERT INTO userauth values($1,$2,$3,$4,$5) returning uid",
            user.uid,
            user.name,
            user.pwd,
            user.semester,
            user.dept
        )
        .fetch_one(db)
        .await?;

        //get this data type
        let inserteduid = response.uid;
        
        Ok(inserteduid)
        
    }
}
