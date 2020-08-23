use std::path::PathBuf;
use structopt::StructOpt;
use std::process::Command;
//use clap::arg_enum;
//use std::str::FromStr;
//use std::string::ParseError;
use serde::{Deserialize, Serialize};
//use serde_json::{Result};
use std::error::Error;
use std::fs::File;
use std::fs;
use std::io::Read;

mod dns;
mod cloudflare;

use dns::*;

/*
impl FromStr for IPV {
    type Err = ParseError;
    fn from_str(ipv: &str) -> Result<Self, Self::Err> {
        match ipv {
            "v6" => Ok(IPV::v6),
            "v4" => Ok(IPV::v4),
            _ => Ok(IPV::v6),
        }
    }
}


impl IPV {
    fn dns_record_type(&self) -> &str {
        match self {
            IPV::v4 => "A",
            IPV::v6 => "AAAA"
        }
    }
}
*/

#[derive(Debug, StructOpt)]
#[structopt(about = "A tool for automating dynamic DNS")]
enum DDNS {
    /// Configure dynamic DNS
    Configure(Config),
    /// Update the remote DNS records to point to the current IP of this machine
    Update,
    /// Print the current IP address of this machine
    Ip {
        #[structopt(short, long, default_value = "v6")]
        version: IPV
    }
}

#[derive(Debug, StructOpt)]
pub struct Opts{
    #[structopt(subcommand)]
    commands: Option<DDNS>
}


fn get_current_ip(ipv: &IPV) -> Option<String> {
    let output = Command::new("dig")
        .args(&[
            "+short",
            "myip.opendns.com",
            "@resolver1.opendns.com",
            ipv.dns_record_type()
        ])
        .output()
        .expect("failed to execute dig");

    return String::from_utf8(output.stdout).ok();
}

// Handling the config file

#[derive(Debug, Serialize, Deserialize, StructOpt)]
struct Config {
    #[structopt(short, long)]
    /// the domain dynamic DNS will be configured for
    domain: String,

    #[structopt(short = "k", long = "api-key")]
    /// the cloudflare api key as documented here: https://api.cloudflare.com/
    api_key: String,

    #[structopt(short = "z", long = "zone-id")]
    /// the cloudflare zone id where this domain is hosted
    zone_id: String
}

static CONFIG_PATH: &str = "~/.config/ddns/config.json";

impl Config {

    fn write(&self) -> Result<(), Box<dyn Error>> {
        fs::create_dir_all("~/.config/ddns")?;
        let file = File::create(CONFIG_PATH)?;
        serde_json::to_writer(&file, &self)?;
        Ok(())
    }

    fn read() -> Result<Config, Box<dyn Error>> {
        let mut file = File::open(CONFIG_PATH)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config: Config = serde_json::from_str(&contents)?;

         return Ok(config);
    }

}

fn show_status() {

    let config = Config::read();
    match config {
        Ok(conf) => {
            println!("Configuration exists at path: {}\n{:?}", CONFIG_PATH, conf);
        },
        Err(err) => {
            println!("Failed to get config at: {} with error: {:?}", CONFIG_PATH, err);
        }
    }

}


fn main() {
    let opts = Opts::from_args();
    println!("{:#?}", opts);

    if let Some(subcommand) = opts.commands {
        match subcommand {
            DDNS::Configure(config) => {
                println!("Configure");
                match config.write() {
                    Ok(()) => {
                        println!("Wrote config to: {}", CONFIG_PATH);
                    },
                    Err(err) => {
                        println!("Failed to write config with error: {:?}", err);   
                    }
                }
            },
            DDNS::Update => {
                match Config::read() {
                Err(err) => { println!("Failed to read config at: {} with error: {}", CONFIG_PATH, err); },
                Ok(config) => {
                    println!("updating..");
                    let ipv6_addr = get_current_ip(&IPV::v6);
                    let ipv4_addr = get_current_ip(&IPV::v4);
                    println!("ipv6: {:?}\nipv4: {:?}", ipv6_addr, ipv4_addr);
                    if let Some(ips) = IP_Set::with(ipv4_addr, ipv6_addr) {
                    
                    
                    
                        let res = cloudflare::update_dns(config.domain, config.zone_id, config.api_key, &ips);
                        println!("Result: {:?}", res);
                    } else {
                        println!("No ipv4 or ipv6 address could be found for this machine");
                    }
                }
                }
            },
            DDNS::Ip { version } => {
                println!("Current IP: {} {}", &version.dns_record_type(), get_current_ip(&version).unwrap_or("".to_string()));
            }
        }

    } else {
        show_status();
    }
}

