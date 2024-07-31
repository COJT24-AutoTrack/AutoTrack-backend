mod db;
mod handlers;
mod middleware;
mod models;
mod routes;
mod state;

use axum;
use state::AppState;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing_subscriber::EnvFilter;
use tokio_rustls::rustls::{self, Certificate, PrivateKey};
use tokio_rustls::TlsAcceptor;
use tokio::net::TcpListener;
use std::fs::File;
use std::io::{self, BufReader};

#[tokio::main]
async fn main() {
    println!("Starting server...");

    let env_filter = EnvFilter::from_default_env();

    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    let db_pool = db::establish_connection().await;

    let state = Arc::new(Mutex::new(AppState {
        db_pool,
        firebase_project_id: env::var("FIREBASE_PROJECT_ID")
            .expect("FIREBASE_PROJECT_ID must be set"),
        require_email_verification: env::var("REQUIRE_EMAIL_VERIFICATION")
            .unwrap_or_else(|_| "true".to_string())
            .parse()
            .expect("REQUIRE_EMAIL_VERIFICATION must be a boolean"),
    }));

    let app = routes::create_routes(state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8369));
    
    // TLS証明書と秘密鍵の読み込み
    let certs = load_certs("/etc/ssl/certs/cert.pem").expect("Failed to load certificate");
    let key = load_private_key("/etc/ssl/private/key.pem").expect("Failed to load private key");
    let config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .expect("Failed to configure TLS");

    let tls_acceptor = TlsAcceptor::from(Arc::new(config));
    let listener = TcpListener::bind(&addr).await.unwrap();

    loop {
        let (stream, peer_addr) = listener.accept().await.unwrap();
        let acceptor = tls_acceptor.clone();
        tokio::spawn(async move {
            let tls_stream = acceptor.accept(stream).await.unwrap();
            axum::serve(tls_stream, app).await.unwrap();
        });
    }
}

// 証明書の読み込み関数
fn load_certs(path: &str) -> io::Result<Vec<Certificate>> {
    let certfile = File::open(path)?;
    let mut reader = BufReader::new(certfile);
    rustls_pemfile::certs(&mut reader)
        .map(|mut certs| certs.drain(..).map(Certificate).collect())
}

// 秘密鍵の読み込み関数
fn load_private_key(path: &str) -> io::Result<PrivateKey> {
    let keyfile = File::open(path)?;
    let mut reader = BufReader::new(keyfile);
    let keys = rustls_pemfile::pkcs8_private_keys(&mut reader)
        .map(|mut keys| keys.drain(..).map(PrivateKey).collect());
    
    keys.and_then(|keys| keys.into_iter().next().ok_or(io::Error::new(io::ErrorKind::InvalidData, "No private key found")))
}
