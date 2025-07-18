use std::net::IpAddr;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};

pub fn resolve_domain(domain: &str) -> Result<Vec<IpAddr>, Box<dyn std::error::Error>> {
    // 使用系统默认配置创建解析器
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default())?;
    
    // 执行DNS查询
    let response = resolver.lookup_ip(domain)?;
    
    // 收集所有IP地址
    let ips: Vec<IpAddr> = response.iter().collect();
    
    Ok(ips)
}

