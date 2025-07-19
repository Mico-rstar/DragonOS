use std::io::Read;
use std::io::Write;
use std::net::{SocketAddr, TcpStream};

// 同步函数实现 GET 请求
// pub fn get_request(ip: String, port: String) -> Result<String, Box<dyn Error>> {
//     let mut stream = TcpStream::connect(ip + port)?;

// }

pub struct request  {
    header: String,
}

impl request {
    pub fn new(url: &url::Url) -> Self {
        // 构造请求路径 (如果路径为空则使用 "/")
        let path = if url.path().is_empty() {
            "/"
        } else {
            url.path()
        };

        let host = url.host_str().ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "URL must have a host",
        )).unwrap();

        // 构造查询字符串 (如果有的话)
        let query = url.query().map(|q| format!("?{}", q)).unwrap_or_default();

        request {
            header: format!(
                "GET {}{} HTTP/1.1\r\n\
         Host: {}\r\n\
         User-Agent: mini-curl/1.0\r\n\
         Connection: close\r\n\r\n",
                path, query, host
            ),
        }
    }

    pub fn set_header(&mut self, _h: &String) -> &mut Self {
        self.header = _h.clone();
        self
    }

    pub fn get(
        &self,
        addrs: &Vec<SocketAddr>
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut stream = TcpStream::connect(&addrs[..])?;

        println!("Connected to the server!");

        // 构造完整的请求头
        let request = &self.header;

        // 发送请求
        stream.write_all(request.as_bytes())?;

        // 创建一个缓冲区来接收数据
        let mut buffer = String::new();

        // 读取服务器返回的数据
        stream.read_to_string(&mut buffer)?;
        Ok(buffer)
    }

    pub fn extract_body(response: &str) -> String {
        // 查找第一个空行（分隔头部和body）
        if let Some(empty_line_pos) = response.find("\r\n\r\n") {
            // 跳过空行后的部分就是body
            String::from(&response[empty_line_pos + 4..])
        } else if let Some(empty_line_pos) = response.find("\n\n") {
            // 有些响应可能只用单个换行符
            String::from(&response[empty_line_pos + 2..])
        } else {
            String::new()
        }
    }
}
