

fn init_logger(){
    log4rs::init_file("./log4rs.yaml",Default::default()).unwrap();
    log::info!("[INFO]INIT Log SUCCESS")
}
