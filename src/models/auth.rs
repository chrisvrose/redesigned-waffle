use sqlx::{PgPool, query_as};

use super::UserAuth;

// use crate::UserAuth;
pub struct UserAuthCredsDTO{
    pub email:String,
    pub pwd:String
}

impl UserAuth{
    pub async fn login(x:&UserAuthCredsDTO,dbpool:&PgPool)->bool{
        let resultrow = query_as!(UserAuthCredsDTO,"SELECT email,pwd from userauth where email=$1::text",x.email).fetch_optional(dbpool).await.unwrap_or_else(|val|{
            println!("Error: {:?}",val);
            None
        });
        match resultrow{
            Some(dbcreds)=>{
                let mut verifier = argonautica::Verifier::default();
                let is_valid = verifier.with_hash(&(dbcreds.pwd)).with_password(&(x.pwd)).verify().unwrap_or(false);
                is_valid
            },
            None=>false
        }
    }

    pub async fn authToken(usercreds:&UserAuthCredsDTO){
        
    }
    
}