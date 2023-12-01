use crate::authentication::reject_anonymous_users;
use crate::configuration::{DatabaseSettings, Settings};
use actix_cors::Cors;
use actix_session::config::{CookieContentSecurity, PersistentSession};
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::time::Duration;
use actix_web::cookie::{Key, SameSite};
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

use crate::handlers::{
    change_password, create_user, get_daily_task_list, health_check, log_out, login,
};

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
        let listener = TcpListener::bind(address.clone())?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, connection_pool, configuration).await?;

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
    configuration: Settings,
) -> Result<Server, anyhow::Error> {
    let signing_key: Secret<String> = configuration.application.signing_key;
    let redis_uri: Secret<String> = configuration.redis_uri;
    let cookie_exp_in_secs: i64 = configuration.application.cookie_exp_in_secs;
    let cookie_secure: bool = configuration.application.cookie_secure;

    let db_pool = Data::new(db_pool);
    let base_url = Data::new(ApplicationBaseUrl(
        configuration.application.base_url.clone(),
    ));

    let secret_key = Key::from(signing_key.expose_secret().as_bytes());
    let redis_store = RedisSessionStore::new(redis_uri.expose_secret()).await?;

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&configuration.application.cors_origin)
            .supports_credentials()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(
                SessionMiddleware::builder(redis_store.clone(), secret_key.clone())
                    .cookie_domain(None)
                    .cookie_name("session_id".into())
                    .cookie_same_site(SameSite::Strict)
                    .cookie_secure(cookie_secure)
                    .cookie_content_security(CookieContentSecurity::Signed)
                    .session_lifecycle(
                        PersistentSession::default()
                            .session_ttl(Duration::seconds(cookie_exp_in_secs)),
                    )
                    .build(),
            )
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/api/auth/login", web::post().to(login))
            .route("/api/auth/createuser", web::post().to(create_user))
            .service(
                web::scope("/api")
                    .wrap(from_fn(reject_anonymous_users))
                    .route("/dailytasklist", web::get().to(get_daily_task_list))
                    .route("/auth/logout", web::post().to(log_out))
                    .route("/auth/changepassword", web::post().to(change_password)),
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
