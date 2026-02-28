use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,  // User ID
    pub email: String,
    pub exp: usize,   // Expiration time
    pub iat: usize,   // Issued at
}
