use std::error::Error;
use std::net::{SocketAddr, TcpStream};
use std::io::Read;
use std::io::Write;

// 同步函数实现 GET 请求
// pub fn get_request(ip: String, port: String) -> Result<String, Box<dyn Error>> {
//     let mut stream = TcpStream::connect(ip + port)?;

// }

pub fn get(addrs: &Vec<SocketAddr>, url: &url::Url) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(&addrs[..])?;

    println!("Connected to the server!");

    // 构造请求路径 (如果路径为空则使用 "/")
    let path = if url.path().is_empty() {
        "/"
    } else {
        url.path()
    };

    let host = url.host_str().ok_or(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        "URL must have a host",
    ))?;
    
    // 构造查询字符串 (如果有的话)
    let query = url.query().map(|q| format!("?{}", q)).unwrap_or_default();
    
    // 构造完整的请求头
    let request = format!(
        "GET {}{} HTTP/1.1\r\n\
         Host: {}\r\n\
         User-Agent: mini-curl/1.0\r\n\
         Connection: close\r\n\r\n",
        path, query, host
    );
    
    // 发送请求
    stream.write_all(request.as_bytes())?;

    // 创建一个缓冲区来接收数据
    let mut buffer = [0; 1024];

    // 读取服务器返回的数据
    if let Ok(bytes_read) = stream.read(&mut buffer) {
        println!(
            "Received: {}",
            String::from_utf8_lossy(&buffer[..bytes_read])
        );
    }
    Ok(())
}
