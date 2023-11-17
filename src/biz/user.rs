use std::error::Error;

pub trait UserRepo {
    fn register(self,user: User) -> Box<dyn Error>;
    fn login(self,user: User)->Result<String,Box<dyn Error>>;
    fn get_user_info(self,user_id:i64) ->Result<User,Box<dyn Error>>;
}


pub(crate) struct User{
    pub user_name:String,
    pub password:String,
    pub phone_number:String
}

struct UserUseCase{
    repo: dyn UserRepo,
}

impl UserRepo for UserUseCase {
    fn register(self,user: User) -> Box<dyn Error> {
        self.repo.register(user: User)
    }

    fn login(self,user: User) -> Result<String, Box<dyn Error>> {
        self.repo.login(user: User)
    }

    fn get_user_info(self,user_id: i64) -> Result<User, Box<dyn Error>> {
        self.get_user_info(user_id)
    }
}
