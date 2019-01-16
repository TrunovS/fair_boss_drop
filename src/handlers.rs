use iron::*;
use iron::mime::{Mime, TopLevel, SubLevel};
use postgres::Connection;

use std::io::Read;
use std::sync::Mutex;
use std::collections::LinkedList;

use BdLayer::PostgresCommands::*;
use BdLayer::ItemsCommands::*;
use BdLayer::BossCommands::*;
use BdLayer::PostgresDealer::*;


pub fn get_bosses(sdb: &Mutex<PostgresSqlData>, req: &mut Request) -> IronResult<Response> {
    let mut bd_data = sdb.lock().unwrap();
    {
        let mut initTables = PostgresInitTables::new();
        bd_data.doCommand(&mut initTables).unwrap();
    }
    {
        let mut getItemTypes = PostgresGetItemTypes::new();
        bd_data.doCommand(&mut getItemTypes).unwrap();
        println!("start {:?}",getItemTypes.getData());
    }

    {
        let mut addItemType = PostgresInsertItemType::new("tip1");
        match bd_data.doCommand(&mut addItemType) {
            Ok(res) => {  println!("item added"); },
            Err(er) => {  println!("{}",er); }
        };
    }
    {
        let mut addItemType = PostgresInsertItemType::new("tip2");
        match bd_data.doCommand(&mut addItemType) {
            Ok(res) => {  println!("item added"); },
            Err(er) => {  println!("{}",er); }
        };
    }

    {
        let mut getItemTypes = PostgresGetItemTypes::new();
        bd_data.doCommand(&mut getItemTypes).unwrap();
        println!("end {:?}",getItemTypes.getData());
    }
    {
        let mut items_list = LinkedList::new();
        items_list.push_back(item_probability{ _id:2, _probability: 0.5});
        items_list.push_back(item_probability{_id:3, _probability: 0.25});
        let mut insertBoss = PostgresInsertBoss::new("boss4",2,items_list);
        bd_data.doCommand(&mut insertBoss).unwrap();
    }

    Ok(Response::with((status::Ok,"Command executed")))

    // let url = req.url.clone().into_generic_url();
    // let path = url.path().unwrap();
    // let sid: &str = &path.iter().last().unwrap();
    // let id;
    // if let Ok(r) = sid.parse() {
    //     id = r;
    // } else {
    //     return Ok(Response::with((status::BadRequest, "bad id")));
    // }

    // let json_record;
    // if let Ok(recs) = ::db::read_one(sdb, id) {
    //     if let Ok(json) = json::encode(&recs) {
    //         json_record = Some(json);
    //     } else {
    //         return Ok(Response::with((status::InternalServerError,
    //                                   "couldn't convert records to JSON")));
    //     }
    // } else {
    //     return Ok(Response::with((status::InternalServerError,
    //                               "couldn't read records from database")));
    // }
    // let content_type = Mime(TopLevel::Application, SubLevel::Json, Vec::new());

    // Ok(Response::with((content_type, status::Ok, json_record.unwrap())))
}
