fn main() {
    let url = String::from("127.0.0.1:8080");
    let url_slice = &url[10..];

    let string_borrow = &url;
    let string_literal = "1234";

    dbg!(&url);
    dbg!(url_slice);
    dbg!(string_borrow);
    dbg!(string_literal);

    // let server = Server::new(url);
    // server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self { addr }
    }

    fn run(self) {}
}
