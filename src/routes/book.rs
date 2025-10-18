use actix_web::{Responder, HttpResponse,get, web::{Data, ReqData}, post};

use crate::{dto::NewBookDTO, errors::response::ResponseResult, misc::{auth::{UserDetails, UserType}, middleware::assert_role_auth, AppData}, models::Booking};


#[post("")]
pub async fn make_booking(appdata:Data<AppData>,exts:Option<ReqData<UserDetails>>,body:actix_web::web::Json<NewBookDTO>)-> ResponseResult<HttpResponse>{
    let UserDetails { uid, .. } = assert_role_auth(exts, Some(UserType::Student))?;
    let db = &appdata.pool;
    let request_body_dto = body.into_inner();
    Booking::make_booking_for_user(uid, request_body_dto.course_code, db).await?;
    Ok(HttpResponse::Ok().body(""))
}


#[get("")]
pub async fn get_user(appdata:Data<AppData>,userdata_opt:Option<ReqData<UserDetails>>)->impl Responder{
    let db = &appdata.pool;
    match userdata_opt {
        Some(userdata)=>{
            let UserDetails { uid, user_type } = userdata.into_inner();
            if let UserType::Student = user_type {
                let bookings = Booking::get_user(uid, db).await;
                match bookings {
                    Ok(ans)=>{
                        HttpResponse::Ok().json(ans)
                    },
                    _=>{
                        HttpResponse::NotFound().json(serde_json::json!({"ok":false}))
                    }
                }
            }else{
                HttpResponse::NotAcceptable().json(serde_json::json!({"ok":false}))
            }
        },
        None=>{
            HttpResponse::Unauthorized().json(serde_json::json!({"ok":false}))
        }
    }
}

// todo only for service accounts
#[get("/calc")]
pub async fn get_calc(_appdata:Data<AppData>)->impl Responder{

    HttpResponse::NotAcceptable().json(serde_json::json!({"ok":false}))
}
