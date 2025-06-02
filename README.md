# 📦 MyLog

A minimal Rust crate providing 3 lightweight logging macros to simplify writing log messages to a file.<br>
🏗️​ This project is in very early development.

## ✨ Features

This crate includes the following macros:
- ```info!()```
- ```warn!()```
- ```error!()```

Each macro:

- Parses input like format!()

- Automatically adds :
    - A timestamp (_Not already implemented_)
    - Log level (INFO, WARN, ERROR)
    - Source file name and line number
- Writes the formatted message to a log.txt file

## 🚀 Usage
[...]
