use serde::{Serialize, Deserialize};
use sqlx::{PgPool,Error,query_as};



#[derive(Debug,Serialize,Deserialize)]
pub struct Dept{
    pub deptid:String,
    pub name:String
}



impl Dept{
    pub async fn get_all(db:&PgPool)->Result<Vec<Dept>,Error>{
        Ok(query_as!(Dept,"Select * from dept").fetch_all(db).await?)
    }
}