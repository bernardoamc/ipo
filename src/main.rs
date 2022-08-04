use clap::Parser;
use std::net::IpAddr;

mod ip;
use ip::{V4, V6};

/// IPO
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   /// The IP address to be obfuscated in ipv4 or ipv6 format.
   #[clap(short, long, value_parser)]
   ip: String,
}
fn main() {
    let args = Args::parse();
    let ip: IpAddr = args.ip
        .parse::<IpAddr>()
        .expect("Please enter a valid IP address");

    match ip {
        IpAddr::V4(ipv4) => {
            let v4 = V4::new(ipv4.octets().to_vec());

            print_banner("decimal");
            println!("|{:^38}|", v4.to_decimal());

            print_banner("octal");
            println!("|{:^38}|", v4.to_octal());

            print_banner("hexadecimal");
            println!("|{:^38}|", v4.to_hex());

            print_banner("ipv6");
            println!("|{:^38}|" ,v4.to_ipv6());

            print_banner("permutations");
            for permutation in v4.permutations() {
                println!("|{:^38}|", permutation);
            }
            println!("{:=<40}", "=");
        },
        IpAddr::V6(ipv6) => {
            let v6 = V6 { octets: ipv6.octets().to_vec() };
            println!("{}", v6.to_decimal());
        }
    };
}

fn print_banner(text: &str) {
    println!("{:=<40}", "=");
    println!("|{:^38}|", text.to_uppercase());
    println!("{:=<40}", "=");
}
