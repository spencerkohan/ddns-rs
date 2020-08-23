use std::path::PathBuf;
use structopt::StructOpt;
use std::str::FromStr;
use std::string::ParseError;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;

#[derive(Debug, StructOpt)]
pub enum IPV {
    v4, 
    v6  
}

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
    pub fn dns_record_type(&self) -> &str {
        match self {
            IPV::v4 => "A",
            IPV::v6 => "AAAA"
        }   
    }   
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IP_Set {
    pub ipv6: Option<String>,
    pub ipv4: Option<String>
}

impl IP_Set {

    /// Returns an IP set if either an ipv4 or ipv6 address is provided 
    pub fn with(ipv4: Option<String>, ipv6: Option<String>) -> Option<IP_Set> {
        // TODO: IP validation can happen here (is it a valid IPV4/6 address)
        let v4 = match ipv4 {
            None => None,
            Some(addr) => if addr.len() > 0 { Some(addr) } else { None }
        };

        let v6 = match ipv6 {
            None => None,
            Some(addr) => if addr.len() > 0 { Some(addr) } else { None }
        };

        if v4 == None && v6 == None {
            return None
        }
    
        return Some(IP_Set {
            ipv6: v6,
            ipv4: v4
        })
    }

}
