use crate::api::service_trait::Reply;

pub fn success_response<T>(data:T,msg:String) ->Reply{
    Reply{
        status: 0,
        code: 0,
        data,
        msg,
    }
}

pub fn error_response(code:i64,msg: String)->Reply{
    Reply{
        status: -1,
        code,
        data: Box::new(()),
        msg,
    }
}
