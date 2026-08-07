use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

//==================| ENUMS ALREADY PROVIDED |=====================

// DB type: user_role_type
#[derive(PartialEq, Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_role_type", rename_all = "lowercase")]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    User,
    Seller,
    Moderator,
    Admin,
}

// DB type: user_status_type
#[derive(PartialEq, Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_status_type", rename_all = "lowercase")]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    Unverified,
    Active,
    Banned,
}

// DB type: auth_provider_type
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "auth_provider_type", rename_all = "lowercase")]
#[serde(rename_all = "snake_case")]
pub enum AuthProvider {
    Google,
}

//==================| NEW ENUMS |=====================

// DB type: order_status_type
#[derive(PartialEq, Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "order_status_type", rename_all = "lowercase")]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
    Returned,
}

// DB type: payment_status_type
#[derive(PartialEq, Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "payment_status_type", rename_all = "lowercase")]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    Pending,
    Paid,
    Failed,
    Refunded,
}

// DB type: product_status_type
#[derive(PartialEq, Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "product_status_type", rename_all = "lowercase")]
#[serde(rename_all = "snake_case")]
pub enum ProductStatus {
    Draft,
    Active,
    Archived,
}

//==================| USER STRUCTS (Provided) |=====================

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub description: Option<String>,
    pub role: UserRole,
    pub status: UserStatus,
    pub provider: AuthProvider,
    pub provider_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub refresh_token_hash: String,
    pub user_agent: Option<String>,
    pub revoked_at: Option<DateTime<Utc>>, // Added missing revoked_at from SQL
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserAddress {
    // Renamed from UserAddresses for singular convention
    pub id: Uuid,
    pub user_id: Uuid,
    pub recipient_name: String,
    pub recipient_phone: String,
    pub address_line: String,
    pub ward: Option<String>,
    pub district: String,
    pub city: String,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
