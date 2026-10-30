use std::{
    env,
    error::Error,
    io::ErrorKind,
    path::Path,
};

use axum::{
    body::Body,
    http::{
        header::{
            CONTENT_SECURITY_POLICY,
            REFERRER_POLICY,
            X_CONTENT_TYPE_OPTIONS,
        },
        HeaderName,
        HeaderValue,
        Request,
    },
    middleware::{self, Next},
    response::Response,
    Extension,
    Router,
};

use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[derive(Clone)]
struct SecurityHeaders {
    csp: Option<HeaderValue>,
    permissions_policy: Option<HeaderValue>,
}

async fn security_headers_middleware(
    req: Request<Body>,
    next: Next,
) -> Response {
    let security_headers = req
        .extensions()
        .get::<SecurityHeaders>()
        .cloned();

    let mut response = next.run(req).await;

    response.headers_mut().insert(
        X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );

    response.headers_mut().insert(
        REFERRER_POLICY,
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );

    if let Some(security_headers) = security_headers {
        if let Some(csp) = security_headers.csp {
            response
                .headers_mut()
                .insert(CONTENT_SECURITY_POLICY, csp);
        }

        if let Some(policy) = security_headers.permissions_policy {
            response.headers_mut().insert(
                HeaderName::from_static("permissions-policy"),
                policy,
            );
        }
    }

    response
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect(
            "Failed to register OS signal handler required for graceful shutdown",
        );
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load .env file if present
    match dotenvy::dotenv() {
        Ok(path) => println!("Loaded .env from {}", path.display()),
        Err(e) => println!("Failed to load .env: {e}"),
    }

    // Validate web root exists
    if !Path::new("./www").is_dir() {
        return Err("www directory does not exist".into());
    }

    let host = env::var("HOST")
        .unwrap_or_else(|_| "127.0.0.1".to_string());

    let port: u16 = match env::var("PORT") {
        Ok(port) => port
            .parse()
            .map_err(|_| "PORT must be a valid TCP port number")?,
        Err(_) => 3000,
    };

    let security_headers = SecurityHeaders {
        csp: env::var("HTTP_CSP")
            .ok()
            .map(|value| HeaderValue::from_str(&value))
            .transpose()?,
        
        permissions_policy: env::var("HTTP_PERMISSIONS_POLICY")
            .ok()
            .map(|value| HeaderValue::from_str(&value))
            .transpose()?,
    };

    let bind_address = format!("{host}:{port}");

    let app = Router::new()
        .fallback_service(ServeDir::new("./www"))
        .layer(middleware::from_fn(security_headers_middleware))
        .layer(Extension(security_headers));

    let listener = match TcpListener::bind(&bind_address).await {
        Ok(listener) => listener,

        Err(error) if error.kind() == ErrorKind::AddrInUse => {
            return Err(
                format!(
                    "Failed to start server. Port {port} is already in use on host {host}."
                )
                .into(),
            );
        }

        Err(error) => {
            return Err(
                format!(
                    "Failed to bind to {bind_address}: {error}"
                )
                .into(),
            );
        }
    };

    println!("Listening on http://{bind_address}");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    println!("Shutdown complete");

    Ok(())
}
