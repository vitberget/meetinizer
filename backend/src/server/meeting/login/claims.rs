use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode, get_current_timestamp};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MeetingEmailClaims {
    meeting: String,
    email: String,
    iat: u64,
    exp: u64,
}

impl MeetingEmailClaims {
    pub fn new(meeting: &str, email: &str) -> Self {
        Self {
            meeting: meeting.to_string(),
            email: email.to_string(),
            iat: get_current_timestamp(),
            exp: get_current_timestamp() + 30*24*60*60 
        }
    }
    pub fn to_jwt(&self, secret: &str) -> anyhow::Result<String> {
        Ok(encode(
            &Header::default(), 
            &MeetingEmailClaims::new(&self.meeting, &self.email), 
            &EncodingKey::from_secret(secret.as_ref())
            )?)
    }
    pub fn get_meeting(&self) -> &str { &self.meeting }
    pub fn get_email(&self) -> &str { &self.email }
}

impl TryFrom<(&str, &str)> for MeetingEmailClaims {
    type Error = anyhow::Error;

    fn try_from((token, secret): (&str, &str)) -> Result<Self, Self::Error> {
        let token_data = decode::<MeetingEmailClaims>(
            token, 
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(jsonwebtoken::Algorithm::HS256))?;
        Ok(token_data.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_claims() -> anyhow::Result<()> {
        let original_claim = MeetingEmailClaims::new("random_meeting", "some_individual"); 
        let jwt = original_claim.to_jwt("secret123")?;
        let parsed_claim: MeetingEmailClaims = (jwt.as_str(), "secret123").try_into()?;
        assert_eq!(original_claim, parsed_claim);
        Ok(())
    }
}
