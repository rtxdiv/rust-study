#[allow(unused)]
#[derive(Debug)]
#[derive(Clone)]
struct ApiResponseError {
    code: u32,
    message: String
}

enum ApiResponse {
    Success(String),
    Error(ApiResponseError)
}

impl ApiResponse {
    fn is_success(&self) -> bool {
        matches!(&self, ApiResponse::Success(_))
    }
    fn ok(&self) -> Option<String> {
        if let ApiResponse::Success(body) = self {
            Some(body.clone())
        } else {
            None
        }
    }
    fn error(&self) -> Option<ApiResponseError> {
        if let ApiResponse::Error(body) = self {
            Some(body.clone())
        } else {
            None
        }
    }
    fn print_info(&self) {
        println!("is_success: {:?}", self.is_success());
        println!("ok: {:?}", self.ok());
        let ok_body = self.ok().map(|s| s.to_uppercase()).unwrap_or(String::from("EMPTY"));
        println!("преобразование ok_body: {}", ok_body);
        println!("error: {:?}", self.error());
        println!();
    }
}


fn parse_port(port_str: &str) -> Option<u16> {
    port_str.trim().parse::<u16>().ok()
}

fn main() {
    // TASK 1
    let resp1 = ApiResponse::Success(String::from("success"));
    println!("resp1");
    resp1.print_info();

    let resp2 = ApiResponse::Error(ApiResponseError { code: (500), message: String::from("server error")});
    println!("resp2");
    resp2.print_info();

    // TASK 2
    println!("Порт 8080: {:?}", parse_port("8080").unwrap_or(80));
    println!("Порт invalid: {:?}", parse_port("invalid").unwrap_or(80));
}
