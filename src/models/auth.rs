use sqlx::{PgPool, query_as};

use super::UserAuth;

// use crate::UserAuth;
pub struct UserAuthCredsDTO{
    pub email:String,
    pub pwd:String
}

impl UserAuth{
    pub async fn login(x:&UserAuthCredsDTO,dbpool:&PgPool)->Result<bool,sqlx::Error>{
        let resultrow = query_as!(UserAuthCredsDTO,"SELECT email,pwd from userauth where email=$1::text",x.email).fetch_optional(dbpool).await?;
        let res = if let Some(dbcredential)=resultrow {
            let mut verifier = argonautica::Verifier::default();
            let isValid = verifier.with_hash(&(dbcredential.pwd)).with_password(&(x.pwd)).verify().unwrap_or(false);
            isValid
        }else{
            false
        };
        Ok(res)
    }
}