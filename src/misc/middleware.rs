use actix_web::{dev::ServiceRequest, HttpMessage};

use super::auth::validate_jwt;
/// Insert authenticator details
pub fn do_auth_insert(req: &ServiceRequest,jwt_secret_for_middleware:&String) {
    let header = req.headers().get("Authorization");
    if let Some(header) = header {
        if let Ok(x) = header.to_str() {
            let res = x.trim().split(' ').nth(1);
            if let Some(res) = res {
                // get the token
                let res: &String = &res.into();
                // if valid, insert it
                if let Ok(user) = validate_jwt(jwt_secret_for_middleware, res) {
                    let mut exts = req.extensions_mut();
                    exts.insert(user.uid);
                }
            }
        }
    }
}
