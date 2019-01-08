extern crate iron;
extern crate router;
extern crate url;

use iron::*;

mod BdLayer;
mod handlers;

use BdLayer::PostgresDealer::*;
use BdLayer::PostgresCommands::*;
use std::sync::{Arc, Mutex};

fn serve(db: PostgresSqlData) {
    let sdb = Arc::new(Mutex::new(db));
    let mut router = router::Router::new();
    {
        let sdb_ = sdb.clone();
        router.get("/v0/bosses",
                   move |req: &mut Request|
                   handlers::get_bosses(sdb_.clone(), req));
    }
    Iron::new(router).http("localhost:3000").unwrap();
}


fn main() {
    let mut bd_data = PostgresSqlData::new();
    bd_data.connect().unwrap();
    bd_data.doCommand(PostgresInitTables::new()).unwrap();
    println!("status of connection: {}", bd_data.isOpen());
    bd_data.finish().unwrap();
    println!("status of connection: {}", bd_data.isOpen());

    println!("Hello, world!");
}
