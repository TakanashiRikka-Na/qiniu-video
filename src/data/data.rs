use crate::data::*;
use crate::data::db::DB;
use crate::data::rdb::RDB;
pub struct Data{
   pub db:db::DB,
   pub rdb:rdb::RDB
}

impl Data{
   pub fn new_data()->Data{
        let db = DB::init_db().expect("connect to database error");
        let rdb=RDB::init_rdb().expect("connect to redis error");
        Data{rdb,db}
    }
}
