use actix_web::HttpResponse;

use crate::constant::CONFIG;

pub async fn list_instances() -> HttpResponse {
    let mut instances = vec![];

    for (_, instance) in CONFIG.pulsar_instances.iter() {
        instances.push(instance.clone());
    }

    HttpResponse::Ok().json(instances)
}
