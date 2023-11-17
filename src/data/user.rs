use std::error::Error;
use regex::Regex;
use sea_orm::{ActiveModelTrait, EntityTrait};
use sea_orm::ActiveValue::Set;
use crate::biz::user::{User, UserRepo};
use crate::data::data::Data;
use crate::data::entity::users;
use crate::data::entity::users::Entity;
use crate::errors::errors::{ClientError};

pub struct userRepo {
    data:Data
}

impl UserRepo for userRepo{
    async fn register(self, user:User) -> Box<dyn Error> {
        validate_password(user.password)?;
        validate_phone_number(user.phone_number)?;
       let user_info= users::ActiveModel {
           id: Set(0),
           user_name: Set(user.user_name.to_owned()),
           phone_number: Set(user.phone_number.to_owned()),
       };
        let result = Entity::insert(user_info).exec(&self.data.db).await?;
    }

    fn login(self,user: User) -> Result<String, Box<dyn Error>> {

    }

    fn get_user_info(self, user_id: i64) -> Result<crate::biz::user::User, Box<dyn Error>> {

    }
}


fn validate_password(password: String) ->Box<ClientError>{
    if password.len() < 8 {
        Box::new(ClientError("密码太短，至少需要8个字符。".to_string()))
    } else if !password.chars().any(|c| c.is_uppercase()) {
        Box::new(ClientError("密码需要至少一个大写字母。".to_string()))
    } else if !password.chars().any(|c| c.is_lowercase()) {
        Box::new(ClientError("密码需要至少一个小写字母。".to_string()))
    } else if !password.chars().any(|c| c.is_numeric()) {
        Box::new(ClientError("密码需要至少一个数字。".to_string()))
    } else {
       Ok(())
    }
}


fn validate_phone_number(phone:String)->Box<ClientError>{
    let re = Regex::new(r"^1[3-9]\d{9}$")?;
    if !re.is_match(&*phone){
        Box::new(ClientError("手机号不符合逻辑".to_string()))
    }else {
        Ok(())
    }
}

