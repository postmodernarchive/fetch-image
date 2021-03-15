#[macro_use]
extern crate log;
use env_logger::{Builder, Target};
use std::env;
use dotenv::dotenv;
use chrono::{Utc};

use std::fs;

mod fetch;
mod parse;

fn main() {
    dotenv().ok();

    let mut log_builder = Builder::from_default_env();
    log_builder.target(Target::Stdout);
    log_builder.init();

    let account_list: String =  env::var("ACCOUNT_LIST").expect("ACOUNT_LIST must be set");

    for acc in account_list.split(',').collect::<Vec<_>>() {
        // using unix timestamp
        let filename = format!("./data/{}-{}.json", Utc::now().format("%s"), acc.to_string());
        let data = fetch::get_profile_info(acc.to_string());
        if !data.is_empty() {
            fs::write(filename.to_string(), data).unwrap();

            info!("Wrote file: {}", filename);
        }
    }

    let mut posts_str: Vec<String> = Vec::new();

    for i in 0..12 {
        posts_str.push(
            parse::get_post_by_id(
                i,
                parse::read_file("./data/www.instagram.com.json".to_string())
            )
        );
    }

    for p in posts_str {
        match parse::str_to_post(p) {
            None => (),
            Some(v) => debug!("{}", v.image_url),
        }
    }
}
