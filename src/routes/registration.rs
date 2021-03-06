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

use fll_scoring::prelude::*;

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
        "INSERT INTO users(userid,email,pw_hash) VALUES ($1, $2, $3)",
        uuid::Uuid::new_v4(),
        form.email,
        user.pw_hash
    )
    .execute(pool.get_ref())
    .await?;

    id.remember(user.userid.to_owned().to_string());

    Ok(HttpResponse::Ok().body("success"))
}

#[get("/register")]
pub async fn register_template(tera: web::Data<tera::Tera>, ctx: web::Data<tera::Context>) -> Result<HttpResponse, ServiceError> {
    let body = tera.render("register.html", &ctx).map_err(|e| ServiceError::InternalServerError(format!("Template error {:?}", e.to_string())))?;

    Ok(HttpResponse::Ok().body(body))
}
