use actix_web::{web};

pub trait ProvideService: Send + Sync {
    fn configure_services(cfg: &mut web::ServiceConfig);
}