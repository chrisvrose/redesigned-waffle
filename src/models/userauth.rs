use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, Error as SqlxError, PgPool};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUserDTO {
    pub name: String,
    pub email: String,
    pub pwd: String,
    pub semester: i32,
    pub deptid: String,
}

/// Outfacing user -> Missing pwd
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutUserDTO {
    pub uid: i32,
    pub name: String,
    pub email: String,
    pub semester: i32,
    pub deptid: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserAuth {
    pub uid: i32,
    pub name: String,
    pub email: String,
    pub pwd: String,
    pub semester: i32,
    pub deptid: String,
}

impl UserAuth {
    /// convert to a simpler output model
    pub fn out(self) -> OutUserDTO {
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
        let resp = query_as!(OutUserDTO, "select uid,email,semester,name,deptid from userauth")
            .fetch_all(db)
            .await;
        resp
    }
    /// add a user
    pub async fn add_user(user: &NewUserDTO, db: &PgPool, salt: &String) -> Result<i32, SqlxError> {
        let mut hasher = argonautica::Hasher::default();
        let pwdref = &(user.pwd);
        let mut tx = db.begin().await?;
        // let pwdhash = hasher.with
        let response = query!(
            "INSERT INTO userauth(name,email,pwd,semester,deptid) values($1,$2,$3,$4,$5) returning uid",
            user.name,
            user.email,
            "foobar",
            user.semester,
            user.deptid
        )
        .fetch_one(&mut tx)
        .await?;

        // only after inserting the user, actually generate a password. Otherwise, not worth the effort of hashing

        let hashed_pwd = hasher
            .with_password(pwdref)
            .with_secret_key(salt)
            .hash()
            .unwrap();
        //get this data type
        let inserteduid = response.uid;
        /* let update_pwd_status = */ query!(
            "update userauth set pwd=$1 where uid=$2",
            hashed_pwd,
            inserteduid
        )
        .execute(&mut tx)
        .await?;

        tx.commit().await?;

        Ok(inserteduid)
    }
}
