use std::any::Any;
use rocket::response::content::Json;

use serde::{Deserialize, Serialize};

pub trait QiniuServer{
    fn register(self,req: Json<RegisterRequest>)->Reply;
}
#[derive(Deserialize,Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RegisterRequest{
   pub user_name:String,
    pub password:String,
    pub phone_number:String,
}

pub struct Reply{
    pub(crate) status:i64,
    pub(crate) code:i64,
    pub(crate) data:Box<dyn Any>,
    pub(crate) msg:String,
}
