use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chat {
    #[serde(rename = "_id", skip_serializing)]
    pub id: Option<ObjectId>,
    pub user: String,
    pub message: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    #[serde(rename = "_id", skip_serializing)]
    pub id: Option<ObjectId>,
    pub user: String,
    pub pass: String,
}
