use std::ops::Add;
use chrono::{DateTime, Duration, Utc};
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use chrono::serde::ts_seconds;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::de::DeserializeOwned;

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum AuthTokenKind {
    UserInternal = 0,
    UserExternal = 1,
    Beacon = 2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthTokenData {
    #[serde(rename = "typ")]
    pub kind: AuthTokenKind,
    #[serde(rename = "exp", with = "ts_seconds")]
    pub expires_at: DateTime<Utc>,
    #[serde(rename = "tid")]
    pub target_id: i64,
}

impl AuthTokenData {
    pub fn new(kind: AuthTokenKind, target_id: i64) -> Self {
        Self {
            kind,
            target_id,
            expires_at: Utc::now().add(Duration::days(7))
        }
    }

    pub fn user_internal(user_id: i64) -> Self {
        Self::new(AuthTokenKind::UserInternal, user_id)
    }

    pub fn user_external(user_id: i64) -> Self {
        Self::new(AuthTokenKind::UserExternal, user_id)
    }

    pub fn _beacon(beacon_id: i64) -> Self {
        Self::new(AuthTokenKind::Beacon, beacon_id)
    }
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum PermitTokenKind {
    UserBeaconJoin = 0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermitTokenData {
    #[serde(rename = "typ")]
    pub kind: PermitTokenKind,
    #[serde(rename = "exp", with = "ts_seconds")]
    pub expires_at: DateTime<Utc>,
    #[serde(rename = "tid")]
    pub target_id: i64,
}

pub fn encode_token<T: Serialize>(secret: &str, data: &T) -> jsonwebtoken::errors::Result<String> {
    jsonwebtoken::encode(
        &Header::default(),
        data,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn decode_token<T: DeserializeOwned>(secret: &str, token: &str) -> jsonwebtoken::errors::Result<T> {
    jsonwebtoken::decode::<T>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    ).map(|d| d.claims)
}
