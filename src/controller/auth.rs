use jsonwebtoken::{decode, decode_header, DecodingKey, Validation};
use jsonwebtoken::jwk::JwkSet;
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::log::private::{log, Level};
use rocket::Route;
use crate::claims::{authorize, Claims};

#[post("/login", data = "<jwt>")]
pub async fn login(jwt: &str, cookies: &CookieJar<'_>) -> Option<()> {
    let sub = authorize(jwt).await?;
    let mut jwt_cookie = Cookie::new("sub", sub);
    jwt_cookie.set_secure(true);
    jwt_cookie.set_same_site(SameSite::Strict);
    cookies.add_private(jwt_cookie);
    Some(())
}

pub fn routes() -> Vec<Route> {
    routes![login]
}