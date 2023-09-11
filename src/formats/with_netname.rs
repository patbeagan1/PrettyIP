use std::net::IpAddr;

pub fn invoke(ip: IpAddr) -> String {
    let ip_with_names = ip
        .to_string()
        .replace("192.168", "LAN")
        .replace("10.0.0", "Private Network")
        .replace("127.0.0.1", "Localhost")
        .replace("0.0.0.0", "Localhost")
        .replace("127.0.0", "Localhost");

    let split_ip_names: Vec<&str> = ip_with_names.split(".").collect();

    split_ip_names.iter().map(|text| {
        return text.to_string()
    }).collect::<Vec<String>>().join(".")
}
