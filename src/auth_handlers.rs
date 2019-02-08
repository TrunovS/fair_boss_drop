use iron::*;
use iron::mime::{Mime, TopLevel, SubLevel};
use iron::modifiers::Redirect;

use fair_boss_drop_server::iron_sessionstorage;
use iron_sessionstorage::traits::*;
use iron_sessionstorage::SessionStorage;
use iron_sessionstorage::backends::SignedCookieBackend;

use std::io::Read;
use std::sync::Mutex;

use BdLayer::ItemsCommands::*;
use BdLayer::PostgresDealer::*;
use fair_boss_drop_server::serde_json;

use fair_boss_drop_server::oauth2::Config;
use std::env;

const GOOGLE_TOKEN: &'static str = "https://www.googleapis.com/oauth2/v4/token";
const GOOGLE_AUTH: &'static str = "https://accounts.google.com/o/oauth2/v2/auth";
const GOOGLE_SCOPE: &'static str = "https://www.googleapis.com/auth/userinfo.email";
const REDIRECT_URL: &'static str  = "http://localhost:3000/auth";

struct Login {
    username: String
}

impl iron_sessionstorage::Value for Login {
    fn get_key() -> &'static str { "logged_in_user" }
    fn into_raw(self) -> String { self.username }
    fn from_raw(value: String) -> Option<Self> {
        if value.is_empty() {
            None
        } else {
            Some(Login { username: value })
        }
    }
}

pub fn login(req: &mut Request) -> IronResult<Response> {
    // let google_client_id = env::var("GOOGLE_CLIENT_ID").
    //     expect("Missing the GOOGLE_CLIENT_ID environment variable.");
    // let google_client_secret =  env::var("GOOGLE_CLIENT_SECRET").
    //     expect("Missing the GOOGLE_CLIENT_SECRET environment variable.");

    let mut config = Config::new("87793627289-8mq9lgkbbdl1641j1hcmtvp5861libn5.apps.googleusercontent.com",
                                 "6ojI9rMhz5gOsioZRFaUSE5Y",
                                 GOOGLE_AUTH,
                                 GOOGLE_TOKEN);
    config = config.add_scope(GOOGLE_SCOPE);
    config = config.set_redirect_url(REDIRECT_URL);

    if let Ok(auth_page) = Url::from_generic_url(config.authorize_url()) {
        return Ok(Response::with((status::Found, Redirect(auth_page))));
    }

    Ok(Response::with((status::InternalServerError,"couldn't redirect to auth")))
}


pub fn authorize(req: &mut Request) -> IronResult<Response> {
    use params::{Params,Value};

    let map = req.get_ref::<Params>().unwrap();

    if let Some(&Value::String(ref code)) = map.find(&["code"]) {
        println!("{:?}",code);
        let mut config = Config::new("87793627289-8mq9lgkbbdl1641j1hcmtvp5861libn5.apps.googleusercontent.com",
                                     "6ojI9rMhz5gOsioZRFaUSE5Y",
                                     GOOGLE_AUTH,
                                     GOOGLE_TOKEN);
        config = config.add_scope(GOOGLE_SCOPE);
        config = config.set_redirect_url(REDIRECT_URL);

        println!("token: {:?}",config.exchange_code(code.as_str()));
    }



    Ok(Response::with((status::InternalServerError,"couldn't authentificate")))
}
