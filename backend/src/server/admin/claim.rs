use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode, get_current_timestamp};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdminClaims {
    iat: u64,
    exp: u64,
}

impl AdminClaims {
    pub fn new() -> Self {
        Self {
            iat: get_current_timestamp(),
            exp: get_current_timestamp() + 30*24*60*60 
        }
    }
    pub fn to_jwt(&self, secret: &str) -> anyhow::Result<String> {
        Ok(encode(
            &Header::default(), 
            &AdminClaims::new(), 
            &EncodingKey::from_secret(secret.as_ref())
            )?)
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
