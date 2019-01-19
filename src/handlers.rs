use iron::*;
use iron::mime::{Mime, TopLevel, SubLevel};
use fair_boss_drop_server::postgres::Connection;

use std::io::Read;
use std::sync::Mutex;
use std::collections::LinkedList;

use BdLayer::PostgresCommands::*;
use BdLayer::ItemsCommands::*;
use BdLayer::BossCommands::*;
use BdLayer::PostgresDealer::*;


pub fn get_item_types(sdb: &Mutex<PostgresSqlData>, req: &mut Request) -> IronResult<Response> {
    let mut bd_data = sdb.lock().unwrap();

    let mut getItemTypes = PostgresGetItemTypes::new();
    match bd_data.doCommand(&mut getItemTypes) {
        Ok(res) => {  println!("get item types"); },
        Err(er) => {  println!("{}",er); }
    }
    println!("start {:?}",getItemTypes.getData());

    Ok(Response::with((status::Ok,"Get Item Types executed")))
}

pub fn insert_item_type(sdb: &Mutex<PostgresSqlData>, req: &mut Request) -> IronResult<Response> {
    let mut bd_data = sdb.lock().unwrap();

    let mut addItemType = PostgresInsertItemType::new("tip1");
    match bd_data.doCommand(&mut addItemType) {
        Ok(res) => {  println!("item added"); },
        Err(er) => {  println!("{}",er); }
    };
    Ok(Response::with((status::Ok,"insert item type executed")))
}

pub fn insert_boss(sdb: &Mutex<PostgresSqlData>, req: &mut Request) -> IronResult<Response> {
    let mut bd_data = sdb.lock().unwrap();

    let mut items : Vec<ItemProbability> = Vec::new();
    items.push(ItemProbability{ _id:2, _probability: 0.5});
    items.push(ItemProbability{ _id:3, _probability: 0.25});

    let mut insertBoss = PostgresInsertBoss::new("boss4",2,items_list);
    match bd_data.doCommand(&mut insertBoss) {
        Ok(res) => {  println!("boss added"); },
        Err(er) => {  println!("{}",er); }
    }

    Ok(Response::with((status::Ok,"insert boss executed")))
}

pub fn get_boss(sdb: &Mutex<PostgresSqlData>, req: &mut Request) -> IronResult<Response> {
    let mut bd_data = sdb.lock().unwrap();

    let mut getBoss = PostgresGetBoss::new("boss3");
    match bd_data.doCommand(&mut getBoss) {
        Ok(res) => {  println!("get boss"); },
        Err(er) => {  println!("{}",er); }
    }

    Ok(Response::with((status::Ok,"Get boss executed")))
}

pub fn get_bosses(sdb: &Mutex<PostgresSqlData>, req: &mut Request) -> IronResult<Response> {
    let mut bd_data = sdb.lock().unwrap();

    let mut getBosses = PostgresGetBosses::new();
    match bd_data.doCommand(&mut getBosses) {
        Ok(res) => {  println!("get bosses");
                      println!("Bosses {:?}",getBosses.getData());
        },
        Err(er) => {  println!("{}",er); }
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
