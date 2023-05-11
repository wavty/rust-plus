mod cmd;
use std::collections::HashMap;

use anyhow::{Ok, Result};
use clap::Parser;
use cmd::{Get, Post, Subcmd};
use colored::Colorize;
use mime::Mime;
use reqwest::{header, Client, Response};

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

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.key, &pair.value);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    Ok(print_resp(resp).await?)
}

async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    Ok(print_resp(resp).await?)
}

fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers().get(header::CONTENT_TYPE).map(|v| {
        let tmp = v.to_str().unwrap().parse();
        tmp.unwrap()
    })
}

async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, &body);
    Ok(())
}

fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status)
}

fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }
    print!("\n")
}

fn print_body(m: Option<Mime>, body: &String) {
    match m {
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan())
        }
        _ => println!("{}", body),
    }
}
