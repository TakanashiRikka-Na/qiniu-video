use std::env;
use sea_orm::{Database, DatabaseConnection,DbErr};
#[derive(Debug)]
pub struct DB{
    db_conn:DatabaseConnection
}

impl DB{
    async fn init_db_connection() ->Result<DatabaseConnection,DbErr>{
        let url=format!("mysql://{}:{}@{}:{}/{}",
                        env::var("MysqlUser")?,
                        env::var("MysqlPassword")?,
                        env::var("MysqlHost")?,
                        env::var("MysqlPort")?,
                        env::var("MysqlDatabase")?
        );
        let connection = Database::connect(url).await?;
        Ok(connection)
    }

    pub fn init_db()->Result<DB,DbErr>{
        let conn=DB::init_db_connection()?;
        Ok(DB{ db_conn:conn })
    }
}
