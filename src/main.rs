use std::fs;
use std::env;
use chrono::Local;

fn main() {
    let mut file = Local::now()
        .format("%b%d")
        .to_string()
        .to_uppercase();
    let note_template = format!("#{}

---
Brain dump
- 
    ", file);
    file.push_str(".md");
    let cwd = env::current_dir()
        .expect("ERROR")
        .join(file);
    fs::write(cwd.as_os_str(), note_template)
        .expect("Error in writing file");
}
