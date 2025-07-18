#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <netdb.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>

#define BUFFER_SIZE 4096

// 结构体存储URL解析结果
typedef struct {
    char *protocol;
    char *hostname;
    int port;
    char *path;
} url_components;

// URL解析函数
int parse_url(const char *url, url_components *parsed) {
    char *p = (char *)url;
    char *host_start, *path_start;
    
    // 初始化为默认值
    parsed->protocol = "http";
    parsed->port = 80;  // 默认HTTP端口
    parsed->hostname = NULL;
    parsed->path = "/";
    
    // 检查协议
    if (strstr(url, "://")) {
        parsed->protocol = p;
        p = strstr(url, "://");
        *p = '\0';
        p += 3;
    }
    
    host_start = p;
    
    // 检查路径
    path_start = strchr(p, '/');
    if (path_start) {
        *path_start = '\0';
        parsed->path = path_start + 1;
        if (*parsed->path != '/') {
            // 确保路径以斜杠开头
            char temp_path[BUFFER_SIZE] = "/";
            strncat(temp_path, parsed->path, BUFFER_SIZE - 2);
            parsed->path = strdup(temp_path);
        }
    }
    
    // 检查端口号
    char *port_start = strchr(p, ':');
    if (port_start) {
        *port_start = '\0';
        port_start++;
        parsed->port = atoi(port_start);
    }
    
    parsed->hostname = strdup(host_start);
    
    return 0;
}

// DNS解析获取地址
struct addrinfo *resolve_host(const char *hostname, int port) {
    struct addrinfo hints, *result;
    char port_str[6];
    
    memset(&hints, 0, sizeof(struct addrinfo));
    hints.ai_family = AF_INET;       // 只使用IPv4简化处理
    hints.ai_socktype = SOCK_STREAM; // TCP
    
    snprintf(port_str, sizeof(port_str), "%d", port);
    
    int status = getaddrinfo(hostname, port_str, &hints, &result);
    
    if (status != 0) {
        fprintf(stderr, "DNS解析错误: %s\n", gai_strerror(status));
        return NULL;
    }
    
    return result;
}

// 创建TCP连接
int create_connection(struct addrinfo *addr) {
    // 遍历地址列表尝试连接
    for (struct addrinfo *p = addr; p != NULL; p = p->ai_next) {
        int sockfd = socket(p->ai_family, p->ai_socktype, p->ai_protocol);
        if (sockfd < 0) 
            continue;  // 创建socket失败，尝试下一个地址
        
        // 尝试连接
        if (connect(sockfd, p->ai_addr, p->ai_addrlen) == 0) {
            return sockfd; // 成功连接
        }
        
        close(sockfd);  // 连接失败，关闭socket继续尝试
    }
    
    return -1; // 所有地址都连接失败
}

// 发送HTTP GET请求
void send_http_get(int sockfd, url_components *url) {
    char request[BUFFER_SIZE];
    
    // 构造请求头
    snprintf(request, sizeof(request),
             "GET %s HTTP/1.1\r\n"
             "Host: %s\r\n"
             "User-Agent: mini-curl/1.0\r\n"
             "Connection: close\r\n\r\n", 
             url->path, url->hostname);
    
    // 发送请求
    if (send(sockfd, request, strlen(request), 0) < 0) {
        perror("请求发送失败");
        exit(EXIT_FAILURE);
    }
}

// 接收并打印响应
void receive_response(int sockfd) {
    char buffer[BUFFER_SIZE];
    ssize_t bytes_received;
    
    // 读取并打印所有响应数据
    while ((bytes_received = recv(sockfd, buffer, sizeof(buffer), 0)) > 0) {
        fwrite(buffer, 1, bytes_received, stdout);
    }
}

// 清理资源
void cleanup(int sockfd, url_components *url, struct addrinfo *addr) {
    if (sockfd != -1) close(sockfd);
    if (url->hostname != NULL) free(url->hostname);
    if (addr != NULL) freeaddrinfo(addr);
}

int main(int argc, char *argv[]) {
    if (argc != 2) {
        fprintf(stderr, "用法: %s <URL>\n", argv[0]);
        fprintf(stderr, "示例: %s http://example.com\n", argv[0]);
        return EXIT_FAILURE;
    }
    
    const char *url = argv[1];
    
    // 解析URL
    url_components parsed_url;
    if (parse_url(url, &parsed_url) != 0) {
        fprintf(stderr, "URL解析失败: %s\n", url);
        return EXIT_FAILURE;
    }
    
    // 检查协议是否支持
    if (strcmp(parsed_url.protocol, "http") != 0) {
        fprintf(stderr, "错误: 本程序只支持HTTP协议\n");
        free(parsed_url.hostname);
        return EXIT_FAILURE;
    }
    
    // DNS解析
    struct addrinfo *addr = resolve_host(parsed_url.hostname, parsed_url.port);
    if (addr == NULL) {
        free(parsed_url.hostname);
        return EXIT_FAILURE;
    }

    for(int i=0; i<14; i++)
    {
        printf("%d ", addr->ai_addr->sa_data[i]);
    }
    printf("\n");
    printf("len=%d\n", addr->ai_addrlen);
   
    // 建立TCP连接
    int sockfd = create_connection(addr);
    if (sockfd == -1) {
        fprintf(stderr, "连接失败: %s\n", parsed_url.hostname);
        freeaddrinfo(addr);
        free(parsed_url.hostname);
        return EXIT_FAILURE;
    }
    
    // 打印连接信息
    printf("连接到 %s (%s) 端口 %d\n", 
           parsed_url.hostname, 
           parsed_url.path, 
           parsed_url.port);
    
    // 发送HTTP请求
    send_http_get(sockfd, &parsed_url);
    
    // 接收并打印响应
    receive_response(sockfd);
    
    // 清理资源
    cleanup(sockfd, &parsed_url, addr);
    
    return EXIT_SUCCESS;
}