#[macro_use]
extern crate rocket;
#[macro_use]
extern crate tracing;

mod config;
mod http;
mod telemetry;
mod utils;

use crate::http::catchers;
use crate::http::cors;
use crate::http::routes;
pub use config::Config;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::EncodingKey;
use rocket::figment::providers::Env;
use rocket::figment::providers::Serialized;
use rocket::{Build, Rocket, fairing};
use rocket_db_pools::{Database, sqlx::PgPool};
pub use telemetry::init_tracing;

#[derive(Database)]
#[database("main")]
pub(crate) struct Db(PgPool);

pub(crate) async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let Some(db) = Db::fetch(&rocket) else {
        return Err(rocket);
    };
    match sqlx::migrate!().run(&**db).await {
        Ok(_) => Ok(rocket),
        Err(e) => {
            error!("Failed to migrate database: {}", e);
            Err(rocket)
        }
    }
}

pub fn construct_rocket(config: Option<Config>) -> Rocket<Build> {
    let config = match config {
        Some(overrides) => rocket::Config::figment()
            .merge(Env::raw())
            .merge(Serialized::globals(overrides)),
        None => rocket::Config::figment().merge(Env::raw()),
    };
    let custom: Config = config.extract().expect("config");
    let config = config.merge(("databases.main.url", custom.database_url));

    let rocket = rocket::custom(config)
        .mount("/", routes![routes::healthz::health])
        .mount(
            "/api/user",
            routes![
                routes::users::read_current_user,
                routes::users::create_user,
                routes::users::update_current_user,
                routes::users::login,
            ],
        )
        .register("/", catchers![catchers::unauthorized])
        .manage(EncodingKey::from_base64_secret(&custom.secret_key).expect("valid base64"))
        .manage(DecodingKey::from_base64_secret(&custom.secret_key).expect("valid base64"))
        .attach(Db::init());

    let rocket = match custom.migrate {
        true => rocket.attach(fairing::AdHoc::try_on_ignite(
            "Run pending database migrations",
            run_migrations,
        )),
        false => rocket,
    };

    match custom.allowed_origins {
        Some(origins) => rocket.attach(cors::cors(&origins)),
        None => rocket,
    }
}
