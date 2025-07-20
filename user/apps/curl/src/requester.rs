use std::io::Read;
use std::io::Write;
use std::net::{SocketAddr, TcpStream};

// 同步函数实现 GET 请求
// pub fn get_request(ip: String, port: String) -> Result<String, Box<dyn Error>> {
//     let mut stream = TcpStream::connect(ip + port)?;

// }

enum Method {
    GET,
    POST,
}

pub struct request {
    data: Option<String>,
    header: Option<String>,
    url: url::Url,
}

impl request {
    pub fn new(_url: &url::Url) -> Self {
        request {
            data: None,
            header: None,
            url: _url.clone(),
        }
    }

    fn construct_header(&mut self, method: Method) {
        // 构造请求路径 (如果路径为空则使用 "/")
        let path = if self.url.path().is_empty() {
            "/"
        } else {
            self.url.path()
        };

        let host = self
            .url
            .host_str()
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "URL must have a host",
            ))
            .unwrap();

        // 构造查询字符串 (如果有的话)
        let query = self
            .url
            .query()
            .map(|q| format!("?{}", q))
            .unwrap_or_default();

        // data length
        let dl = self.data.clone().unwrap_or_default().len();

        match method {
            Method::GET => {
                self.header = Some(format!(
                    "GET {}{} HTTP/1.1\r\n\
                    Host: {}\r\n\
                    Content-Type: application/x-www-form-urlencoded\r\n\
                    Content-Length: {}\r\n\
                    Connection: close\r\n\r\n\
                    ",
                    path, query, host, dl
                ));
            }
            Method::POST => {
                self.header = Some(format!(
                    "POST {} HTTP/1.1\r\n\
                Host: {}\r\n\
                Content-Type: application/x-www-form-urlencoded\r\n\
                Content-Length: {}\r\n\
                Connection: close\r\n\r\n\
                ",
                    path, host, dl
                ));
            }
        }
    }

    pub fn set_header(&mut self, _h: &String) -> &mut Self {
        self.header = Some(_h.clone());
        self
    }
    pub fn set_data(&mut self, _d: &String) -> &mut Self {
        self.data = Some(_d.clone());
        self
    }

    pub fn get(&mut self, addrs: &Vec<SocketAddr>) -> Result<(), Box<dyn std::error::Error>> {
        let mut stream = TcpStream::connect(&addrs[..])?;

        // 构造完整的请求头
        self.construct_header(Method::GET);
        let request = &self.header.clone().unwrap_or_default();

        // 发送请求
        stream.write_all(request.as_bytes())?;

        // 创建一个缓冲区来接收数据
        // let mut buffer = String::new();

        // 读取服务器返回的数据
        // stream.read_to_string(&mut buffer)?;
        // Ok(buffer)
        //println!("buffer: {}", buffer);

        let mut buffer = [0; 1024];

        // 读取服务器返回的数据
        loop {
            match stream.read(&mut buffer) {
                Ok(n) => {
                    if n == 0 {
                        break;
                    };
                    println!("{}", String::from_utf8_lossy(&buffer[..n]));
                }
                Err(e) => {
                    // 与linux不一致
                    println!("与linux不一致: {:?}", e);
                    break;
                }
            }
        }

        Ok(())
    }

    pub fn post(&mut self, addrs: &Vec<SocketAddr>) -> Result<(), Box<dyn std::error::Error>> {
        let mut stream = TcpStream::connect(&addrs[..])?;

        // 构造完整的请求头
        self.construct_header(Method::POST);
        let request =
            self.header.clone().unwrap_or_default() + &self.data.clone().unwrap_or_default();
        println!("request: {}", request);

        // 发送请求
        stream.write_all(request.as_bytes())?;

        // // 创建一个缓冲区来接收数据
        // let mut buffer = String::new();

        // // 读取服务器返回的数据
        // stream.read_to_string(&mut buffer)?;
        // Ok(buffer)

        let mut buffer = [0; 1024];

        // 读取服务器返回的数据
        loop {
            match stream.read(&mut buffer) {
                Ok(n) => {
                    if n == 0 {
                        break;
                    };
                    println!("{}", String::from_utf8_lossy(&buffer[..n]));
                }
                Err(e) => {
                    // 与linux不一致
                    println!("与linux不一致: {:?}", e);
                    break;
                }
            }
        }

        Ok(())
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
