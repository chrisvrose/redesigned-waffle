use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, PgPool, Pool, Postgres};
#[derive(Serialize, Deserialize, Debug)]
pub struct Subject {
    pub coursecode: String,
    pub name: String,
    pub semester: i32,
    pub isglobal: bool,
    pub deptid: String,
    pub maxcapacity: i32,
}

impl Subject {
    pub async fn get_all(db: &Pool<Postgres>) -> Result<Vec<Subject>, sqlx::error::Error> {
        let resp = query_as!(Subject, "select * from subject")
            .fetch_all(db)
            .await;
        resp
    }

    pub async fn get_for_user(id: &i32, db: &PgPool) -> Result<Vec<Subject>, sqlx::error::Error> {
        let resp = query_as!(Subject,"select subject.* from subject,userauth where userauth.uid=$1 and userauth.semester=subject.semester and ((subject.isglobal and userauth.deptid!=subject.deptid) or (not(subject.isglobal) and userauth.deptid=subject.deptid))",id).fetch_all(db).await?;

        Ok(resp)
    }

    pub async fn get_one(
        id: &String,
        db: &Pool<Postgres>,
    ) -> Result<Option<Subject>, sqlx::error::Error> {
        let resp = query_as!(Subject, "Select * from subject where coursecode=$1", id)
            .fetch_optional(db)
            .await?;
        Ok(resp)
    }

    // pub async fn insert(data: &Subject, db: &Pool<Postgres>) -> Result<u64, sqlx::error::Error> {
    //     // start transaction
    //     let mut tx = db.begin().await?;
    //     // let resp = query!("Insert into subject select * from ")

    //     /* let resp = */
    //     let resp = query!(
    //         "INSERT INTO subject values($1,$2,$3,$4,$5,$6)",
    //         data.coursecode,
    //         data.name,
    //         data.semester,
    //         data.isglobal,
    //         data.deptid,
    //         data.maxcapacity
    //     )
    //     .execute(&mut tx)
    //     .await?;
    //     // commit

    //     tx.commit().await?;
    //     return Ok(resp.rows_affected());
    // }

    pub async fn insert_all(
        subjects: &Vec<Subject>,
        db: &Pool<Postgres>,
    ) -> Result<Vec<String>, sqlx::error::Error> {
        let mut tx = db.begin().await?;
        let mut inserted_ids: Vec<String> = Vec::new();
        for subject in subjects {
            let resp = query!(
                r#"INSERT INTO subject values ($1,$2,$3,$4,$5,$6) returning coursecode"#,
                subject.coursecode,
                subject.name,
                subject.semester,
                subject.isglobal,
                subject.deptid,
                subject.maxcapacity
            ).fetch_one(&mut tx)
            // .execute(&mut tx)
            .await?;

            
            inserted_ids.push(resp.coursecode);
        }
        tx.commit().await?;
        return Ok(inserted_ids);
    }
}
