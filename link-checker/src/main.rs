use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use reqwest::blocking::{get, Response};
use reqwest::Url;
use scraper::{Html, Selector};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

fn extract_links(response: Response) -> Result<Vec<Url>, Error> {
    let base_url = response.url().to_owned();
    let document = response.text()?;
    let html = Html::parse_document(&document);
    let selector = Selector::parse("a").unwrap();

    let mut valid_urls = Vec::new();
    for element in html.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            match base_url.join(href) {
                Ok(url) => valid_urls.push(url),
                Err(err) => {
                    println!("On {base_url}: could not parse {href:?}: {err} (ignored)",);
                }
            }
        }
    }

    Ok(valid_urls)
}

fn main() {
    let start_urls = vec![
        "https://example.com/",
        "https://crypto.stanford.edu/cs142/projects/1/project1.html",
        "https://crypto.stanford.edu/cs142/projects/2/project2.html",
    ];

    let (tx, rx) = mpsc::sync_channel(start_urls.len());

    for start_url in start_urls {
        let tx = tx.clone();
        let _ = thread::spawn(move || {
            let thread_id = thread::current().id();

            let response = get(start_url).unwrap();
            match extract_links(response) {
                Ok(links) => tx
                    .send(format!("{:#?} {:#?}", thread_id, links))
                    .unwrap(),
                Err(err) => println!("Could not extract links: {err:#}"),
            }
        });
    }

    drop(tx);

    for msg in rx.iter() {
        println!("Main: got {}", msg);
    }
}
