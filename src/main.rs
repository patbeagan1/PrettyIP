mod formats;
mod util;

use std::net::{IpAddr, Ipv4Addr};

use clap::Parser;
use formats::{adj_animal, two_color, color_line, just_colored};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The format that we want to use
    #[arg(short, long, default_value_t = 1)]
    format: u8,

    /// Prints out all the formats at once
    #[arg(short, long, default_value_t = false)]
    all: bool,

    /// Manually entered ip address
    from: Option<String>,
}

fn main() {
    let args = Args::parse();

    let ip = match args.from {
        Some(it) => {
            let ip_str: &str = it.as_str();
            let ip_vals: Vec<u8> = ip_str
                .split(".")
                .into_iter()
                .map(|it| it.parse::<u8>().unwrap())
                .collect();
            IpAddr::V4(Ipv4Addr::new(
                ip_vals[0], ip_vals[1], ip_vals[2], ip_vals[3],
            ))
        }
        None => util::get_local_ip(),
    };

    if args.all {
        handle_arg_all(ip);
    } else {
        handle_arg_single_format(args.format, ip);
    }
}

fn handle_arg_single_format(format: u8, ip: IpAddr) {
    println!(
        "{}",
        match process(format, ip) {
            Ok(it) => it,
            Err(it) => it.to_string(),
        }
    );
}

fn handle_arg_all(ip: IpAddr) {
    for i in 1..=5 {
        println!("{}: {}", i, process(i, ip).unwrap())
    }
}

fn process(format: u8, ip: IpAddr) -> Result<String, &'static str> {
    match format {
        0 => Ok(ip.to_string()),
        1 => Ok(just_colored::invoke(ip)),
        2 => Ok(adj_animal::invoke(ip)),
        3 => Ok(two_color::invoke(ip)),
        4 => Ok(formats::color_dot::invoke(ip)),
        5 => Ok(color_line::invoke(ip)),
        // reminder to add new variation to handle_arg_all
        _ => Err("This format is not yet implemented"),
    }
}
