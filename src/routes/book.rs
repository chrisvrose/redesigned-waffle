use actix_web::{Responder, HttpResponse,get, web::{Data, ReqData}, post};

use crate::{misc::{AppData, auth::UserType}, models::{BookingPresentation, Booking, NewBookDTO}};


#[post("")]
pub async fn make_booking(appdata:Data<AppData>,exts:Option<ReqData<UserType>>,body:actix_web::web::Json<NewBookDTO>)-> impl Responder{
    if let Some(userdata) = exts{
        
        let vars = userdata.into_inner();
        if let UserType::Student(uid) = vars{
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
pub async fn get_user(appdata:Data<AppData>,exts:Option<ReqData<UserType>>)->impl Responder{
    let db = &appdata.pool;
    match exts {
        Some(userdata)=>{ 
            let usertype = userdata.into_inner();
            if let UserType::Student(uid) = usertype {
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