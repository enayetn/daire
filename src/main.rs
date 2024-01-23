use std::fs;
use std::env;
use chrono::Local;

fn main() {
    let mut file = Local::now()
        .format("%b%d")
        .to_string()
        .to_uppercase();

    let note_template = get_journal_template(&file);

    file.push_str(".md");
    let cwd = env::current_dir()
        .expect("ERROR")
        .join(file);

    if cwd.exists() {
        println!("Looks like you've already generated a diary entry for today -- if this is a mistake, delete the existing entry and try again");
        return
    }

    fs::write(cwd.as_os_str(), note_template)
        .expect("Error in writing file");
}

fn get_journal_template(template_title: &String) -> String {
    format!("#{}


    ---
    ## Rituals
    - [ ] Journal Entry
    - [ ] At least 1 line of Rust
        ", template_title)
}