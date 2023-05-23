use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use paas_dashboard::{checker, pulsar, util};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    util::init();
    checker::init();
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(actix_files::Files::new("/static", "portal").index_file("index.html"))
            .configure(config)
    })
        .bind(("0.0.0.0", 11111))?
        .run()
        .await
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/pulsar").configure(pulsar::pulsar_router));
}
