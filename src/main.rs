#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

extern crate csv;
extern crate indicatif;
extern crate num_cpus;
extern crate pretty_env_logger;
extern crate regex;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

mod endpoints;
mod genius_crawl;
mod io;
mod types;
mod utils;

use std::{
    sync::{
        Arc,
    },
};

use reqwest::{
    Client,
};

fn main() {
    pretty_env_logger::init();
    
    let client = Arc::new(Client::new());
    genius_crawl::main(client);
}
