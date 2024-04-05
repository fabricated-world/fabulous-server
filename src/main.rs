mod database;

use actix_web::{web, App, HttpServer};
use database::get_database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = get_database("sqlite");
    database.init();

    HttpServer::new(|| {
        App::new()
            .service(web::redirect("/", "http://github.com/fabricated-world/fabulous-server"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
