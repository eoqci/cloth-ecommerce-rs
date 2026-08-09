use oauth2::{
    AuthUrl, ClientId, ClientSecret, EndpointNotSet, EndpointSet, RedirectUrl, TokenUrl,
    basic::BasicClient,
};

use crate::{config::Config, errors::AppError};

// Generic order: HasAuthUrl, HasDeviceAuthUrl, HasIntrospectionUrl, HasRevocationUrl, HasTokenUrl
pub type GoogleOAuthClient =
    BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>;

pub fn build_oauth_client(config: &Config) -> Result<GoogleOAuthClient, AppError> {
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .map_err(|e| AppError::Config(e.to_string()))?;
    let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
        .map_err(|e| AppError::Config(e.to_string()))?;
    let redirect_url = RedirectUrl::new(config.google_redirect_uri.clone())
        .map_err(|e| AppError::Config(e.to_string()))?;

    Ok(
        BasicClient::new(ClientId::new(config.google_client_id.clone()))
            .set_client_secret(ClientSecret::new(config.google_client_secret.clone()))
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
            .set_redirect_uri(redirect_url),
    )
}
