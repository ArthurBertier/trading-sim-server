use log::{info, warn, error};
use mongodb::{bson::{doc, oid::ObjectId}, Collection, Database, Client};
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::models::users::{User, AuthPayload, AuthResponse, UserNotifications, UserProfile, UserSettings};
use crate::db::mongo;
use sentry::capture_message;

pub async fn get_database() -> Result<Database, String> {
    let client: Client = mongo::init().await.map_err(|e| e.to_string())?;
    Ok(client.database("trading_simulator"))
}

pub async fn login(payload: AuthPayload) -> Result<AuthResponse, String> {
    let db = get_database().await?;
    let users: Collection<User> = db.collection("users");

    match users.find_one(doc! {"email": &payload.email}, None).await {
        Ok(Some(user)) => {
            if verify(&payload.password, &user.password).is_ok() {
                info!("Login successful for: {}", payload.email);
                capture_message(&format!("Login successful for: {}", payload.email), sentry::Level::Info);
                Ok(AuthResponse {
                    success: true,
                    message: "Login successful".to_string(),
                    user_id: user.id.map(|id| id.to_hex())
                })
            } else {
                warn!("Failed login attempt for: {}", payload.email);
                capture_message(&format!("Failed login attempt for: {}", payload.email), sentry::Level::Warning);
                Err("Invalid email or password".into())
            }
        },
        Ok(None) => {
            warn!("No user found for email: {}", payload.email);
            capture_message(&format!("No user found for email: {}", payload.email), sentry::Level::Warning);
            Err("User not found".into())
        },
        Err(e) => {
            error!("Database error during login: {}", e);
            capture_message(&format!("Database error during login: {}", e), sentry::Level::Error);
            Err("Internal server error".into())
        }
    }
}

pub async fn register(payload: AuthPayload) -> Result<AuthResponse, String> {
    let db = get_database().await?;
    let users: Collection<User> = db.collection("users");

    if users.find_one(doc! {"email": &payload.email}, None).await.unwrap().is_some() {
        warn!("Registration attempt for already registered email: {}", payload.email);
        capture_message(&format!("Registration attempt for already registered email: {}", payload.email), sentry::Level::Warning);
        return Err("Email already registered".into());
    }

    let hashed_password = match hash(&payload.password, DEFAULT_COST) {
        Ok(p) => p,
        Err(_) => {
            error!("Password hashing failed for: {}", payload.email);
            capture_message(&format!("Password hashing failed for: {}", payload.email), sentry::Level::Error);
            return Err("Password hashing failed".into());
        }
    };

    let new_user = User {
        id: None,
        email: payload.email.clone(),
        password: hashed_password,
        name: None,
        profile: UserProfile { bio: None, avatar_url: None },
        settings: UserSettings { theme: "light".to_string(), notifications: UserNotifications { email: true, sms: false } },
        balance: 10000.0,
        trades: vec![],
    };

    match users.insert_one(new_user, None).await {
        Ok(insert_result) => {
            let user_id = insert_result.inserted_id.as_object_id().unwrap();
            info!("User registration successful for: {}", payload.email);
            capture_message(&format!("User registration successful for: {}", payload.email), sentry::Level::Info);
            Ok(AuthResponse {
                success: true,
                message: "Registration successful".to_string(),
                user_id: Some(user_id.to_hex())
            })
        },
        Err(e) => {
            error!("Failed to register user: {}", e);
            capture_message(&format!("Failed to register user: {}", e), sentry::Level::Error);
            Err("Internal server error".into())
        }
    }
}
