use log::debug;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, PgPool, Pool, Postgres};

#[derive(Serialize, Deserialize, Debug)]
pub struct Course {
    pub coursecode: String,
    pub name: String,
    pub semester: i32,
    pub isglobal: bool,
    pub deptid: String,
    pub maxcapacity: i32,
}

impl Course {
    pub async fn get_all(db: &Pool<Postgres>) -> Result<Vec<Course>, sqlx::error::Error> {
        debug!("Fetching all courses");
        let resp = query_as!(Course, "select * from course")
            .fetch_all(db)
            .await;
        resp
    }

    pub async fn get_for_user(id: &i32, db: &PgPool) -> Result<Vec<Course>, sqlx::error::Error> {
        let resp = query_as!(Course,"select course.* from course,userauth where userauth.uid=$1 and userauth.semester=course.semester and ((course.isglobal and userauth.deptid!=course.deptid) or (not(course.isglobal) and userauth.deptid=course.deptid))",id)
                    .fetch_all(db)
                    .await;
        resp
    }

    pub async fn get_one(
        id: &String,
        db: &Pool<Postgres>,
    ) -> Result<Option<Course>, sqlx::error::Error> {
        let resp = query_as!(Course, "select * from course where coursecode=$1", id)
            .fetch_optional(db)
            .await;
        resp
    }

    // pub async fn insert(data: &Subject, db: &Pool<Postgres>) -> Result<u64, sqlx::error::Error> {
    //     // start transaction
    //     let mut tx = db.begin().await?;
    //     // let resp = query!("Insert into course select * from ")

    //     /* let resp = */
    //     let resp = query!(
    //         "INSERT INTO course values($1,$2,$3,$4,$5,$6)",
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
        courses: &Vec<Course>,
        db: &Pool<Postgres>,
    ) -> Result<Vec<String>, sqlx::error::Error> {
        let mut tx = db.begin().await?;
        let mut inserted_ids: Vec<String> = Vec::new();
        for course in courses {
            let resp = query!(
                "INSERT INTO course values ($1,$2,$3,$4,$5,$6) returning coursecode",
                course.coursecode,
                course.name,
                course.semester,
                course.isglobal,
                course.deptid,
                course.maxcapacity
            ).fetch_one(&mut *tx)
            .await?;


            inserted_ids.push(resp.coursecode);
        }
        tx.commit().await?;
        return Ok(inserted_ids);
    }
}
