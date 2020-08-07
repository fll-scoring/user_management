use chrono::{DateTime, Utc};
use fll_scoring::errors::ServiceError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub userid: Uuid,
    pub email: String,
    pub pw_hash: String,
    pub admin: bool,
    pub is_verified: bool,
}

impl User {
    pub fn from_details<S: Into<String>, T: Into<String>>(email: S, pwd: T) -> Self {
        User {
            userid: Uuid::new_v4(),
            email: email.into(),
            pw_hash: pwd.into(),
            admin: false,
            is_verified: false
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Invitation {
    pub id: uuid::Uuid,
    pub email: String,
    pub expires_at: chrono::NaiveDateTime,
}

impl<T> From<T> for Invitation
where
    T: Into<String>,
{
    fn from(email: T) -> Self {
        Invitation {
            id: uuid::Uuid::new_v4(),
            email: email.into(),
            expires_at: chrono::Local::now().naive_local() + chrono::Duration::hours(24),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SlimUser {
    pub email: String,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser { email: user.email }
    }
}
