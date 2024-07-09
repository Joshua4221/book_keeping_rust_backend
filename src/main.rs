use controllers::{Response, SuccessResponse};
use fairings::cors::{options, Cors};
use migrator::Migrator;
use rocket::http::Status;
use sea_orm_migration::MigratorTrait;

#[macro_use]
extern crate rocket;

mod auth;
mod controllers;
mod db;
mod entities;
mod fairings;
mod migrator;

pub struct AppConfig {
    db_host: String,
    db_port: String,
    db_username: String,
    db_password: String,
    db_database: String,
    jwt_secret: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            db_host: std::env::var("BOOKSTORE_DB_HOST").unwrap_or("localhost".to_string()),
            db_port: std::env::var("BOOKSTORE_DB_PORT").unwrap_or("3306".to_string()),
            db_username: std::env::var("BOOKSTORE_DB_USERNAME").unwrap_or("root".to_string()),
            //TODO: It is not a good practice to allow empty password. Give some default value.
            db_password: std::env::var("BOOKSTORE_DB_PASSWORD").unwrap_or("".to_string()),
            db_database: std::env::var("BOOKSTORE_DB_DATABASE")
                .unwrap_or("book_keeping".to_string()),
            jwt_secret: std::env::var("BOOKSTORE_JWT_SECRET")
                .expect("Please set the BOOKSTORE_JWT_SECRET env variable."),
        }
    }
}

//TODO: It does not give any functionality, please remove it.
#[get("/")]
fn index() -> Response<String> {
    Ok(SuccessResponse((Status::Ok, "Hello World".to_string())))
}

#[launch]
async fn rocket() -> _ {
    dotenvy::dotenv().ok();

    env_logger::init();

    let config = AppConfig::default();

    //TODO: It can panic but please handle the error properly. Avoid using unwrap.
    let db = db::connect(&config).await.unwrap();

    //TODO: It can panic but please handle the error properly. Avoid using unwrap.
    Migrator::up(&db, None).await.unwrap();

    rocket::build()
        .attach(Cors)
        .manage(db)
        .manage(config)
        .mount("/", routes![options])
        .mount("/", routes![index])
        .mount(
            "/auth",
            routes![
                controllers::auth::sign_in,
                controllers::auth::sign_up,
                controllers::auth::me
            ],
        )
        .mount(
            "/authors",
            routes![
                controllers::authors::index,
                controllers::authors::create,
                controllers::authors::show,
                controllers::authors::update,
                controllers::authors::delete,
                controllers::authors::get_books
            ],
        )
        .mount(
            "/books",
            routes![
                controllers::books::index,
                controllers::books::create,
                controllers::books::show,
                controllers::books::update,
                controllers::books::delete
            ],
        )
}
