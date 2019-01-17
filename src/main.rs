#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate iron;
extern crate router;
extern crate url;

mod handlers;
mod BdLayer;

use iron::*;
use std::sync::{Arc, Mutex};
use BdLayer::PostgresDealer::*;
use BdLayer::PostgresCommands::PostgresInitTables;

fn serve(db: PostgresSqlData) {
    let sdb = Arc::new(Mutex::new(db));
    let mut router = router::Router::new();
    {
        let sdb_ = sdb.clone();

        router.get("/api/v0/itemtypes",
                   move |req: &mut Request|
                   handlers::get_item_types(&sdb_.clone(), req),
                   "test_bosses");

        router.get("/api/v0/bosses",
                   move |req: &mut Request|
                   handlers::get_bosses(&sdb_.clone(), req),
                   "test_bosses");
    }
    Iron::new(router).http("localhost:3000").unwrap();
}


fn main() {
    let mut bd_data = PostgresSqlData::new();
    bd_data.connect().unwrap();

    {
        let mut initTables = PostgresInitTables::new();
        bd_data.doCommand(&mut initTables).unwrap();
    }

    serve(bd_data);

    println!("Hello, world!");
}
