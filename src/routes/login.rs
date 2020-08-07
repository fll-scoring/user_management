use sqlx::postgres::PgPool;
use crate::{models::{Invitation, SlimUser, User}, utils::verify};
use actix_web::{get, post, web, HttpResponse};
use actix_identity::Identity;
use serde::Deserialize;

use fll_scoring::errors::ServiceError;

#[derive(Deserialize)]
pub struct UserLogin {
  pub email: String,
  pub password: String,
}

#[post("/api/users/login")]
pub async fn login_user(form: web::Form<UserLogin>, id: Identity, pool: web::Data<PgPool>) -> Result<HttpResponse, ServiceError> {
  let result = match sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", form.email).fetch_one(pool.get_ref()).await {
    Ok(res) => res,
    Err(_) => {
      return Err(ServiceError::InternalServerError("Unable to query user database".to_string()));
    }
  };

  

  let is_pass_correct = verify(&form.password, &result.pw_hash)?;
  
  id.remember(result.userid.to_owned().to_string());

  if is_pass_correct {
  Ok(HttpResponse::Ok().body("success"))
  } else {
    Ok(HttpResponse::Ok().body("Wrong password"))
  }
}

#[get("/login")]
pub async fn login_page(tera: web::Data<tera::Tera>) -> Result<actix_web::HttpResponse, ServiceError> {
  let body = tera.render("login.html", &tera::Context::new()).map_err(|e| ServiceError::InternalServerError(format!("Template error {:?}", e.to_string())))?;

  Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
