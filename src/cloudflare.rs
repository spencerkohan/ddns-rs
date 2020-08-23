//! This module is responsible for communicating with the Cloudflare api

use serde::{Deserialize, Serialize};
use std::vec::Vec;
use std::error::Error;
use ureq;
//use serde_json::Result;
use serde_json;

//mod crate::dns;
use crate::dns::*;

#[derive(Debug, Deserialize, Serialize)]
struct CFResponse<T> {
    success: bool,
    result: Vec<T>
}

#[derive(Debug, Deserialize, Serialize)]
struct DNS_Record {
    id: String,
    #[serde(rename = "type")]
    record_type: String,
    name: String,
    content: String
}



#[derive(Debug, Deserialize, Serialize)]
struct DNS_Record_Post {
    #[serde(rename = "type")]
    record_type: String,
    name: String,
    content: String,
    ttl: u32
}

#[derive(Debug, Deserialize, Serialize)]
struct DNS_Record_Patch {
    //#[serde(rename = "type")]
    //record_type: String,
    //name: String,
    content: String

}

fn get_dns_records(domain: &String, zone: &String, api_key: &String) -> Result<Vec<DNS_Record>, Box<dyn Error>> {

    let uri = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records?name={}", zone, domain);
    let response = ureq::get(uri.as_str())
        .set("Authorization", format!("Bearer {}", api_key).as_str())
        .call();

    //println!("status: {}", response.status());
    //println!("body: {:?}", &response.into_string());

    let data = response.into_string()?;
    let result: CFResponse<DNS_Record> = serde_json::from_str(data.as_str())?;

    println!("result: {:?}", result);

    return Ok(result.result);

}

fn update_dns_record(records: &Vec<DNS_Record>, domain: &String, zone: &String, api_key: &String, ip: &String, ipv: &IPV) -> Result<(), Box<dyn Error>> {

    let mut found: bool = false;
    for rec in records.iter() {
        if rec.record_type == ipv.dns_record_type() {

            println!("Updating DNS record: {:?}", rec);

            let body = serde_json::to_string(&DNS_Record_Patch { content: ip.clone() })?;

            let uri = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}", zone, rec.id);
            let response = ureq::patch(uri.as_str())
                .set("Authorization", format!("Bearer {}", api_key).as_str())
                .send_string(body.as_str());

            println!("response: {:?}", response);
            found = true;
        }
    }
    /// Create record if none found
    if !found {

        println!("no existing {} record found, creating a new one", ipv.dns_record_type());
        
        let body = serde_json::to_string(&DNS_Record_Post {
            record_type: ipv.dns_record_type().to_string(),
            name: domain.clone(),
            content: ip.clone(),
            ttl: 300
        })?;
        
        let uri = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records", zone);
        let response = ureq::post(uri.as_str())
            .set("Authorization", format!("Bearer {}", api_key).as_str())
            .send_string(body.as_str());

        println!("response: {:?}", response);
        println!("response data: {:?}", response.into_string());
    }

    Ok(())
}

pub fn update_dns(domain: String, zone: String, api_key: String, ips: &IP_Set) -> Result<(), Box<dyn Error>> {
    let records = get_dns_records(&domain, &zone, &api_key)?;

    if let Some(ipv4) = &ips.ipv4 {
        update_dns_record(&records, &domain, &zone, &api_key, &ipv4, &IPV::v4)?;
    }

    if let Some(ipv6) = &ips.ipv6 {
        update_dns_record(&records, &domain, &zone, &api_key, &ipv6, &IPV::v6)?;
    }

    Ok(())
}



