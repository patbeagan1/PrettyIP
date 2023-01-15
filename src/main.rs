mod pretty_ip_1;
mod pretty_ip_2;
mod pretty_ip_3;
mod pretty_ip_4;
mod pretty_ip_5;
mod util;

use std::net::{IpAddr, Ipv4Addr};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The format that we want to use
    #[arg(short, long, default_value_t = 4)]
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
        for i in 1..=5 {
            println!("{}: {}", i, process(i, ip).unwrap())
        }
    } else {
        println!(
            "{}",
            match process(args.format, ip) {
                Ok(it) => it,
                Err(it) => it.to_string(),
            }
        );
    }
}

fn process(format: u8, ip: IpAddr) -> Result<String, &'static str> {
    match format {
        0 => Ok(ip.to_string()),
        1 => Ok(pretty_ip_1::invoke(ip)),
        2 => Ok(pretty_ip_2::invoke(ip)),
        3 => Ok(pretty_ip_3::invoke(ip)),
        4 => Ok(pretty_ip_4::invoke(ip)),
        5 => Ok(pretty_ip_5::invoke(ip)),
        _ => Err("This format is not yet implemented"),
    }
}
