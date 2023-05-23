mod instances;

use actix_web::{HttpResponse, web};

pub fn pulsar_router(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/hello").route(web::get().to(hello)))
        .service(web::resource("/instances").route(web::get().to(instances::list_instances)))
    ;
}

async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello, Pulsar")
}
