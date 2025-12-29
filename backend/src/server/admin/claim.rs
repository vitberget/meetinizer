use anyhow::bail;
use axum_extra::extract::CookieJar;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode, get_current_timestamp};
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::config::get_jwt_secret;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdminClaims {
    iat: u64,
    exp: u64,
}

impl AdminClaims {
    pub fn to_jwt(&self, secret: &str) -> anyhow::Result<String> {
        Ok(encode(
            &Header::default(), 
            self,
            &EncodingKey::from_secret(secret.as_ref())
            )?)
    }

    pub fn get_and_validate(cookies: &CookieJar) -> anyhow::Result<AdminClaims> {
        match cookies.get("admin") {
            Some(admin) => match get_jwt_secret() {
                Ok(secret) => match <(&str, &str) as TryInto<AdminClaims>>::try_into((admin.value(), &secret)) {
                    Ok(claims) => Ok(claims),
                    Err(error) => {
                        warn!("not a claim in cookie {error}");
                        bail!("not a claim in cookie {error}");
                    }
                }

                Err(error) => {
                    warn!("failed get_jwt_secret {error}");
                    bail!("failed get_jwt_secret {error}");
                }
            }
            None => {
                warn!("missing admin cookie");
                bail!("missing admin cookie");
            }
        }
    }
}

impl Default for AdminClaims {
    fn default() -> Self {
        Self {
            iat: get_current_timestamp(),
            exp: get_current_timestamp() + 30*24*60*60 
        }
    }
}

impl TryFrom<(&str, &str)> for AdminClaims {
    type Error = anyhow::Error;

    fn try_from((token, secret): (&str, &str)) -> Result<Self, Self::Error> {
        let token_data = decode::<AdminClaims>(
            token, 
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(jsonwebtoken::Algorithm::HS256))?;
        Ok(token_data.claims)
    }
}
