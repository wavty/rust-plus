mod cmd;
use std::collections::HashMap;

use anyhow::{Ok, Result};
use clap::Parser;
use cmd::{Get, Post, Subcmd};
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = cmd::Opts::parse();
    // 创建http客户端
    let client = Client::new();
    let result = match opts.subcmd {
        Subcmd::Get(ref args) => get(client, args).await?,
        Subcmd::Post(ref args) => post(client, args).await?,
    };
    Ok(result)
}

async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    println!("{:?}", resp.text().await?);
    Ok(())
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.key, &pair.value);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    println!("{:?}", resp.text().await?);
    Ok(())
}
