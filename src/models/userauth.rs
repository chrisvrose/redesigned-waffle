use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, Error as SqlxError, PgPool, Executor};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct NewUserDTO{
    pub name:String,
    pub email:String,
    pub pwd:String,
    pub semester:i32,
    pub dept:String
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct OutUserDTO{
    pub uid:i32,
    pub name:String,
    pub email:String,
    pub semester:i32,
    pub dept:String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserAuth {
    pub uid: i32,
    pub name: String,
    pub email:String,
    pub pwd: String,
    pub semester: i32,
    pub dept: String,
}

impl UserAuth {
    /// convert to a simpler output model
    pub fn out(self)->OutUserDTO{
        OutUserDTO{
            uid:self.uid,
            email:self.email,
            semester:self.semester,
            name:self.name,
            dept:self.dept
        }
    }
    /// get all bodies
    pub async fn get_all(db: &PgPool) -> Result<Vec<UserAuth>, SqlxError> {
        // let resp = query_as!(OutUserDTO, "select uid,email,semester,name,dept from userauth")
        let resp = query_as!(UserAuth, "select * from userauth")
            .fetch_all(db)
            .await;
        resp
    }

    pub async fn add_user(user: &NewUserDTO, db: &PgPool,salt:&String) -> Result<i32, SqlxError> {
        
        let mut hasher = argonautica::Hasher::default();
        let pwdref = &(user.pwd);
        let mut tx = db.begin().await?;
        // let pwdhash = hasher.with
        let response = query!(
            "INSERT INTO userauth(name,email,pwd,semester,dept) values($1,$2,$3,$4,$5) returning uid",
            user.name,
            user.email,
            "foobar",
            user.semester,
            user.dept
        )
        .fetch_one(&mut tx)
        .await?;
        
        // only after inserting the user, actually generate a password. Otherwise, not worth the effort of hashing

        let hashed_pwd = hasher.with_password(pwdref).with_secret_key(salt).hash().unwrap();
        //get this data type
        let inserteduid = response.uid;
        let updatePwdStatus = query!("update userauth set pwd=$1 where uid=$2",hashed_pwd,inserteduid).execute(&mut tx).await?;

        
        
        let stat= tx.commit().await?;
        
        Ok(inserteduid)
        
    }
}
