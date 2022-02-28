use axum::{response::Html, routing::get, Router, Server};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
mod load_env;

use load_env::{ASyncEnvLoad, LoadEnv};

use crate::load_env::EnvConfig;

#[tokio::main]
async fn main() {
    let env_laoded = async move {
        let _y = LoadEnv::<EnvConfig>::load_to_env_from_file().await;
        LoadEnv::<EnvConfig>::load_env().await
    }
    .await
    .unwrap();

    let router = make_router();

    let service = router.into_make_service();

    /*
    run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    */

    // let x = load_env()?;

    let addrs = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        env_laoded.get_port(),
    );

    /*
    let addrs = "0.0.0.0:3000".parse::<SocketAddr>().unwrap();
    */

    let bound_server = Server::bind(&addrs);

    let server_handle = bound_server.serve(service);
    println!("listening on {addrs}");

    let x = server_handle.await;

    x.unwrap();
}

fn make_router() -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/test", get(handler2))
}

async fn handler() -> Html<&'static str> {
    Html("<a>Hello, World!")
}

async fn handler2() -> Html<&'static str> {
    Html("<h2>test</h2>")
}
