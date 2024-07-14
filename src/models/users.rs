use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub password: String,
    pub name: Option<String>,
    pub profile: UserProfile,
    pub settings: UserSettings,
    pub balance: f64,
    pub trades: Vec<ObjectId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSettings {
    pub theme: String,
    pub notifications: UserNotifications,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserNotifications {
    pub email: bool,
    pub sms: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthPayload {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub success: bool,
    pub message: String,
    pub user_id: Option<String>,
}
