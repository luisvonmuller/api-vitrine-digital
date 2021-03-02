#[macro_use]
extern crate diesel;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpRequest, HttpServer, Responder};
use rand::Rng;

/* Diesel Imports */
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

/* Controllers */
pub mod arts;
pub mod clients;
pub mod demands;
pub mod login;
pub mod products;

/* Model structures */
pub mod models;

/* Schema */
pub mod schema;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

async fn hello(_req: HttpRequest) -> impl Responder {
    return String::from("Hello");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /* Logger */
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    /* Dot env checker */
    dotenv::dotenv().ok();

    /* Configures mysql enviroment */
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<MysqlConnection>::new(connspec);

    /* Create database pool */
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    /* Where we binds */
    let bind = "127.0.0.1:8080";
    println!("Server is running at: {}", bind);

    /* Cookie generation stuff */
    let private_key = rand::thread_rng().gen::<[u8; 32]>();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&private_key)
                    .name("auth-example")
                    .secure(false),
            ))
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(hello))
            .route("/login", web::get().to(crate::login::execute_login))
            .service(crate::clients::create::add_client)
            .service(crate::clients::read::get_client)
            .service(crate::clients::read::get_all)
            .service(crate::clients::delete::delete_client)
            .service(crate::clients::update::update_client)
    })
    .bind(&bind)?
    .run()
    .await
}
