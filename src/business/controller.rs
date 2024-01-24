use actix_web::{get, Responder};


#[get("/business/State{name}")]
pub fn get_business_state_by_business_id(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}!", name))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_business_state_by_business_id));
}