use std::{sync::Arc, fs, net::SocketAddr, convert::Infallible, borrow::Cow};
use clap::Parser;
use hyper::{Request, Body, Response, StatusCode, Method, body::Buf};
use indexmap::IndexMap;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::json;
use thiserror::Error;
use tokio::sync::RwLock;
use photon::{api, Dataset, ConfigError};

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

fn init_datasets(config_dir: &std::path::Path) -> IndexMap<String, Result<Dataset, ConfigError>> {
    fs::read_dir(config_dir).unwrap()
        .filter_map(|f| f.ok())
        .filter_map(|f| {
            if let Some(name) = f.file_name().to_str().and_then(|name| name.strip_suffix(".dataset.toml")) {
                let dataset = Dataset::from_config_file(f.path());
                
                if let Err(e) = &dataset {
                    eprintln!("Configuration error for dataset `{name}`: {e}")
                }

                Some((name.to_owned(), dataset))
            } else {
                None
            }
        })
        .collect()
}

type Datasets = Arc<RwLock<IndexMap<String, Result<Dataset, ConfigError>>>>;


#[tokio::main]
async fn main() {
    let args = Args::parse();

    let datasets: Datasets = Arc::new(RwLock::new(init_datasets(&args.config_dir)));

    let service = hyper::service::service_fn(move |req| {
        let datasets = datasets.clone();
        async move {
            Result::<_, Infallible>::Ok(match handle_request(&datasets, req).await {
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

async fn serve_static(path: &str) -> Result<Response<Body>, Error> {
    assert!(!path.contains("/"));
    let body: Option<Cow<'static, str>> = if let Some(manifest_dir) = std::env::var_os("CARGO_MANIFEST_DIR") {
        let path = std::path::Path::new(&manifest_dir).join("ui/www").join(path);
        tokio::fs::read_to_string(path).await.ok().map(|x| x.into())
    } else {
        match path {
            "index.html" => Some(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/ui/www/index.html")).into()),
            "style.css" => Some(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/ui/www/style.css")).into()),
            "app.js" => Some(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/ui/www/app.js")).into()),
            _ => None
        }
    };

    if let Some(body) = body {
        let content_type = 
            if path.ends_with(".html") { "text/html" }
            else if path.ends_with(".css") { "text/css" }
            else if path.ends_with(".js") { "application/javascript" }
            else { "application/octet-stream" };

        Ok(Response::builder()
            .status(StatusCode::OK)
            .header("content-type", content_type)
            .body(body.into())
            .unwrap())
    } else {
        Err(Error::NotFound)
    }
}

async fn handle_request(datasets: &Datasets, request: Request<Body>) -> Result<Response<Body>, Error> {
    let is_html = request.headers().get("accept")
        .and_then(|v| v.to_str().ok())
        .map_or(false, accepts_html);

    let path = request.uri().path().to_owned();
    let path_parts: Vec<_> = path.split("/").filter(|p| !p.is_empty()).collect();

    match (request.method(), &path_parts[..]) {
        (&Method::GET, &["_static", file]) => serve_static(file).await,
        (&Method::GET, _) if is_html => serve_static("index.html").await,
        (&Method::GET, &[]) => Ok(json_response(json!({
            "version": env!("CARGO_PKG_VERSION")
        }))),
        (_, &[dataset_name, ref subpath @ ..]) => {
            match datasets.read().await.get(dataset_name) {
                Some(Ok(dataset)) => handle_dataset_request(dataset, request, subpath).await,
                Some(Err(_)) => Err(Error::DatasetConfigError),
                None => Err(Error::NotFound)
            }
        }
        _ => Err(Error::NotFound)
    }
}

async fn handle_dataset_request(dataset: &Dataset, mut request: Request<Body>, path_parts: &[&str]) -> Result<Response<Body>, Error> {
    match (request.method(), path_parts) {
        (&Method::GET, &["_fields"]) => {
            Ok(json_response(dataset.fields()))
        }
        (&Method::POST, &["_query"]) => {
            let query = json_request::<api::query::Query>(&mut request).await?;
            let results = dataset.query(&query).map_err(Error::QueryError)?; 
            Ok(json_response(photon::api::query::Response { results }))
        }
        _ => Err(Error::NotFound)
    }
}

async fn json_request<V: DeserializeOwned>(req: &mut Request<Body>) -> Result<V, Error> {
    if req.headers().get("content-type").and_then(|v| v.to_str().ok()) == Some("application/json") {
        let whole_body = hyper::body::aggregate(req).await.unwrap();
        serde_json::from_reader(whole_body.reader()).map_err(Error::InvalidRequestBody)
    } else {
        Err(Error::RequestNotJson)
    }
}

fn json_response(v: impl Serialize) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(serde_json::to_vec(&v).unwrap().into())
        .unwrap()
}

fn accepts_html(v: &str) -> bool {
    v.split(",")
        .map(|v| v.trim().split_once(";").map_or(v, |v| v.0.trim()))
        .any(|v| v == "text/html")
}

#[test]
fn test_accept_header() {
    assert_eq!(accepts_html("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8"), true);
    assert_eq!(accepts_html("text/html ;q=1"), true);
    assert_eq!(accepts_html("application/json"), false)
}

#[derive(Debug, Error)]
enum Error {
    #[error("Not found")]
    NotFound,

    #[error("Expected JSON request body")]
    RequestNotJson,

    #[error("Invalid request body: {0}")]
    InvalidRequestBody(serde_json::Error),

    #[error("Dataset configuration could not be loaded")]
    DatasetConfigError,

    #[error("Query failed")]
    QueryError(photon::QueryError)
}

impl Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::DatasetConfigError => StatusCode::SERVICE_UNAVAILABLE,
            Error::RequestNotJson => StatusCode::BAD_REQUEST,
            Error::InvalidRequestBody(_) => StatusCode::BAD_REQUEST,
            Error::QueryError(_) => StatusCode::BAD_REQUEST,
            
        }
    }
    fn into_response(self) -> Response<Body> {
        let status = self.status_code();
        let message = format!("{self}");

        Response::builder()
            .status(status)
            .header("content-type", "application/json")
            .body(serde_json::to_vec(&serde_json::json!({"message": message})).unwrap().into())
            .unwrap()
    }
}
