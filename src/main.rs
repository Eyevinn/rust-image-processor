mod server;
mod services;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let img_server = server::Server::new(String::from("0.0.0.0"), 8080);
    let _ = img_server.init().await;
}
