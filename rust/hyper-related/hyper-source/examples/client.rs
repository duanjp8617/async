#![warn(rust_2018_idioms)]
use std::env;

use hyper::{body::HttpBody as _, Client};
use tokio::io::{self, AsyncWriteExt as _};
use std::ops;

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    // Some simple CLI args requirements...
    let url = match env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("Usage: client <url>");
            return Ok(());
        }
    };

    // HTTPS requires picking a TLS implementation, so give a better
    // warning if the user tries to request an 'https' URL.
    let url = url.parse::<hyper::Uri>().unwrap();
    if url.scheme_str() != Some("http") {
        println!("This example only works with 'http' URLs.");
        return Ok(());
    }
    let c = Client::new();
    let c_clone = c.clone();

    let t1 = tokio::spawn(fetch_url(c, url.clone(), 1));
    let t2 = tokio::spawn(fetch_url(c_clone, url, 2));
    println!("fuck1");
    let _ = t1.await.unwrap();
    println!("fuck2");
    let _ = t2.await.unwrap();
    println!("fuck3");

    Ok(())
}

async fn fetch_url(c: Client<hyper::client::HttpConnector>, url: hyper::Uri, id: i32) -> Result<()> {
    let res = c.get(url, id).await?;

    println!("Request ID {}: Response Status {}", id, res.status());
    Ok(())
}
