use std::net::IpAddr;

use local_ip_address::local_ip;

pub fn prettify_ip(ip: IpAddr, on_each: &dyn Fn(&str) -> String) -> Vec<String> {
    return ip
        .to_string()
        .as_str()
        .split(".")
        .collect::<Vec<&str>>()
        .iter()
        .map(|each| on_each(each))
        .collect::<Vec<String>>();
}

pub fn prettify_ip_indexed(ip: IpAddr, on_each: &dyn Fn(usize, &str) -> String) -> Vec<String> {
    return ip
        .to_string()
        .as_str()
        .split(".")
        .collect::<Vec<&str>>()
        .iter()
        .enumerate()
        .map(|each| on_each(each.0, each.1))
        .collect::<Vec<String>>();
}

pub fn get_local_ip() -> IpAddr {
    let my_local_ip = local_ip();
    return my_local_ip.unwrap();
}

pub fn color_wrap_back(num: usize, content: &str) -> String {
    return format!("\u{001B}[48;5;{}m{}\u{001B}[0m", num, content);
}
pub fn color_wrap_front(num: usize, content: &str) -> String {
    return format!("\u{001B}[38;5;{}m{}\u{001B}[0m", num, content);
}
