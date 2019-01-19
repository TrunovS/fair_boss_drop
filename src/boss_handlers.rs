use iron::*;
use iron::mime::{Mime, TopLevel, SubLevel};

use std::io::Read;
use std::sync::Mutex;

use BdLayer::BossCommands::*;
use BdLayer::PostgresDealer::*;

pub fn insert_boss(sdb: &Mutex<PostgresSqlData>, req: &mut Request) -> IronResult<Response> {
    let mut bd_data = sdb.lock().unwrap();

    let mut items : Vec<ItemProbability> = Vec::new();
    items.push(ItemProbability{ _id:2, _probability: 0.5});
    items.push(ItemProbability{ _id:3, _probability: 0.25});

    let mut insert_boss = PostgresInsertBoss::new("boss4",2,items);
    match bd_data.doCommand(&mut insert_boss) {
        Ok(res) => {  println!("boss added"); },
        Err(er) => {  println!("{}",er); }
    }

    Ok(Response::with((status::Ok,"insert boss executed")))
}

pub fn get_boss(sdb: &Mutex<PostgresSqlData>, req: &mut Request) -> IronResult<Response> {
    let mut bd_data = sdb.lock().unwrap();

    let mut get_boss = PostgresGetBoss::new("boss3");
    match bd_data.doCommand(&mut get_boss) {
        Ok(res) => {  println!("get boss"); },
        Err(er) => {  println!("{}",er); }
    }

    Ok(Response::with((status::Ok,"Get boss executed")))
}

pub fn get_bosses(sdb: &Mutex<PostgresSqlData>, req: &mut Request) -> IronResult<Response> {
    let mut bd_data = sdb.lock().unwrap();

    let mut get_bosses = PostgresGetBosses::new();
    match bd_data.doCommand(&mut get_bosses) {
        Ok(res) => {  println!("get bosses");
                      println!("Bosses {:?}",get_bosses.getData());
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
