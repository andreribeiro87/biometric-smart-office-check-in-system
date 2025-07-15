use crate::schema::users;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rpa::Rpa;

#[diesel(table_name = users)]
#[connection_type = "POSTGRESQL"]
#[derive(
    AsChangeset,
    Serialize,
    Deserialize,
    Queryable,
    QueryableByName,
    Insertable,
    Identifiable,
    TypeInfo,
    Debug,
    Clone,
    Rpa
)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub fingerprint: Vec<u8>,
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub fingerprint: Vec<u8>,
}

#[derive(Serialize, Deserialize, AsChangeset, Debug)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub fingerprint: Option<Vec<u8>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<NewUser> for User {
    fn from(new_user: NewUser) -> Self {
        let now = Utc::now();
        User {
            id: new_user.id,
            name: new_user.name,
            email: new_user.email,
            created_at: now,
            updated_at: now,
            fingerprint: new_user.fingerprint,
        }
    }
}
