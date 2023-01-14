mod adjective_words;

use regex::Regex;
use std::net::IpAddr;

use local_ip_address::local_ip;

fn main() {
    let pretty_ip = prettify_ip(get_local_ip());
    println!("{}", pretty_ip);

    let pretty_ip = prettify_ip_colors(get_local_ip());
    println!("{}", pretty_ip);

    //    let stdin = io::stdin();
    //    for line in stdin.lock().lines() {
    //        process_line(&adjs, &line.unwrap());
    //    }
}
fn prettify_ip(ip: IpAddr) -> String {
    let adjs: [&str; 256] = adjective_words::get_words();
    let re = Regex::new(r"\d+").unwrap();

    let output: String = ip.to_string();
    let output: &str = output.as_str();
    let output: Vec<&str> = output.split(".").collect();
    let output: Vec<&str> = output
        .iter()
        .map(|text| {
            let is_match = re.is_match(text);
            return if is_match {
                let index = text.parse::<usize>().ok().unwrap();
                let res: &str = adjs[index];
                res
            } else {
                text
            };
        })
        .collect::<Vec<&str>>();

    return output.join(".");
}

fn prettify_ip_colors(ip: IpAddr) -> String {
    let colors: [&str; 16] = adjective_words::get_colors();
    let re = Regex::new(r"\d+").unwrap();

    let output: String = ip.to_string();
    let output: &str = output.as_str();
    let output: Vec<&str> = output.split(".").collect();
    let output: Vec<String> = output
        .iter()
        .map(|text| {
            let is_match = re.is_match(text);
            return if is_match {
                let index = text.parse::<usize>().ok().unwrap();
                let r: String = format!("{}{}", colors[index / 16], colors[index % 16]);
                r
            } else {
                text.to_string()
            };
        })
        .collect::<Vec<String>>();

    return output.join(".");
}
fn get_local_ip() -> IpAddr {
    let my_local_ip = local_ip();
    if let Ok(my_local_ip) = my_local_ip {
        println!("Local IP address: {:?}", my_local_ip);
    } else {
        println!("Error getting local IP: {:?}", my_local_ip);
    }
    return my_local_ip.unwrap();
}
fn process_line(adjs: &[&str; 256], line: &str) {
    println!("{}", line);
}
