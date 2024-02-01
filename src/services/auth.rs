use serde::{Deserialize, Serialize};
use std::pin::Pin;
use jsonwebtoken::{Algorithm, decode, DecodingKey, Validation};
use jsonwebtoken::errors::ErrorKind;
use actix_web::{dev::ServiceRequest, Error as ActixError};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use std::error::Error;

#[derive(Debug, Deserialize, Serialize)]
struct JWKS {
    keys: Vec<JWK>,
}

#[derive(Debug, Deserialize, Serialize)]
struct JWK {
    kty: String,
    n: String,
    e: String,
    use_: Option<String>,
    alg: Option<String>,
    kid: Option<String>,
    x5c: Option<Vec<String>>,
}

async fn get_public_keys() -> Result<JWKS, Box<dyn Error>> {
    let jwks_url = std::env::var("API_AUTH_CERTS").map_err(|e| format!("API_AUTH_CERTS must be set: {e:?}")).expect("AUTH_CERTS must be set");
    let response = reqwest::get(&jwks_url).await?;
    let jwks: JWKS = response.json().await?;

    Ok(jwks)
}

#[derive(Debug, Deserialize)]
struct Claims {
    aud: String,
    sub: String,
    exp: u64,
}

pub async fn validate_token(token: &str) -> Result<bool, Box<dyn Error>> {
    let public_keys = get_public_keys().await?.keys;
    for key in public_keys {
        let decoding_key = DecodingKey::from_rsa_components(&key.n, &key.e)?;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&["account"]);

        match decode::<Claims>(&token, &decoding_key, &validation) {
            Ok(res) => {
                log::info!("Token validated: {:?}", res.claims);
                return Ok(true);
            }
            Err(err) => match err.kind() {
                ErrorKind::InvalidToken => continue,
                other_kind => return Err(format!("Error validating token: {:?}", other_kind).into()),
            },
        }
    }

    Ok(false)
}

pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (ActixError, ServiceRequest)> {
    let config = req
        .app_data::<Config>()
        .map(|data| Pin::new(data).get_ref().clone())
        .unwrap_or_default();

    match validate_token(credentials.token()).await {
        Ok(res) => {
            if res {
                Ok(req)
            } else {
                Err((AuthenticationError::from(config).into(), req))
            }
        }
        Err(err) => {
            Err((AuthenticationError::from(config).into(), req))
        }
    }
}