use std::ptr::null;
use rocket::response::content::Json;
use crate::api::service_trait::{QiniuServer, RegisterRequest, Reply};
use crate::biz::user::{User, UserRepo, UserUseCase};
use crate::errors::errors::ClientError;
use crate::server::response::{error_response, success_response};

pub struct QiniuService{
    pub uc:*UserUseCase
}

impl QiniuServer for QiniuService {
    #[post("/user/register"),data="<req>"]
    fn register(self, req: Json<RegisterRequest>) -> Reply {
        let err = self.uc.register(User {
            user_name: req.0.user_name,
            password: req.0.password,
            phone_number: req.0.phone_number,
        });
       match  err.downcast::<ClientError>(){
           Ok(_) => success_response(null(),"Register successfully".to_string()),
           Err(e) => {
               log::error!("{}",e);
               error_response(1,e.to_string())
           }
       }
    }
}



