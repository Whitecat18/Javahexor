// async Function for Tunneling !!
//

use std::io::{Write,BufRead, BufReader};
use std::process::{Command, Stdio};
// use crate::{new_users, open_help, start_http_server};
use std::process::exit;
const ERROR : &str = "Failed to start the server . Please check if the tunnel or endpoint services are installed properly";

#[allow(unused_macros)]

macro_rules! inp_num {
    ($out:ident as $type:ty) => {
        std::io::stdout().flush().unwrap();
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("Provide Proper input");
        let $out = inner.trim().parse::<$type>().expect("Error occurs while parsing . Please check the code");
    };
}
pub async fn tunnel() -> Result<(), anyhow::Error> {
    println!();
    println!("Select the Providers");
    println!("1.TunnelMole \n2.Telebit \n3.Bore \n4.Tunnel Pyjam ");
    print!("Your Option :");
    inp_num!(y as u8);

    // let error_msg = "Failed to start the server . Please check if the tunnel or endpoint services are installed properly";
    match y {
        1 => {
            println!("Starting TunnelMole !");
            start_tmole().await?;
            println!("Closed without any error . Exiting");
            exit(1);
        },
        2 => {
            println!("Starting Telebit !");
            start_telebit().await?;
            println!("Closed without any error . Exiting");
            exit(1);
        },
        3 => {
            println!("Starting bore");
            start_bore().await?;
            println!("Closed without any error . Exiting");
            exit(1);
        },
        4 => {
            println!("Showcasing :");
            start_tunnel_pyjam().await?;
            exit(1);
        },
        _ => {
            println!("Exited without any Error");
        },
    };
    Ok(())
}

pub async fn start_tmole() -> Result<(), anyhow::Error>{
    let mut output = Command::new("tmole")
        .arg("8080")
        .stdout(Stdio::piped())
        .spawn()
        .expect(ERROR);
    let output = BufReader::new(output.stdout.take().expect(ERROR));

    for line in output.lines(){
        let line = line.expect(ERROR);
        println!("{}",line);
    }
    Ok(())
}

pub async fn start_telebit() -> Result<(), anyhow::Error>{
    // Command::new("sudo").arg("systemctl").arg("start").arg("telebit").output().expect(ERROR);
    let mut output = Command::new("sudo")
        .arg("telebit")
        .arg("http")
        .arg("8080")
        .stdout(Stdio::piped())
        .spawn()
        .expect(ERROR);
    let output = BufReader::new(output.stdout.take().expect(ERROR));

    for line in output.lines(){
        let line = line.expect(ERROR);
        println!("{}",line);
    }
    println!("CTRL+C to Exit");
    tokio::time::sleep(std::time::Duration::from_secs(86400)).await;
    Ok(())
}

pub async fn start_bore() -> Result<(), anyhow::Error>{
    let mut output = Command::new("bore")
        .arg("local")
        .arg("8080")
        .arg("--to")
        .arg("bore.pub")
        .stdout(Stdio::piped())
        .spawn()
        .expect(ERROR);
    let output = BufReader::new(output.stdout.take().expect(ERROR));

    for line in output.lines(){
        let line = line.expect(ERROR);
        println!("{}",line);
    }
    Ok(())
}

pub async fn start_tunnel_pyjam() -> Result<(), anyhow::Error>{
    println!("This is an different tunnel , you must enable it automatically !");
    println!("Open New Terminal and execute the following commands ");
    println!("-----------");
    println!("curl https://tunnel.pyjam.as/8080 > tunnel.conf && wg-quick up ./tunnel.conf ");
    println!("-----------");
    println!();
    println!("To stop the tunnel . Type !");
    println!("-----------");
    println!("wg-quick down ./tunnel.conf");
    println!("-----------");
    println!();
    println!("Press any key to continue...");
    std::io::stdin().read_line(&mut String::new()).unwrap();

    Ok(())
}

