use std::net::IpAddr;

use regex::Regex;

use crate::util;

pub fn invoke(ip: IpAddr) -> String {
    let re = Regex::new(r"\d+").unwrap();

    util::prettify_ip(ip, &|text: &str| {
        let is_match = re.is_match(text);
        return if is_match {
            let index = text.parse::<usize>().ok().unwrap();
            util::color_wrap_back(index, " ")
        } else {
            text.to_string()
        };
    })
    .join("")
}
