use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use paas_dashboard::util;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    util::init();
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(actix_files::Files::new("/", "static").index_file("index.html"))
            .configure(config)
    })
        .bind(("0.0.0.0", 11111))?
        .run()
        .await
}

fn config(_cfg: &mut web::ServiceConfig) {}
