extern crate reqwest;

use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;
use std::env::args;
use serde::Deserialize;

#[derive(Deserialize)]
struct Response {
    status: String,
    available: Option<bool>,
    tier: Option<String>,
    reason: Option<String>,
}

fn main() -> std::io::Result<()> {
    let path: PathBuf = PathBuf::from(args().nth(1).expect("No File Provided"));

    let file = File::open(path)?;

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let domain = line?;

        let resp: Response = reqwest::get(&format!("https://domain-registry.appspot.com/check?domain={}.dev", domain)).expect("Unable to fetch request")
            .json().expect("Unable to parse JSON");

        if resp.status != "success" {
            if let Some(reason) = &resp.reason {
                println!("{} -> Unknown/Invalid - {}", domain, reason);
                continue;
            }

            println!("{} -> Unknown/Invalid", domain);
            continue;
        }

        if let Some(available) = &resp.available {
            if !available {
                println!("{} -> Unavailable", domain);
                continue;
            }
        }

        if let Some(tier) = &resp.tier {
            if tier != "standard" {
                println!("{} -> P-Available (Premium Available)", domain);
                continue;
            }

            if tier == "standard" {
                println!("{} - S-Available (Standard Available)", domain);
                continue;
            }

            println!("{} -> {}-Available", domain, tier);
        }
    }

    Ok(())
}
