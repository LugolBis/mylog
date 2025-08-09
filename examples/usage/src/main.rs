use mylog::{error, info, warn, logs::init};

fn main() {
    if let Err(error) = init(
        "tests/path/to/mylogs".to_string(), 
        "50 ko".to_string(), 
        "2 days".to_string()
    )
    {
        panic!("Failed to initialize logs :\n{}", error);
    };

    info!("Welcome on MyLog !");
    warn!("Driving too fast is dangerous -> {:#?}", ["car0", "car1"]);
    error!("{} NOT FOUND - We can't find your dignity...", 404);
}
