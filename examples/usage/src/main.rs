use mylog::{error, info, warn, logs::init};

fn main() {
    init("path/to/mylogs".to_string());

    info!("Welcome on MyLog !");
    warn!("Driving too fast is dangerous -> {:#?}", ["car0", "car1"]);
    error!("{} NOT FOUND - We can't find your dignity...", 404);
}
