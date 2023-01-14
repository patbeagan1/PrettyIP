mod adjective_words;

use regex::Regex;
use std::net::IpAddr;

use local_ip_address::local_ip;

fn main() {

    println!("{}", pretty_ip_1());
    println!("{}", pretty_ip_2());
}
fn pretty_ip_2() -> String {
    let re = Regex::new(r"\d+").unwrap();
    let adjs: [&str; 256] = adjective_words::get_words();

    prettify_ip(get_local_ip(), &|text: &str| {
        let is_match = re.is_match(text);
        return if is_match {
            let index = text.parse::<usize>().ok().unwrap();
            let res: &str = adjs[index];
            res.to_string()
        } else {
            text.to_string()
        };
    })
}

fn pretty_ip_1() -> String {
    let re = Regex::new(r"\d+").unwrap();
    let colors: [&str; 16] = adjective_words::get_colors();

    prettify_ip(get_local_ip(), &|text| {
        let is_match = re.is_match(text);
        return if is_match {
            let index = text.parse::<usize>().ok().unwrap();
            format!("{}{}", colors[index / 16], colors[index % 16])
        } else {
            text.to_string()
        };
    })
}
fn prettify_ip(ip: IpAddr, on_each: &dyn Fn(&str) -> String) -> String {
    let output: String = ip.to_string();
    let output: &str = output.as_str();
    let output: Vec<&str> = output.split(".").collect();
    let output: Vec<String> = output
        .iter()
        .map(|each| on_each(each))
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
