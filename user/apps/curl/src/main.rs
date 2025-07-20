use clap::{Arg, Command};

use std::net::IpAddr;
use std::net::SocketAddr;
use url::{Host, Url};

mod parser;
mod requester;

// url_str -> [ip:port, ...]
fn to_adders(url: &Url) -> Result<Vec<SocketAddr>, Box<dyn std::error::Error>> {
    let mut res = vec![];

    match url.host() {
        Some(Host::Domain(domain)) => {
            println!("domain: {}", domain);
            // dns 解析
            match parser::resolve_domain(domain) {
                Ok(ips) => {
                    println!("Resolved IP addresses for {}: {:?}", domain, ips);
                    assert!(!ips.is_empty(), "Should resolve to at least one IP");

                    let port;
                    if let Some(p) = url.port() {
                        port = p;
                    } else {
                        port = 80;
                    }
                    for ipi in &ips {
                        res.push(SocketAddr::new(*ipi, port));
                    }
                    return Ok(res);
                }
                Err(e) => return Err(format!("Failed to resolve domain: {}", e).into()),
            }
        }
        Some(Host::Ipv6(_)) => return Err("not support ipv6".into()),
        // 明文ip
        Some(Host::Ipv4(ips)) => {
            let port;
            if let Some(p) = url.port() {
                port = p;
            } else {
                port = 80;
            }
            res.push(SocketAddr::new(IpAddr::V4(ips), port));
            return Ok(res);
        }
        None => return Err("miss host name".into()),
    }
}

fn main() {
    // 定义命令行界面
    let matches = Command::new("curl")
        .version("1.0")
        .author("Rene <2114678406@qq.com>")
        .about("A simple curl in rust")
        // 添加参数和选项
        .arg(
            Arg::new("method")
                .help("Set request method")
                .short('X')
                .long("request")
                .value_name("STRING")
                .num_args(1)
                .default_value("GET"),
        )
        .arg(
            Arg::new("header")
                .help("Set request header")
                .short('H')
                .long("header")
                .num_args(1)
                .value_name("STRING"),
        )
        .arg(
            Arg::new("data")
                .help("Set data for request")
                .short('d')
                .long("data")
                .num_args(1)
                .value_name("STRING"),
        )
        .arg(Arg::new("url").help("Sets the url").required(true))
        .get_matches();

    let url_str = matches.get_one::<String>("url").unwrap();

    let url = Url::parse(url_str).unwrap();

    let addrs = to_adders(&url).unwrap();

    let mut request = requester::request::new(&url);

    if let Some(header) = matches.get_one::<String>("header") {
        request.set_header(&header);
    }

    if let Some(data) = matches.get_one::<String>("data") {
        request.set_data(&data);
    }

    // 默认为GET
    let method = matches.get_one::<String>("method").unwrap();

    if method == "GET" {
        // 使用默认header进行get
        request.get(&addrs).unwrap();
    } else if method == "POST" {
        // 使用默认header进行get
        request.post(&addrs).unwrap();
    }

    
}
