use serde::{Deserialize, Serialize};

/// Outfacing user -> Missing pwd
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutUserDTO {
    pub uid: i32,
    pub name: String,
    pub email: String,
    pub semester: Option<i32>,
    pub deptid: String,
}
