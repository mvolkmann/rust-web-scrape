extern crate reqwest;
extern crate tokio;

//use futures::prelude::*;
use reqwest::header::USER_AGENT;
use std::boxed::Box;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
//use std::thread;
use std::time::Instant;

// We need to set the user agent because some sites return 403 Forbidden
// for requests that do not seem to be coming from a web browser.
//const UA: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 11_1_0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/88.0.4324.146 Safari/537.36";
const UA: &str = "Mozilla/5.0"; // This is enough.

type FileLines = Lines<BufReader<File>>;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

async fn get_sites() -> Result<FileLines> {
    let path = "./web-sites.txt";
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    Ok(reader.lines())
}

async fn process_site(url: &str) -> Result<()> {
    // If commented ...
    if url.starts_with("#") {
        return Ok(());
    }

    let client = reqwest::Client::new();
    let res = client.get(url).header(USER_AGENT, UA).send().await?;
    let html = res.text().await?;
    //dbg!(&html);
    let images: Vec<&str> = html.matches("<img ").collect();
    println!("{} has {} img tags", url, images.len());
    Ok(())
}

#[tokio::main] // starts the Tokio runtime
async fn main() -> Result<()> {
    // Single threaded ...
    let sites = get_sites().await?;
    let start = Instant::now();
    for site in sites {
        if let Ok(url) = site {
            process_site(&url).await?;
        }
    }
    println!("single-threaded time: {:?}\n", start.elapsed());

    // Multi-threaded ...
    let sites = get_sites().await?;
    let start = Instant::now();
    let mut handles = Vec::new();
    for site in sites {
        //handles.push(thread::spawn(|| async move {
        handles.push(tokio::task::spawn(async {
            if let Ok(url) = site {
                process_site(&url).await?;
            }
            // The ? operator used above propagates the error
            // and can convert it to a different type.
            // The error type isn't specified, so we do that the next line.
            Ok::<_, GenericError>(())
        }));
    }
    for handle in handles {
        if let Err(e) = handle.await? {
            eprintln!("error: {}", e);
        }
    }
    println!("multi-threaded time: {:?}", start.elapsed());

    Ok(())
}
