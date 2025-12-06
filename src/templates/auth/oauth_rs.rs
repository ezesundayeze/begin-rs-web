pub fn generate() -> &'static str {
    r#"use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    RedirectUrl, RevocationUrl, Scope, TokenResponse, TokenUrl,
};
use reqwest;
use serde::{Deserialize, Serialize};

use crate::error::{AppError, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleUserInfo {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub picture: Option<String>,
}

pub struct GoogleOAuthClient {
    client: BasicClient,
}

impl GoogleOAuthClient {
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Result<Self> {
        let google_client_id = ClientId::new(client_id);
        let google_client_secret = ClientSecret::new(client_secret);

        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
            .map_err(|e| AppError::InternalError(format!("Invalid auth URL: {}", e)))?;

        let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
            .map_err(|e| AppError::InternalError(format!("Invalid token URL: {}", e)))?;

        let client = BasicClient::new(
            google_client_id,
            Some(google_client_secret),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(
            RedirectUrl::new(redirect_uri)
                .map_err(|e| AppError::InternalError(format!("Invalid redirect URI: {}", e)))?,
        );

        Ok(Self { client })
    }

    pub fn get_authorization_url(&self) -> (String, CsrfToken) {
        let (auth_url, csrf_token) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("email".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .url();

        (auth_url.to_string(), csrf_token)
    }

    pub async fn exchange_code(&self, code: String) -> Result<GoogleUserInfo> {
        let token = self
            .client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(oauth2::reqwest::async_http_client)
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to exchange code: {}", e)))?;

        let access_token = token.access_token().secret();

        let client = reqwest::Client::new();
        let user_info = client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to get user info: {}", e)))?
            .json::<GoogleUserInfo>()
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to parse user info: {}", e)))?;

        Ok(user_info)
    }
}
"#
}
