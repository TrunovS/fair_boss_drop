use iron::*;
use iron::mime::{Mime, TopLevel, SubLevel};
use iron::modifiers::Redirect;

use std::io::Read;
use std::sync::Mutex;

use BdLayer::ItemsCommands::*;
use BdLayer::PostgresDealer::*;
use fair_boss_drop_server::serde_json;

use fair_boss_drop_server::oauth2::Config;
use std::env;


pub fn login(req: &mut Request) -> IronResult<Response> {
    // let google_client_id = env::var("GOOGLE_CLIENT_ID").
    //     expect("Missing the GOOGLE_CLIENT_ID environment variable.");
    // let google_client_secret =  env::var("GOOGLE_CLIENT_SECRET").
    //     expect("Missing the GOOGLE_CLIENT_SECRET environment variable.");

    let mut config = Config::new("87793627289-8mq9lgkbbdl1641j1hcmtvp5861libn5.apps.googleusercontent.com",
                                 "6ojI9rMhz5gOsioZRFaUSE5Y",
                                 "https://accounts.google.com/o/oauth2/v2/auth",
                                 "https://www.googleapis.com/oauth2/v3/token");
    config = config.add_scope("https://www.googleapis.com/auth/userinfo.email");
    config = config.set_redirect_url("http://localhost:3000/auth");

    // // Generate the full authorization URL.
    // // This is the URL you should redirect the user to, in order to trigger the authorization process.
    // println!("Browse to: {}", config.authorize_url());

    // let token_result = config.exchange_client_credentials();

    if let Ok(redir_url) = Url::from_generic_url(config.authorize_url()) {
        return Ok(Response::with((status::Found, Redirect(redir_url))));
    }

    Ok(Response::with((status::InternalServerError,"couldn't redirect to auth")))
}


pub fn auth(req: &mut Request) -> IronResult<Response> {

    Ok(Response::with((status::InternalServerError,"couldn't authentificate")))
}
