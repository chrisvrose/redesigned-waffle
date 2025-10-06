use actix_web::{Responder, HttpResponse,get, web::{Data, ReqData}, post};

use crate::{dto::NewBookDTO, misc::{auth::{UserDetails, UserType}, AppData}, models::Booking};


#[post("")]
pub async fn make_booking(appdata:Data<AppData>,exts:Option<ReqData<UserDetails>>,body:actix_web::web::Json<NewBookDTO>)-> impl Responder{
    if let Some(user_details) = exts{
        let UserDetails { uid, user_type } = user_details.into_inner();
        if let UserType::Student = user_type {
            let db =& appdata.pool;
            let resp = body.into_inner();
            if let Ok(_) = Booking::make_user(uid, resp.course_code, db).await{
                HttpResponse::Ok().json(serde_json::json!({"ok":true}))
            }else{
                HttpResponse::BadRequest().json(serde_json::json!({"ok":false}))
            }
        }else{
            HttpResponse::NotFound().json(serde_json::json!({"ok":false}))
        }
    }else{
        HttpResponse::NotFound().json(serde_json::json!({"ok":false,"reason":"No auth"}))
    }
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
