use iron::*;
use iron::mime::{Mime, TopLevel, SubLevel};

use std::io::Read;
use std::sync::Mutex;

use BdLayer::ItemsCommands::*;
use BdLayer::PostgresDealer::*;
use fair_boss_drop_server::serde_json;

use fair_boss_drop_server::oauth2::Config;
use std::env;

pub fn google_auth() {
    let google_client_id = env::var("GOOGLE_CLIENT_ID").
        expect("Missing the GOOGLE_CLIENT_ID environment variable.");
    let google_client_secret =  env::var("GOOGLE_CLIENT_SECRET").
        expect("Missing the GOOGLE_CLIENT_SECRET environment variable.");


    let mut config = Config::new(google_client_id,
                                 google_client_secret,
                                 "https://accounts.google.com/o/oauth2/v2/auth",
                                 "https://www.googleapis.com/oauth2/v3/token");
    config = config.add_scope("https://www.googleapis.com/auth/userinfo.email");
    config = config.set_redirect_url("http://localhost:8080/auth");

    // Generate the full authorization URL.
    // This is the URL you should redirect the user to, in order to trigger the authorization process.
    println!("Browse to: {}", config.authorize_url());

    let token_result = config.exchange_client_credentials();
}


pub fn login(sdb: &Mutex<PostgresSqlData>, req: &mut Request) -> IronResult<Response> {
    let mut bd_data = sdb.lock().unwrap();

    google_auth();

    Ok(Response::with((status::InternalServerError,"couldn't convert records to JSON")))
}
