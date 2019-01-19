extern crate fair_boss_drop_server;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate iron;
extern crate router;
extern crate url;

mod handlers;

use iron::*;
use std::sync::{Arc, Mutex};

use fair_boss_drop_server::BdLayer;
use BdLayer::PostgresDealer::*;
use BdLayer::PostgresCommands::PostgresInitTables;
use BdLayer::BossCommands::*;
use std::collections::LinkedList;

fn serve(db: PostgresSqlData) {
    let sdb = Arc::new(Mutex::new(db));
    let mut router = router::Router::new();
    {
        let sdb_ = sdb.clone();

        router.get("/api/v0/itemtypes", move |req: &mut Request|
                   handlers::get_item_types(&sdb_.clone(), req), "get_all_item_types");
    }
    {
        let sdb_ = sdb.clone();

        router.get("/api/v0/bosses", move |req: &mut Request|
                   handlers::get_bosses(&sdb_.clone(), req), "get_all_bosses");
    }

    Iron::new(router).http("localhost:3000").unwrap();
}


fn main() {
    let mut bd_data = PostgresSqlData::new();
    bd_data.connect().unwrap();
    {
        let mut initTables = PostgresInitTables::new();
        bd_data.doCommand(&mut initTables).expect("Error when init db tables");
    }

    // {
    //     let mut lst = LinkedList::new();
    //     lst.push_back(item_probability::new(1,0.2));
    //     lst.push_back(item_probability::new(2,0.4));

    //     let mut insertBoss = PostgresInsertBoss::new("boss1",1,lst);
    //     match bd_data.doCommand(&mut insertBoss) {
    //         Ok(res) => { println!("Boss inserted"); },
    //         Err(e) => { println!("err {}",e); }
    //     }
    // }

    {
        let mut getBoss = PostgresGetBoss::new("boss1");
        match bd_data.doCommand(&mut getBoss) {
            Ok(res) => { println!("Boss get "); },
            Err(e) => { println!("err {}",e); }
        }
    }

    serve(bd_data);

    println!("Hello, world!");
}
