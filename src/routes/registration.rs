use crate::{
    models::{Invitation, SlimUser, User},
    utils::hash_password,
};
use actix_web::{error::BlockingError, get, post, web, HttpResponse};
use fll_scoring::errors::ServiceError;
use serde::Deserialize;

use actix_identity::Identity;
use handlebars::Handlebars;
use sqlx::postgres::PgPool;

#[derive(Deserialize)]
pub struct RegisteredUser {
    pub email: String,
    pub password: String,
}

#[post("/api/users/register")]
pub async fn register_user(
    form: web::Form<RegisteredUser>,
    id: Identity,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ServiceError> {
    let hashed_password = hash_password(&form.password)?;
    let user: User = User::from_details(form.email.clone(), hashed_password);

    sqlx::query!(
        "INSERT INTO users(userid,email,username,pw_hash) VALUES (?1, ?2, ?3, ?4)",
        Uuid::new_v4(),
        form.email,
        hashed_password
    )
    .execute(&pool)
    .await?;

    id.remember(user.uid.to_owned().to_string());

    Ok(HttpResponse::Ok().body("success"))
}

#[get("/register")]
pub async fn register_template(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let body = hb.render("register", &json!({})).unwrap();

    HttpResponse::Ok().body(body)
}
