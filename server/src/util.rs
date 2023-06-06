use actix_web::{error, http, HttpRequest, Result};
use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("authorization not in form 'Bearer <token>'")]
struct InvalidBearerAuthorization;

fn extract_bearer_token(value: &str) -> std::result::Result<&str, InvalidBearerAuthorization> {
    lazy_static! {
        static ref TOKEN_RE: Regex = Regex::new(r"^Bearer (\S+)$").unwrap();
    }

    if let Some(captures) = TOKEN_RE.captures(value) {
        Ok(captures.get(1).unwrap().as_str())
    } else {
        Err(InvalidBearerAuthorization)
    }
}

pub fn authenticate_user(req: &HttpRequest) -> Result<auth::AuthenticatedUser> {
    fn map_err<E: std::fmt::Display>(err: E) -> error::Error {
        warn!("invalid access token: {}", err);
        error::ErrorUnauthorized("UNAUTHORIZED")
    }

    if let Some(authorization_header) = req.headers().get(http::header::AUTHORIZATION) {
        let authorization = authorization_header.to_str().map_err(map_err)?;
        let token = extract_bearer_token(authorization).map_err(map_err)?;
        auth::verify_user_token(token).map_err(map_err)
    } else {
        Err(error::ErrorUnauthorized("UNAUTHORIZED"))
    }
}
