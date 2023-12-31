#[warn(unused_imports)]
// use anyhow::Result;
// use tokio;
// use std::io;
use std::{fs::File, io::Write};

pub async fn download_index_html(url: &str, file_path: &str) -> std::result::Result<(), anyhow::Error> {
    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let mut file = File::create(file_path)?;
        file.write_all(&response.bytes().await?)?;
        println!("Website Cloned Successfully!");
    } else {
        println!("Failed to download index.html: {}", response.status());
    }
    Ok(())
}
