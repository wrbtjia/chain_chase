use std::convert::Infallible;
use async_trait::async_trait;




use axum::http::request::Parts;
use axum::RequestPartsExt;
use axum_extra::TypedHeader;
use headers::Authorization;
use headers::authorization::Bearer;

pub struct LoginVerify {}

/*#[async_trait]
impl<S> FromRequestParts<S> for LoginVerify
    where
        S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(Self)
    }
}*/