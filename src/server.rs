use std::{sync::Arc, borrow::Cow};
use tokio::sync::RwLock;
use hyper::{Request, Body, Response, StatusCode, Method, body::Buf};
use serde::{Serialize, de::DeserializeOwned};
use thiserror::Error;
use photon::{api, Dataset, Config};

pub async fn handle_request(config: &Arc<RwLock<Config>>, request: Request<Body>) -> Result<Response<Body>, Error> {
    let is_html = request.headers().get("accept")
        .and_then(|v| v.to_str().ok())
        .map_or(false, accepts_html);

    let path = request.uri().path().to_owned();
    let path_parts: Vec<_> = path.split("/").filter(|p| !p.is_empty()).collect();

    match (request.method(), &path_parts[..]) {
        (&Method::GET, &["_static", file]) => serve_static(file).await,
        (&Method::GET, _) if is_html => serve_static("index.html").await,
        (&Method::GET, &[]) => {
            let conf = config.read().await;
            
            let ds_response = conf.datasets().map(|(name, d)| {
                (name.to_owned(), api::root::Dataset { ok: d.is_ok() })
            }).collect();

            Ok(json_response(api::root::RootResponse {
                datasets: ds_response,
                version: env!("CARGO_PKG_VERSION")
            }))
        }
        (_, &[dataset_name, ref subpath @ ..]) => {
            match config.read().await.dataset(dataset_name) {
                Some(Ok(dataset)) => handle_dataset_request(dataset, request, subpath).await,
                Some(Err(e)) => Err(Error::DatasetConfigError(e.to_string())),
                None => Err(Error::DatasetNotFound)
            }
        }
        _ => Err(Error::InvalidRoute)
    }
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
        Err(Error::InvalidRoute)
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
        _ => Err(Error::InvalidRoute)
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
pub enum Error {
    #[error("Invalid route")]
    InvalidRoute,

    #[error("Dataset not found")]
    DatasetNotFound,

    #[error("Expected JSON request body")]
    RequestNotJson,

    #[error("Invalid request body: {0}")]
    InvalidRequestBody(serde_json::Error),

    #[error("Dataset configuration could not be loaded")]
    DatasetConfigError(String),

    #[error("Query failed")]
    QueryError(photon::QueryError)
}

impl Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::InvalidRoute => StatusCode::NOT_FOUND,
            Error::DatasetNotFound => StatusCode::NOT_FOUND,
            Error::DatasetConfigError(_) => StatusCode::SERVICE_UNAVAILABLE,
            Error::RequestNotJson => StatusCode::BAD_REQUEST,
            Error::InvalidRequestBody(_) => StatusCode::BAD_REQUEST,
            Error::QueryError(_) => StatusCode::BAD_REQUEST,
            
        }
    }

    fn error_code(&self) -> &'static str {
        match self {
            Error::InvalidRoute => "invalid_route",
            Error::DatasetNotFound => "dataset_not_found",
            Error::RequestNotJson => "invalid_request_json",
            Error::InvalidRequestBody(_) => "invalid_request",
            Error::DatasetConfigError(_) => "config_error",
            Error::QueryError(_) => "query_failed",
        }
    }

    fn detail(&self) -> Option<String> {
        match self {
            Error::DatasetConfigError(e) => Some(e.to_string()),
            _ => None,
        }
    }

    pub fn into_response(self) -> Response<Body> {
        let status = self.status_code();

        Response::builder()
            .status(status)
            .header("content-type", "application/json")
            .body(serde_json::to_vec(&serde_json::json!({
                "code": self.error_code(),
                "message": format!("{self}"),
                "detail": self.detail(),
            })).unwrap().into())
            .unwrap()
    }
}
