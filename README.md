# 📦 MyLog

A minimal Rust crate providing 3 lightweight logging macros to simplify writing log messages to a file.

## ✨ Features

This crate includes the following macros:
- ```info!()```
- ```warn!()```
- ```error!()```

Each macro:

- Parses input like format!()

- Automatically adds :
    - A timestamp
    - Log level (INFO, WARN, ERROR)
    - Source file name and line number
- Writes the formatted message to a log.txt file

> [!NOTE]
> You can easily set the folder where the log file is saved with the function ```init()```.

## 🚀 Getting Started

### 🔧​ Install
To start using this crate in your Rust project, you can choose one of the following integration methods :

- Add the crate as a dependency by editing your `Cargo.toml` :
```toml
[dependencies]
mylog = "0.1.2"
```

- Use `cargo add` :
```bash
cargo add mylog
```

### 📚​ Examples
You could clone the repository to test the following example :
```bash
git clone https://github.com/LugolBis/mylog
```
```bash
cd mylog && cargo run examples
```

A simple example :
```rust
// A Rust script

use mylog::{error, info, warn, logs::init};

fn main() {
    // To save the log file in the folder 'path/to/my_logs'
    init("path/to/mylogs".to_string());

    info!("Welcome on MyLog !");
    warn!("Driving too fast is dangerous -> {:#?}", ["car0", "car1"]);
    error!("{} NOT FOUND - We can't find your dignity...", 404);
}
```

This will write the following content in a file `logs.txt` at the root of your project :
```
[2025-06-04 23:35:06] [INFO] [examples/usage/src/main.rs:4] : Welcome on MyLog !
[2025-06-04 23:35:06] [WARNING] [examples/usage/src/main.rs:5] : Driving too fast is dangerous -> [
    "car0",
    "car1",
]
[2025-06-04 23:35:06] [ERROR] [examples/usage/src/main.rs:6] : 404 NOT FOUND - We can't find your dignity...
```

> [!NOTE]
> For performancies prefer the version ```1.0.0```, who's provide less fonctionnalities.
