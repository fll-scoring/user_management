use chrono::{DateTime, Utc};
use fll_scoring::errors::ServiceError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub uid: Uuid,
    pub email: String,
    pub pw_hash: String,
    pub created_at: chrono::DateTime<Utc>,
}

impl User {
    pub fn from_details<S: Into<String>, T: Into<String>>(email: S, pwd: T) -> Self {
        User {
            uid: Uuid::new_v4(),
            email: email.into(),
            pw_hash: pwd.into(),
            created_at: chrono::Utc::now(),
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
