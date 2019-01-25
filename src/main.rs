extern crate fair_boss_drop_server;

extern crate iron;
extern crate router;
extern crate url;

use iron::*;
use std::sync::{Arc, Mutex};

use fair_boss_drop_server::BdLayer;
use BdLayer::PostgresDealer::*;
use BdLayer::PostgresCommands::PostgresInitTables;
mod item_handlers;
mod boss_handlers;


fn serve(db: PostgresSqlData) {
    let sdb = Arc::new(Mutex::new(db));
    let mut router = router::Router::new();

    {   let sdb_ = sdb.clone();
        router.get("/api/v0/itemtypes", move |req: &mut Request|
                   item_handlers::get_item_types(&sdb_.clone(), req), "get_all_item_types"); }
    {   let sdb_ = sdb.clone();
        router.post("/api/v0/itemtypes", move |req: &mut Request|
                   item_handlers::insert_item_type(&sdb_.clone(), req), "insert_into_item_types"); }

    {   let sdb_ = sdb.clone();
        router.get("/api/v0/items", move |req: &mut Request|
                   item_handlers::get_items(&sdb_.clone(), req), "get_all_items"); }
    {   let sdb_ = sdb.clone();
        router.post("/api/v0/items", move |req: &mut Request|
                   item_handlers::insert_item(&sdb_.clone(), req), "insert_into_items"); }


    {   let sdb_ = sdb.clone();
        router.get("/api/v0/bosses", move |req: &mut Request|
                   boss_handlers::get_bosses(&sdb_.clone(), req), "get_all_bosses");    }
    {   let sdb_ = sdb.clone();
        router.get("/api/v0/bosses/:id", move |req: &mut Request|
                   boss_handlers::get_boss(&sdb_.clone(), req), "get_boss");    }

    Iron::new(router).http("localhost:3000").expect("Error when start iron server.");
}


fn main() {
    let mut bd_data = PostgresSqlData::new();
    bd_data.connect().unwrap();
    {
        let mut init_tables = PostgresInitTables::new();
        bd_data.doCommand(&mut init_tables).expect("Error when init db tables");
    }

    serve(bd_data);

    println!("Hello, world!");
}
