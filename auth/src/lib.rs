use db::UserId;

#[derive(Debug)]
pub struct AuthenticatedUser {
    // internal id
    pub id: UserId,

    // Firebase id (used to lookup internal id, not for application use)
    uid: String,

    // an unregistered user may be authenticated as anonymous
    pub is_anon: bool,
}

// authenticate Firebase token
pub fn verify_user_token(token: &str) -> anyhow::Result<AuthenticatedUser> {
    log::info!("TODO - verify user token and get account info: {}", token);
    // https://firebase.google.com/docs/auth/admin/verify-id-tokens#verify_id_tokens_using_a_third-party_jwt_library

    Ok(AuthenticatedUser {
        id: UserId(0),
        uid: "TODO".to_owned(),
        is_anon: false,
    })
    // Err(
    //     anyhow::Error::new(std::io::Error::from(std::io::ErrorKind::PermissionDenied))
    //         .context("token verification failed"),
    // )
}
