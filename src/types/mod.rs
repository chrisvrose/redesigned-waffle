use serde::{Serialize};
#[derive(Serialize)]
pub struct JsonResponse<T>{
    pub ok:bool,
    pub response:T
}
impl<T> JsonResponse<T>{
    pub fn new(ok:bool,response:T)->JsonResponse<T>{
        JsonResponse{ok,response}
    }
}