use core::panic;
use std::net::SocketAddr;

use rand::distributions::{Alphanumeric, DistString};
use tokio::{net::TcpListener, process::Command};

pub async fn reset_db(db_suffix: &String) {
    match Command::new("sqlx")
        .arg("database")
        .arg("reset")
        .arg("-y")
        .arg("--database-url")
        .arg(format!(
            "postgres://postgres:root@localhost/postgres_test_{}",
            db_suffix
        ))
        .output()
        .await
    {
        Ok(_) => {
            println!("DATABASE RESET")
        }
        Err(_) => {
            panic!("Could not reset database using 'sqlx database reset -y'")
        }
    }
}

pub async fn create_listener() -> (SocketAddr, TcpListener) {
    let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    (addr, listener)
}

pub async fn spawn_server(listener: TcpListener, db_suffix: &String) {
    let mut db_uri = String::from("postgres://postgres:root@localhost/postgres_test_");
    db_uri.push_str(db_suffix);
    let db_pool = csveer_server::get_db_pool(db_uri).await.unwrap();
    tokio::spawn(async move {
        let app = csveer_server::build_app(db_pool.clone()).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });
}

pub async fn prepare_for_test() -> SocketAddr {
    let db_suffix: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    reset_db(&db_suffix).await;
    let (addr, listener) = create_listener().await;
    spawn_server(listener, &db_suffix).await;
    addr
}

pub async fn create_file_source(addr: &SocketAddr) {
    let client = reqwest::Client::new();
    let _ = client
        .post(format!("http://{}/source", addr))
        .header("Content-Type", "application/json")
        .body(include_str!(
            "../requests/file_source/create_file_source.json"
        ))
        .send()
        .await
        .expect("Failed to create file source for test");
}
