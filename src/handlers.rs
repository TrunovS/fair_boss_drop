use iron::*;
use iron::mime::{Mime, TopLevel, SubLevel};
use postgres::Connection;

use std::io::Read;
use std::sync::Mutex;

use BdLayer::PostgresCommands::*;
use BdLayer::PostgresDealer::*;

pub fn get_bosses(sdb: &Mutex<PostgresSqlData>, req: &mut Request) -> IronResult<Response> {
    let mut bd_data = sdb.lock().unwrap();
    bd_data.doCommand(PostgresInitTables::new()).unwrap();

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
