extern crate fair_boss_drop_server;

extern crate gotham;
#[macro_use]
extern crate gotham_derive;

extern crate hyper;

use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::{FromState, State};

use std::sync::{Arc, Mutex};

use fair_boss_drop_server::BdLayer;
use BdLayer::PostgresDealer::*;
use BdLayer::PostgresCommands::PostgresInitTables;

mod item_handlers;
mod boss_handlers;
mod auth_handlers;

#[derive(Deserialize, StateData, StaticResponseExtender)]
struct IdPathExtractor {
    id: i32,
}


fn router() -> Router {
    build_simple_router(|route| {
        router.get("/login").to(auth_handlers::login);
        router.get("/auth").to(auth_handlers::authorize);

        route.scope("/api/v0", |route| {

            route.associate("itemtypes", |assoc| {
                assoc.get().to(item_handlers::get_item_types);
                assoc.post().to(item_handlers::insert_item_type);
            });

            route.get("items/:id").with_path_extractor::<IdPathExtractor>()
                .to(item_handlers::get_item);
            route.associate("items", |assoc| {
                assoc.get().to(item_handlers::get_items);
                assoc.post().to(item_handlers::insert_item);
            });

            route.get("bosses/:id").with_path_extractor::<IdPathExtractor>()
                .to(boss_handlers::get_boss);
            route.associate("bosses", |assoc| {
                assoc.get().to(boss_handlers::get_bosses);
                assoc.post().to(boss_handlers::insert_boss);
            });
        });
    })

    // {
    // let sdb = Arc::new(Mutex::new(db));
    // {   let sdb_ = sdb.clone();
    //     router.get("/api/v0/bosses/:id", move |req: &mut Request|
    //                boss_handlers::get_boss(&sdb_.clone(), req), "get_boss");    }
}


fn main() {
    // let mut bd_data = PostgresSqlData::new();
    // bd_data.connect().unwrap();
    // {
    //     let mut init_tables = PostgresInitTables::new();
    //     bd_data.doCommand(&mut init_tables).expect("Error when init db tables");
    // }

    let addr = "localhost:3000";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr,router)

    println!("Hello, world!");
}
