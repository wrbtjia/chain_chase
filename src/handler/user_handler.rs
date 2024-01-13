use std::fmt::Display;
use axum::{ async_trait,
            extract::{Extension, Path},
           Json};
use axum_extra::TypedHeader;
use headers::Authorization;
use headers::authorization::Bearer;
use jsonwebtoken::{DecodingKey, encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use crate::api::api_result::{ApiErr, ApiOK, Result};
use crate::api::api_result::ApiErr::{Error, ErrParams, ErrPassword};



static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = "secret".to_string();
    Keys::new(secret.as_bytes())
});

pub async fn get_user() -> Result<ApiOK<String>>{

    Ok(ApiOK(Some("查询用户".to_string())))
}


pub async fn verify(TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>) -> Result<ApiOK<String>>{
    let digest = md5::compute(b"123456");
    Ok(ApiOK(Some("jwt token 验证成功".to_string())))
}


pub async fn login(Json(payload): Json<AuthPayload>) -> Result<ApiOK<String>>{
    if payload.user_name !="name" {
        return Err(ErrParams(None));
    }
    if payload.password !="e10adc3949ba59abbe56e057f20f883e" {
        return Err(ErrPassword(None));
    }

    let claims = Claims {
        sub: payload.user_name,
        company: payload.password,
        // Mandatory expiry time as UTC timestamp
        exp: 1000000000, // May 2033
    };
    let token = encode(&Header::default(), &claims, &KEYS.encoding).unwrap();
    Ok(ApiOK(Some(token.to_string())))
}



#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    pub user_name: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}
impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Email: {}\nCompany: {}", self.sub, self.company)
    }
}



struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}