use std::fs::File;
use std::fs;
use std::error::Error;
use std::process::Command;

const SERVICE_PATH: &str = "/lib/systemd/system/ddns.service";

pub fn create_service(user: String) -> Result<(), Box<dyn Error>> {

    let service_def = format!("\
        [Unit] \n\
        Description=Update the cloudflare DNS configuration \n\
        Requires=network-online.target \n\
        After=network-online.target \n\
        \n\
        [Service] \n\
        User={} \n\
        ExecStart={} \"update\" \n\
        ", 
        user, 
        std::env::current_exe()?.to_str().expect("failed to get current exe path"));


    fs::write(SERVICE_PATH, service_def).expect(format!("Unable to write to file: {}", SERVICE_PATH).as_str());

    let output = Command::new("systemctl")
        .args(&[
            "activate",
            "ddns"
        ])  
        .output()
        .expect("failed to activate ddns.service");

    println!("activate: {:?}", String::from_utf8(output.stdout));

    Ok(())
}

pub fn remove_service() -> Result<(), Box<dyn Error>> {

    let output = Command::new("systemctl")
         .args(&[
             "deactivate",
             "ddns"
         ])
         .output()
         .expect("failed to deactivate ddns.service");

    println!("deactivate: {:?}", String::from_utf8(output.stdout));

    fs::remove_file(SERVICE_PATH)?;

    Ok(())
}

