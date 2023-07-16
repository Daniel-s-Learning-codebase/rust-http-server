fn main() {
    let get = Method::GET("hello".to_string());
    let delete = Method::DELETE(293);
    let post = Method::POST;
    let put = Method::PUT;
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self { addr }
    }

    fn run(self) {
        println!("Listening on {}", self.addr);
    }
}

struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

enum Method {
    GET(String),
    DELETE(u64),
    POST,
    PUT = 5, // value can be assigned manually
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATH,
}
