use std::net::IpAddr;

use regex::Regex;

use crate::util;

mod adjectives;
mod animals;

pub fn invoke(ip: IpAddr) -> String {
    let re = Regex::new(r"\d+").unwrap();
    let adjs: [&str; 256] = adjectives::get_words();
    let animals: [&str; 256] = animals::get_animals();

    util::prettify_ip_indexed(ip, &|current: usize, text: &str| {
        let is_match = re.is_match(text);
        return if is_match {
            let index = text.parse::<usize>().ok().unwrap();
            if current % 2 == 0 {
                util::color_wrap_front(index, adjs[index])
            } else {
                util::color_wrap_front(index, animals[index])
            }
        } else {
            text.to_string()
        };
    })
    .join(".")
}
