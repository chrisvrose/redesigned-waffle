use crate::{errors::response::ResponseErrors, misc::auth::{AuthTokenData, UserDetails, UserType}};

use super::auth::validate_jwt;
use actix_web::{dev::ServiceRequest, web::ReqData, HttpMessage, ResponseError};
use log::trace;


/// Insert authenticator details
pub fn jwt_authentication(req: &ServiceRequest, jwt_secret_for_middleware: &String) {
    trace!("HTTP request on {}",req.path());
    let header = req.headers().get("Authorization");
    if let Some(header) = header {
        if let Ok(x) = header.to_str() {
            let res = x.trim().split(' ').nth(1);
            if let Some(res) = res {
                // get the token
                let res: &String = &res.into();
                // if valid, insert it
                if let Ok(AuthTokenData{user_details:user,..}) = validate_jwt(jwt_secret_for_middleware, res) {
                    trace!("Got a valid token {} for uid {:?}", res, (user.uid));
                    let mut exts = req.extensions_mut();
                    exts.insert(user.uid);
                }
            }
        }
    }
}

/// Assert some role is present and return the User Details. If no role present, all roles will be authorized.
pub fn assert_role_auth(ext: Option<ReqData<UserDetails>>,role_expected:Option<UserType>)->Result<UserDetails,ResponseErrors>{
    if let Some(user_details_reqdata) = ext{
        let user_details = user_details_reqdata.into_inner();
        match role_expected{
            Some(expected_user_type) =>{
                let user_type_got = user_details.user_type;
                if (user_type_got==expected_user_type){
                    Ok(user_details)
                }else{
                    Err(ResponseErrors::Forbidden)
                }
            },
            None => Ok(user_details),
        }
    } else{
        Err(ResponseErrors::Unauthorized)
    }
}
