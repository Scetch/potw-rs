use oauth2::{
    AuthorizationCode,
    AuthUrl,
    ClientId,
    ClientSecret,
    CsrfToken,
    RedirectUrl,
    Scope,
    TokenUrl,
    prelude::*,
    basic::BasicClient,
};
use reqwest::{ self, Url, header::Authorization };
use failure::{ self, Error };

const CLIENT_ID: &str = env!("GOOGLE_CLIENT_ID");
const CLIENT_SECRET: &str = env!("GOOGLE_CLIENT_SECRET");
const AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const TOKEN_URL: &str = "https://www.googleapis.com/oauth2/v3/token";
const REDIRECT_URL: &str = "http://potw.scetch.net/authorize";

#[derive(Debug, Deserialize)]
pub struct GProfile {
    pub family_name: String,
    pub name: String,
    pub picture: String,
    pub email: String,
    pub given_name: String,
    pub id: String,
    pub hd: String,
    pub verified_email: bool,
}

pub struct OAuth2 {
    client: BasicClient,
}

impl OAuth2 {
    pub fn new() -> Self {
        let client_id = ClientId::new(CLIENT_ID.into());
        let client_secret = ClientSecret::new(CLIENT_SECRET.into());
        let auth_url = AuthUrl::new(Url::parse(AUTH_URL.into()).unwrap());
        let token_url = TokenUrl::new(Url::parse(TOKEN_URL.into()).unwrap());

        let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
            .add_scope(Scope::new("https://www.googleapis.com/auth/userinfo.email".into()))
            .add_scope(Scope::new("https://www.googleapis.com/auth/userinfo.profile".into()))
            .set_redirect_url(RedirectUrl::new(Url::parse(REDIRECT_URL).unwrap()));

        Self {
            client: client,
        }
    }

    pub fn authorize_url(&self) -> (Url, String) {
        let (mut url, state) = self.client.authorize_url(CsrfToken::new_random);
        url.query_pairs_mut().append_pair("hd", "uwindsor.ca"); // Force uwindsor.ca
        (url, state.secret().clone())
    }

    pub fn get_profile(&self, code: String) -> Result<GProfile, Error> {
        let tok = self.client.exchange_code(AuthorizationCode::new(code))?;
        let profile = reqwest::Client::new()
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .header(Authorization(format!("Bearer {}", tok.access_token().secret())))
            .send()
            .and_then(|mut resp| resp.json::<GProfile>())?;

        // Verify that the email is a uwindsor email.
        if profile.hd == "uwindsor.ca" {
            Ok(profile)
        } else {
            Err(failure::err_msg("Not a uwindsor email."))
        }
    }
}
