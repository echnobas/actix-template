
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub id: i64
}

impl User {
    pub fn new(username: String, id: i64) -> Self {
        Self { username, id }
    }
}

impl From<bson::Document> for User {
    fn from(d: bson::Document) -> Self {
        Self { username: d.get("username").unwrap().to_string(), id: d.get("id").unwrap().as_i64().unwrap() }
    }
}

impl From<User> for bson::Document {
    fn from(u: User) -> bson::Document {
        doc! {
            "username": u.username,
            "id": u.id
        }
    }
}