use iron::*;
use iron::mime::{Mime, TopLevel, SubLevel};

use std::io::Read;
use std::sync::Mutex;

use BdLayer::ItemsCommands::*;
use BdLayer::PostgresDealer::*;
use fair_boss_drop_server::serde_json;


pub fn get_item_types(sdb: &Mutex<PostgresSqlData>, req: &mut Request) -> IronResult<Response> {
    let mut bd_data = sdb.lock().unwrap();

    let mut get_item_types = PostgresGetItemTypes::new();
    match bd_data.doCommand(&mut get_item_types) {
        Ok(res) => {
            println!("get item_types");
            if let Ok(json) = serde_json::to_string(&get_item_types) {
                let content_type = Mime(TopLevel::Application, SubLevel::Json, Vec::new());
                return Ok(Response::with((content_type, status::Ok, json)));
            }

            return Ok(Response::with((status::InternalServerError,
                                      "couldn't convert records to JSON")));
        },
        Err(er) => { let err_mes = format!("get item_types command execute error {}",er);
                     return Ok(Response::with((status::InternalServerError, err_mes)));
        }
    }
}

pub fn insert_item_type(sdb: &Mutex<PostgresSqlData>, req: &mut Request) -> IronResult<Response> {
    let mut bd_data = sdb.lock().unwrap();

    let mut body = String::new();
    if let Err(er) = req.body.read_to_string(&mut body) {
        return Ok(Response::with((status::InternalServerError,
                                  "couldn't read request body")));
    }

    let mut add_item_type: PostgresInsertItemType = serde_json::from_str(&body).unwrap();
    let mut commands: Vec<Box<PostgresCommand>> = vec![Box::new(add_item_type),Box::new(PostgresGetItemTypes::new())];

    if let Err(er) = bd_data.doCommands(&mut commands) {
        let err_mes = format!("insert item_type command execute error {}",er);
        return Ok(Response::with((status::InternalServerError, err_mes)));
    }

    let mut get_item_types = commands.pop().unwrap();

    if let Ok(json) = serde_json::to_string(&get_item_types) {
        let content_type = Mime(TopLevel::Application, SubLevel::Json, Vec::new());
        return Ok(Response::with((content_type, status::Ok, json)));
    }

    return Ok(Response::with((status::InternalServerError,
                              "couldn't convert records to JSON")));
}
