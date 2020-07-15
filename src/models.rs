use serde::{Deserialize, Serialize};
use bson::{Document, doc};
use fll_scoring::errors::ServiceError;
use chrono::{DateTime,Utc};
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

impl From<Document> for User {
    fn from(doc: Document) -> Self  {
        let email = match doc.get_str("email") {
            Ok(eml) => eml.to_string(),
            Err(_) => "".to_string(),
        };
        let uid = match doc.get_str("uid") {
            Ok(id) => Uuid::parse_str(id).unwrap(),
            Err(_) => Uuid::new_v4(),
        };
        let pw_hash = match doc.get_str("pw_hash") {
            Ok(hash) => hash.to_string(),
            Err(_) => "".to_string(),
        };
        let created_at = match doc.get_datetime("created_at") {
            Ok(datetime) => datetime.clone(),
            Err(_) => chrono::Utc::now(),
        };

        User {
            uid,
            email,
            pw_hash,
            created_at: created_at.clone(),
        }
    }
}

impl From<User> for Document {
    fn from(user: User) -> Self {
        doc! {"uid": user.uid.to_string(), "email": user.email, "pw_hash": user.pw_hash, "created_at": user.created_at}        
    }
}
