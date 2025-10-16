use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, PgPool};

#[derive(Serialize, Deserialize, Debug)]
pub struct Booking {
    pub uid: i32,
    pub insert_time: chrono::DateTime<chrono::Utc>,
    pub course_code: String,
}
impl Booking {
    pub async fn to_external(self: Self, db: &PgPool) -> Result<BookingPresentation, sqlx::Error> {
        let Booking {
            uid,
            course_code,
            insert_time,
        } = self;

        let ans = query!("select email from userauth where uid=$1", uid)
            .fetch_one(db)
            .await?;
        let email = ans.email;

        Ok(BookingPresentation {
            email,
            course_code,
            insert_time,
        })
    }
    pub async fn get_all(db: &PgPool) -> Result<Vec<Booking>, sqlx::Error> {
        let ans = query_as!(
            Booking,
            "select uid,coursecode as course_code,insert_time from book"
        )
        .fetch_all(db)
        .await?;
        Ok(ans)
    }
    pub async fn get_user(uid: i32, db: &PgPool) -> Result<Vec<Booking>, sqlx::Error> {
        let ans = query_as!(
            Booking,
            "select uid,coursecode as course_code,insert_time from book where uid=$1",
            uid
        )
        .fetch_all(db)
        .await?;
        Ok(ans)
    }
    /// make a booking for a user
    pub async fn make_user(uid: i32, course_code: String, db: &PgPool) -> Result<(), sqlx::Error> {
        query!("insert into book values($1,$2)", uid, course_code)
            .execute(db)
            .await?;
        Ok(())
    }
    pub async fn clear_user(uid: i32, course_code: String, db: &PgPool) -> Result<(), sqlx::Error> {
        query!("delete from book where uid=$1", uid)
            .execute(db)
            .await?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BookingPresentation {
    pub email: String,
    pub course_code: String,
    pub insert_time: chrono::DateTime<chrono::Utc>,
}

impl BookingPresentation {
    /// get internal representation of booking
    pub async fn to_internal(self: Self, db: &PgPool) -> Result<Booking, sqlx::Error> {
        let BookingPresentation {
            course_code,
            email,
            insert_time,
        } = self;

        let uidres = query!(
            "Select uid from userauth where email=$1::varchar(64)",
            email
        )
        .fetch_one(db)
        .await?;
        let uid = uidres.uid;

        Ok(Booking {
            uid,
            course_code,
            insert_time,
        })
    }
}
