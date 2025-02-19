//! Run with
//!
//! ```not_rust
//! cargo run --example cookie_session
//! ```
use poem::{
    get, handler,
    listener::TcpListener,
    middleware::CookieJarManager,
    web::cookie::{Cookie, CookieJar},
    EndpointExt, Route, Server,
};

#[handler]
async fn count(cookie_jar: &CookieJar) -> String {
    let count = match cookie_jar.get("count") {
        Some(cookie) => {
            let count = cookie.value::<i32>().unwrap() + 1;
            cookie_jar.add(Cookie::new("count", count));
            count
        }
        None => {
            cookie_jar.add(Cookie::new("count", 1));
            1
        }
    };

    format!("Hello!\nHow many times have seen you: {}", count)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let app = Route::new()
        .at("/", get(count))
        .with(CookieJarManager::new());
    let listener = TcpListener::bind("127.0.0.1:3000");
    let server = Server::new(listener).await?;
    server.run(app).await
}
