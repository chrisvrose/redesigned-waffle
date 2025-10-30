use serde::{Deserialize, Serialize};

use crate::{misc::auth::UserType, models::UserAuth};

/// Outfacing user -> Missing pwd
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutUserDTO {
    pub uid: i32,
    pub name: String,
    pub email: String,
    pub semester: Option<i32>,
    pub deptid: String,
    pub role: UserType,
}

impl From<UserAuth> for OutUserDTO {
    fn from(val: UserAuth) -> Self {
        OutUserDTO {
            uid: val.uid,
            email: val.email,
            semester: val.semester,
            name: val.name,
            deptid: val.deptid,
            role: val.role,
        }
    }
}
