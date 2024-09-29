use actix_web::{get, middleware, web, App, HttpServer, Responder};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let port = get_port();
    let address: &str = &get_address();
    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
            .service(is_prime_endpoint)
    })
    .bind((address, port))?
    .run();
    server.await
}

fn get_port() -> u16 {
    let port_string = env::var("PORT").unwrap_or("8080".to_string());
    port_string.parse().expect("PORT variable must be a number")
}

fn get_address() -> String {
    env::var("ADDRESS").unwrap_or("0.0.0.0".to_string())
}

#[get("/api/isprime/{number}")]
async fn is_prime_endpoint(num_string: web::Path<String>) -> impl Responder {
    let any_num: i32 = match num_string.parse() {
        Ok(any_num) => any_num,
        Err(_) => return r#"{"error": "This operation isn't prime for success"}"#.to_owned(),
    };
    let num = match u32::try_from(any_num) {
        Ok(num) => num,
        Err(_) => {
            return r#"{"error": "Negative number. Please upgrade to isPrime API Premium or Enterprise for support"}"#.to_owned()
        }
    };
    let is_prime = is_prime(num);
    format!("{{\"isprime\": {is_prime}}}")
}

/// Test whether a number is prime. Checks every odd number up to `sqrt(n)`.
fn is_prime(x: u32) -> bool {
    // If even, it is not prime
    if x % 2 == 0 {
        return false;
    };
    // Check every odd number up until the square root i.e. the square exceeds x
    for n in (3..).step_by(2).take_while(|m| m * m <= x) {
        if x % n == 0 {
            return false;
        };
    }
    // No factor found. It must be prime.
    true
}
