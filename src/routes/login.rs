use sqlx::postgres::PgPool;
use crate::{models::{Invitation, SlimUser, User}, utils::verify};
use actix_web::{get, post, web, HttpResponse};
use actix_identity::Identity;
use serde::Deserialize;
use askama_actix::{Template, TemplateIntoResponse};
use fll_scoring::errors::ServiceError;

#[derive(Deserialize)]
struct UserLogin {
  pub email: String,
  pub password: String,
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {}

#[post("/api/users/login")]
pub async fn login_user(form: web::Form<UserLogin>, id: Identity, pool: web::Data<PgPool>) -> Result<HttpResponse, ServiceError> {
  let result = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", form.email).fetch_one(pool.get_ref()).await?; 

  let is_pass_correct = verify(form.password, result.pw_hash)?;

  if is_pass_correct {
  Ok(HttpResponse::Ok().body("success"))
  } else {
    Ok(HttpResponse::Ok().body("Wrong password"))
  }
}

#[get("/login")]
pub async fn login_page() -> Result<HttpResponse, ServiceError> {
  LoginTemplate.into_response()
}
