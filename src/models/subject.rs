use serde::{Deserialize, Serialize};
use sqlx::{query,query_as, Pool, Postgres};
#[derive(Serialize, Deserialize, Debug)]
pub struct Subject {
    pub coursecode: String,
    pub name: String,
    pub semester:i32,
    pub dept:String,
    pub maxcapacity:i32
}

impl Subject {
    pub async fn get_all(db: &Pool<Postgres>) -> Result<Vec<Subject>, sqlx::error::Error> {
        let resp = query_as!(Subject,"select * from subject")
            .fetch_all(db)
            .await;
        resp
    }

    pub async fn get_one(id:&String,db:&Pool<Postgres>)->Result<Option<Subject>,sqlx::error::Error>{
        let resp = query_as!(Subject,"Select * from subject where coursecode=$1",id).fetch_optional(db).await?;
        Ok(resp)
    }

    pub async fn insert(data: &Subject, db: &Pool<Postgres>)->Result<u64,sqlx::error::Error> {
        // start transaction
        let mut tx = db.begin().await?;
        
        let resp = query!(
            "INSERT INTO subject values($1,$2,$3,$4,$5)",
            data.coursecode,
            data.name,
            data.semester,
            data.dept,
            data.maxcapacity
        ).execute(&mut tx).await?;
        // commit 
        tx.commit().await?;
        return Ok(resp.rows_affected());
    }
}
