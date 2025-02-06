use jsonwebtoken::errors::Error;
pub use jsonwebtoken::errors::ErrorKind as JWTErrorKind;
pub use jsonwebtoken::Algorithm;
use jsonwebtoken::{
    decode as jdec, encode as jenc, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: i64,
    pub iat: i64,
    pub email: String,
    pub role: u64,
    pub session: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedisInfo {
    pub email: String,
    pub role: u64,
    pub session: u64,
    pub address: String,
    pub browser: String,
    pub token: String,
}

pub fn generate_token<T>(algorithm: Algorithm, key: &str, claims: T) -> Result<String, Error>
where
    T: Serialize,
{
    jenc(
        &Header::new(algorithm),
        &claims,
        &EncodingKey::from_secret(key.as_bytes()),
    )
}

pub fn validate_token<T>(
    algorithm: Algorithm,
    key: &str,
    token: &str,
) -> Result<TokenData<T>, Error>
where
    for<'a> T: Deserialize<'a>,
{
    let validation = Validation::new(algorithm);
    jdec::<T>(
        &token,
        &DecodingKey::from_secret(key.as_bytes()),
        &validation,
    )
}

// cargo test --package lib_crypto check_jwt_function -- --nocapture
#[cfg(test)]
mod test {
    use super::*;
    use lib_config::AppConfig;
    #[test]
    fn check_jwt_functions() {
        let mut my_claim = Claims {
            exp: i64::MAX,
            iat: 0,
            email: "test@unit".to_owned(),
            id: 0,
            role: 1000,
            session: 1,
        };
        let config = AppConfig::new();
        let secret = config.jwt_access_key;
        match generate_token(Algorithm::HS256, &secret, &my_claim) {
            Ok(token) => {
                println!("Valid Token: {}", token);
                let validate = validate_token::<Claims>(Algorithm::HS256, &secret, &token).unwrap();
                println!("Validation Claim {:#?}", validate.claims);
                println!("Validation Header {:#?}", validate.header);
                assert_eq!(validate.claims.email, "test@unit".to_owned());
            }
            Err(e) => println!("{}", e),
        };
        my_claim.exp = 0;
        let token = generate_token(Algorithm::HS256, &secret, &my_claim).unwrap();
        println!("Invalid Token: {}", token);
        let error = validate_token::<Claims>(Algorithm::HS256, &secret, &token).err();
        assert_eq!(error.unwrap().kind(), &JWTErrorKind::ExpiredSignature)
    }
}
