use core::panic;
use std::net::SocketAddr;

use tokio::{net::TcpListener, process::Command};

pub async fn reset_db() {
    match Command::new("sqlx")
        .arg("database")
        .arg("reset")
        .arg("-y")
        .arg("--database-url")
        .arg("postgres://postgres:root@localhost/postgres_test")
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

pub async fn spawn_server(listener: TcpListener) {
    let db_uri = String::from("postgres://postgres:root@localhost/postgres_test");
    let db_pool = csveer_server::get_db_pool(db_uri).await.unwrap();
    tokio::spawn(async move {
        let app = csveer_server::build_app(db_pool.clone()).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });
}
