use actix_web::{App, HttpServer, web};
use error_chain::error_chain;
use rbatis::Rbatis;
use rbdc_mysql::driver::MysqlDriver;
use tracing_subscriber::{filter, fmt, prelude::*, reload};

use crate::controller::{openapi, user_controller};

mod common;
mod controller;
mod model;

error_chain! {
    foreign_links {
        Io(std::io::Error);
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    let filter = filter::LevelFilter::DEBUG;
    let (filter, _reload_handle) = reload::Layer::new(filter);
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::Layer::default())
        .init();

    let db_username = "pig";
    let db_password = "123456";
    let db_host = "127.0.0.1";
    let db_port = 3306;
    let db_schema = "example";
    let db_url = format!("mysql://{}:{}@{}:{}/{}", db_username, db_password, db_host, db_port, db_schema);
    let rb = Rbatis::new();
    rb.init(MysqlDriver {}, &db_url)
        .unwrap();

    let state = common::AppState { pool: rb };

    let server_host = "127.0.0.1";
    let server_port = 9991;

    tracing::info!("Server is running on http://{}:{}", server_host, server_port);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(user_controller::register_routes)
            .configure(openapi::init)
    })
        .workers(5)
        .bind((server_host, server_port))?
        .run();

    _ = server.await;

    Ok(())
}

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(controller::api_routes());
}

