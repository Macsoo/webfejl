use jsonwebtoken::{decode, decode_header, DecodingKey, Validation};
use jsonwebtoken::jwk::JwkSet;
use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Claims {
    pub sub: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.cookies().get_private("sub") {
            Some(cookie) => Outcome::Success(Claims {
                sub: cookie.value().to_string(),
            }),
            None => Outcome::Error((Status::Forbidden, ()))
        }
    }
}

pub async fn authorize(jwt: &str) -> Option<String> {
    let jwk_set = reqwest::get("https://www.googleapis.com/oauth2/v3/certs")
        .await.ok()?
        .json::<JwkSet>()
        .await.ok()?;
    let header = decode_header(jwt).ok()?;
    let jwk = jwk_set.find(&*header.kid?)?;
    let mut validation = Validation::new(header.alg);
    validation.set_audience(&[dotenvy::var("GOOGLE_CLIENT_ID").ok()?]);
    validation.set_issuer(&["accounts.google.com", "https://accounts.google.com"]);
    let jwt = decode::<Claims>(jwt, &DecodingKey::from_jwk(jwk).ok()?, &validation).ok()?;
    Some(jwt.claims.sub)
}