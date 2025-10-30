use crate::{
    errors::response::ResponseErrors,
    misc::auth::{AuthTokenData, UserDetails, UserType},
};

use super::auth::validate_jwt;
use actix_web::{HttpMessage, dev::ServiceRequest, web::ReqData};
use log::{trace, warn};

/// Insert authenticator details
pub fn jwt_authentication(req: &ServiceRequest, jwt_secret_for_middleware: &String) {
    trace!("HTTP request on {}", req.path());
    let header = req.headers().get("Authorization");
    if let Some(header) = header {
        if let Ok(x) = header.to_str() {
            let res = x.trim().split(' ').nth(1);
            if let Some(res) = res {
                // get the token
                let res: &String = &res.into();
                // if valid, insert it
                if let Ok(AuthTokenData {
                    user_details: user, ..
                }) = validate_jwt(jwt_secret_for_middleware, res)
                {
                    trace!("Got a valid token for uid {}", user.uid);
                    let mut exts = req.extensions_mut();
                    exts.insert(user);
                } else {
                    trace!("JWT illegal");
                }
            }
        }
    }
}

/// Assert some role is present and return the User Details. If no role present, all roles will be authorized.
pub fn assert_role_auth(
    ext: Option<ReqData<UserDetails>>,
    role_expected: Option<UserType>,
) -> Result<UserDetails, ResponseErrors> {
    let ext_map = ext.map(|ex| ex.into_inner());
    assert_role_auth_addr(ext_map, role_expected)
}

fn assert_role_auth_addr(
    ext: Option<UserDetails>,
    role_expected: Option<UserType>,
) -> Result<UserDetails, ResponseErrors> {
    if let Some(user_details) = ext {
        match role_expected {
            Some(expected_user_type) => {
                let user_type_got = user_details.user_type;
                if user_type_got == expected_user_type {
                    Ok(user_details)
                } else {
                    Err(ResponseErrors::Forbidden)
                }
            }
            None => Ok(user_details),
        }
    } else {
        Err(ResponseErrors::Unauthorized)
    }
}

#[cfg(test)]
mod test {
    use actix_web::web::ReqData;

    use crate::{
        errors::response::ResponseErrors,
        misc::{
            auth::{UserDetails, UserType},
            middleware::{assert_role_auth, assert_role_auth_addr},
        },
    };

    #[test]
    pub fn test_role_assert_nothing_provided() {
        const ROLE: Option<ReqData<UserDetails>> = None;
        let roles = vec![None, Some(UserType::Student), Some(UserType::Admin)];
        for role in roles {
            let res = assert_role_auth(ROLE, role);
            assert!(res.is_err());
            assert!(res.unwrap_err() == ResponseErrors::Unauthorized);
        }
    }

    #[test]
    pub fn test_role_assert_a_role() {
        let user_details = UserDetails {
            uid: 1,
            user_type: UserType::Student,
        };
        let role_expected = None;

        let res = assert_role_auth_addr(Some(user_details), role_expected);
        assert!(res.is_ok());
        assert!(res.unwrap() == user_details);
    }
    #[test]
    pub fn test_role_assert_a_role_match() {
        let user_details = UserDetails {
            uid: 1,
            user_type: UserType::Student,
        };
        let role_expected = UserType::Student;

        let res = assert_role_auth_addr(Some(user_details), Some(role_expected));
        assert!(res.is_ok());
        assert!(res.unwrap() == user_details);
    }
    #[test]
    pub fn test_role_assert_a_role_mismatch() {
        let user_details = UserDetails {
            uid: 1,
            user_type: UserType::Student,
        };
        let role_expected = UserType::Admin;

        let res = assert_role_auth_addr(Some(user_details), Some(role_expected));
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), ResponseErrors::Forbidden);
    }
}
