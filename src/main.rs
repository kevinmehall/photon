use std::{sync::Arc, net::SocketAddr, convert::Infallible};
use clap::Parser;
use tokio::sync::RwLock;
use photon::{Config};

mod server;

/// Log analysis and visualization tool
#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    /// Directory containing configuration files
    #[clap(short, long)]
    config_dir: std::path::PathBuf,

    #[clap(short, long, default_value_t = SocketAddr::from(([127, 0, 0, 1], 3333)))]
    listen: SocketAddr,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let config = Arc::new(RwLock::new(Config::load(args.config_dir).unwrap()));

    let service = hyper::service::service_fn(move |req| {
        let config = config.clone();
        async move {
            Result::<_, Infallible>::Ok(match server::handle_request(&config, req).await {
                Ok(res) => res,
                Err(e) => e.into_response(),
            })
        }
    });

    let make_service = hyper::service::make_service_fn(move |_conn| {
        std::future::ready(Ok::<_, Infallible>(service.clone()))
    });

    hyper::Server::bind(&args.listen)
        .serve(make_service)
        .await
        .unwrap();
}
