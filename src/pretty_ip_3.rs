use regex::Regex;

#[path = "util.rs"]
mod util;

pub fn invoke() -> String {
    let re = Regex::new(r"\d+").unwrap();

    util::prettify_ip(util::get_local_ip(), &|text: &str| {
        let is_match = re.is_match(text);
        return if is_match {
            let index = text.parse::<usize>().ok().unwrap();
            util::color_wrap_back(index / 16, util::color_wrap_front(index % 16, "▀").as_str())
        } else {
            text.to_string()
        };
    }).join("")
}
