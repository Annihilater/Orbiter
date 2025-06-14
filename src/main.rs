use actix_web::{web, App, HttpServer, middleware::Logger};
use sqlx::postgres::PgPoolOptions;
use std::io;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod config;
mod handlers;
mod middleware;
mod models;
mod utils;

use config::Config;
use middleware::Auth;
use handlers::ApiDoc;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(
        env_logger::Env::new()
            .default_filter_or("info")
            .default_write_style_or("always")
    );
    
    log::info!("Starting server...");
    
    let config = Config::from_env();
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to create pool");

    log::info!("Database connected, starting HTTP server...");
    
    let host = config.host.clone();
    let port = config.port;
    
    log::info!("Server running at: http://{}:{}", host, port);
    log::info!("API Base URL: http://{}:{}/api", host, port);
    log::info!("Swagger UI: http://{}:{}/swagger-ui/", host, port);
    log::info!("API Documentation: http://{}:{}/api-docs/openapi.json", host, port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new(
                "%a '%r' %s %b '%{Referer}i' '%{User-Agent}i' %T"
            ))
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(config.clone()))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/auth")
                            .route("/register", web::post().to(handlers::register))
                            .route("/login", web::post().to(handlers::login))
                    )
                    .service(
                        web::scope("/users")
                            .wrap(Auth)
                            .route("/me", web::get().to(handlers::me))
                    )
            )
    })
    .bind((host.as_str(), port))?
    .run()
    .await
} 