#[macro_use]
extern crate serde_json;

use actix_web::{HttpServer, App, web};
use handlebars::Handlebars;
use fll_scoring::config::{get_service_config_value, get_global_value};
use actix_identity::{Identity, CookieIdentityPolicy,IdentityService};

pub mod models;
pub mod routes;
pub mod utils;

use routes::{registration::*};


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./templates")
        .unwrap();
    let hb_ref = web::Data::new(handlebars);

    let pool = fll_scoring::data::get_db_pool().await.unwrap();
    let pool_ref = web::Data::new(pool);

    let cookie_secret_key = get_global_value("secret_key", true).unwrap();
    let domain = match get_global_value("base-domain", false) {
        Ok(dom) => dom,
        Err(_) => String::from("localhost")
    };

    let bind_addr = match get_service_config_value("user_management", "bind-addr", false) {
        Ok(addr) => addr,
        Err(_) => String::from("127.0.0.1:8001"),
    };
 

    HttpServer::new(move || {
        App::new()
            .app_data(hb_ref.clone())
            .app_data(pool_ref.clone())
            .service(register_user)
            .service(register_template)
            .wrap(IdentityService::new(
                    CookieIdentityPolicy::new(&cookie_secret_key.as_bytes())
                        .name("fll-scoring-auth")
                        .domain(domain.clone())
                        .secure(false)
            ))
    })
    .bind(bind_addr)?
    .run()
    .await
}
