use std::net::IpAddr;

use local_ip_address::local_ip;

pub fn prettify_ip(ip: IpAddr, on_each: &dyn Fn(&str) -> String) -> Vec<String> {
    let output: String = ip.to_string();
    let output: &str = output.as_str();
    let output: Vec<&str> = output.split(".").collect();
    let output: Vec<String> = output
        .iter()
        .map(|each| on_each(each))
        .collect::<Vec<String>>();
    return output;
}

pub fn get_local_ip() -> IpAddr {
    let my_local_ip = local_ip();
    if let Ok(my_local_ip) = my_local_ip {
        println!("Local IP address: {:?}", my_local_ip);
    } else {
        println!("Error getting local IP: {:?}", my_local_ip);
    }
    return my_local_ip.unwrap();
}

pub fn color_wrap_back(num: usize, content: &str) -> String {
    return format!("\u{001B}[48;5;{}m{}\u{001B}[0m", num, content);
}
pub fn color_wrap_front(num: usize, content: &str) -> String {
    return format!("\u{001B}[38;5;{}m{}\u{001B}[0m", num, content);
}
