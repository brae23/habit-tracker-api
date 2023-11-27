use crate::authentication::reject_anonymous_users;
use crate::configuration::{DatabaseSettings, Settings};
use actix_session::config::{PersistentSession, CookieContentSecurity};
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::{Key, SameSite};
use actix_web::cookie::time::Duration;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{
    web::{self},
    App, HttpServer,
};
use actix_web_lab::middleware::from_fn;
use secrecy::{ExposeSecret, Secret};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::handlers::{change_password, get_daily_task_list, health_check, log_out, login};

pub struct Application {
    port: u16,
    server: Server,
}

pub struct ApplicationBaseUrl(pub String);

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            connection_pool,
            configuration.application.base_url,
            configuration.application.hmac_secret,
            configuration.redis_uri,
            configuration.application.token_exp_in_secs,
        )
        .await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    base_url: String,
    hmac_secret: Secret<String>,
    redis_uri: Secret<String>,
    token_exp_in_secs: i64,
) -> Result<Server, anyhow::Error> {
    let db_pool = Data::new(db_pool);
    let base_url = Data::new(ApplicationBaseUrl(base_url));

    let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());
    let redis_store = RedisSessionStore::new(redis_uri.expose_secret()).await?;

    let server = HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::builder(redis_store.clone(), secret_key.clone())
                .cookie_same_site(SameSite::Strict)
                .cookie_content_security(CookieContentSecurity::Signed)
                .session_lifecycle(
                    PersistentSession::default().session_ttl(Duration::seconds(token_exp_in_secs))
                )
                .build())
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/auth/login", web::post().to(login))
            .service(
                web::scope("/auth")
                .wrap(from_fn(reject_anonymous_users))
                .route("/logout", web::post().to(log_out))
                .route("/changepassword", web::post().to(change_password))
            )
            .service(
                web::scope("/api")
                    .wrap(from_fn(reject_anonymous_users))
                    .route("/dailytasklist", web::get().to(get_daily_task_list))
            )
            .app_data(db_pool.clone())
            .app_data(base_url.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}