use sea_orm::DeriveEntityModel;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel,Serialize,Deserialize)]
#[sea_orm(table_name="users")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub id:i32,
    pub user_name:String,
    pub phone_number:String
}
