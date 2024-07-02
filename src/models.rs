use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Serialize)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
}

impl From<Row> for User {
    fn from(value: Row) -> Self {
        Self {
            id: value.get("id"),
            first_name: value.get("first_name"),
            last_name: value.get("last_name"),
            username: value.get("username"),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
}
