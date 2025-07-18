use clap::{Arg, Command};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use url::{Host, ParseError, Url};

mod parser;
mod requester;

fn main() {
    // 定义命令行界面
    let matches = Command::new("curl")
        .version("1.0")
        .author("Rene <2114678406@qq.com>")
        .about("A simple curl in rust")
        // 添加参数和选项
        .arg(Arg::new("url").help("Sets the url").required(true).index(1))
        .arg(
            Arg::new("method")
                .help("Set request method")
                .short('X')
                .long("request")
                .value_name("STRING")
                .default_value("GET"),
        )
        .arg(
            Arg::new("header")
                .help("Set request header")
                .short('H')
                .long("header")
                .value_name("STRING"),
        )
        .arg(
            Arg::new("data")
                .help("Set data for request")
                .short('d')
                .long("data")
                .value_name("STRING"),
        )
        .get_matches();

    let url_str = matches.get_one::<String>("url").unwrap();
    println!("url: {}", url_str);

    // 解析url
    if let Ok(url) = Url::parse(url_str) {
        println!("url parse successfully");

        match url.host() {
            Some(Host::Domain(domain)) => {
                println!("domain: {}", domain);
                // dns 解析
                match parser::resolve_domain(domain) {
                    Ok(ips) => {
                        println!("Resolved IP addresses for {}: {:?}", domain, ips);
                        assert!(!ips.is_empty(), "Should resolve to at least one IP");
                    }
                    Err(e) => panic!("Failed to resolve domain: {}", e),
                }
            }
            Some(Host::Ipv6(_)) => println!("not support ipv6"),
            _ => (),
        }

        // 默认为GET
        let method = matches.get_one::<String>("method").unwrap();
        println!("method: {}", method);

        // 发送get请求
        
    } else {
        println!("url pattern not correct!");
        return;
    }

    // if matches.get_flag("header") {
    //     println!("header: {}", header);
    // }

    if let Some(header) = matches.get_one::<String>("header") {
        println!("header: {}", header);
    }

    if let Some(data) = matches.get_one::<String>("data") {
        println!("data: {}", data);
    }
}
