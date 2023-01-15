use std::net::IpAddr;

use regex::Regex;

#[path = "util.rs"]
mod util;

pub fn invoke(ip: IpAddr) -> String {
    let re = Regex::new(r"\d+").unwrap();
    let colors: [&str; 16] = get_colors();

    util::prettify_ip(ip, &|text| {
        let is_match = re.is_match(text);
        return if is_match {
            let index = text.parse::<usize>().ok().unwrap();
            format!("{}{}", colors[index / 16], colors[index % 16])
        } else {
            text.to_string()
        };
    }).join(":")
}

fn get_colors() -> [&'static str; 16] {
    return [
        "🔴", "🟠", "🟡", "🟢", "🔵", "🟣", "⚫️", "⚪️", "🟥", "🟧", "🟨", "🟩", "🟦", "🟪", "⬛️",
        "⬜️",
    ];
}
