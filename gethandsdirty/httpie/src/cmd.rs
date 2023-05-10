use anyhow::{anyhow, Result};
use clap::{Args, Parser, Subcommand};
use reqwest::Url;
use std::format;
use std::str::FromStr;

/// A naive httpie implementation with Rust.
#[derive(Parser, Debug)]
#[command(version, author="wavty", about="httpie", long_about=None)]
pub struct Opts {
    /// httpie subcommand
    #[command(subcommand)]
    pub subcmd: Subcmd,
}

// ref: https://docs.rs/clap/4.2.7/clap/_derive/_tutorial/index.html#subcommands
#[derive(Subcommand, Debug)]
pub enum Subcmd {
    /// Initiate http get request
    Get(Get),
    /// Initiate http post request
    Post(Post),
}

// args validate: https://docs.rs/clap/4.2.7/clap/_derive/_tutorial/index.html#validated-values
#[derive(Args, Debug)]
pub struct Get {
    /// HTTP request url
    #[arg(value_parser = parse_url)]
    pub url: String,
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

#[derive(Args, Debug)]
pub struct Post {
    /// HTTP request url
    #[arg(value_parser = parse_url)]
    pub url: String,
    /// HTTP request body
    #[arg(value_parser = parse_kv_pair)]
    pub body: Vec<KvPair>,
}

#[derive(Debug, Clone)]
pub struct KvPair {
    pub key: String,
    pub value: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            key: (split.next().ok_or_else(err)?).into(),
            value: (split.next().ok_or_else(err)?).into(),
        })
    }
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}
