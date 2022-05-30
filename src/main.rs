use clap::Parser;
use photon::Config;
use std::{convert::Infallible, net::SocketAddr, sync::Arc, io};
use tokio::sync::RwLock;

mod server;

/// Log analysis and visualization tool
#[derive(Parser, Debug)]
#[clap(version, about)]
enum Args {
    Serve {
        /// Directory containing configuration files
        #[clap(short, long)]
        config_dir: std::path::PathBuf,

        #[clap(short, long, default_value_t = SocketAddr::from(([127, 0, 0, 1], 3333)))]
        listen: SocketAddr,
    },
    Query {
        /// Directory containing configuration files
        #[clap(short, long)]
        config_dir: std::path::PathBuf,

        #[clap(short, long)]
        dataset: String,

        #[clap(short, long)]
        query: String,
    },
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args {
        Args::Serve { config_dir, listen } => {
            let config = Arc::new(RwLock::new(Config::load(config_dir).unwrap()));
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

            hyper::Server::bind(&listen)
                .serve(make_service)
                .await
                .unwrap();
        }
        Args::Query {
            config_dir,
            dataset,
            query,
        } => {
            let config = Config::load(config_dir).unwrap();
            let dataset = config.dataset(&dataset).expect("dataset does not exist").expect("config error");

            let query = serde_json::from_str(&query).expect("failed to parse query");
            let results = dataset.query(&query).expect("failed to run query");

            serde_json::to_writer(io::stdout().lock(), &results).expect("failed to write output");
        }
    }
}
