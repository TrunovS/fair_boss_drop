use iron::*;
use iron::mime::{Mime, TopLevel, SubLevel};

use std::io::Read;
use std::sync::Mutex;
use std::any::Any;

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

    let mut add_item_type: PostgresInsertItemType = serde_json::from_str(&body).expect("can't parse body");
    let mut commands: Vec<Box<PostgresCommand>> = vec![Box::new(add_item_type),
                                                       Box::new(PostgresGetItemTypes::new())];

    if let Err(er) = bd_data.doCommands(&mut commands) {
        let err_mes = format!("insert item_type command execute error {}",er);
        return Ok(Response::with((status::InternalServerError, err_mes)));
    }

    let mut bget_item_types = commands.pop().unwrap();
    let mut aget = Box::leak(bget_item_types);
    if let Some(get_item_types) = aget.downcast_mut::<PostgresGetItemTypes>() {
        if let Ok(json) = serde_json::to_string(get_item_types) {
            let content_type = Mime(TopLevel::Application, SubLevel::Json, Vec::new());
            return Ok(Response::with((content_type, status::Ok, json)));
        }
    }

    return Ok(Response::with((status::InternalServerError,
                              "couldn't convert records to JSON")));
}



pub fn get_items(sdb: &Mutex<PostgresSqlData>, req: &mut Request) -> IronResult<Response> {
    let mut bd_data = sdb.lock().unwrap();

    let mut get_items = PostgresGetItems::new();
    match bd_data.doCommand(&mut get_items) {
        Ok(res) => {
            println!("get items");
            if let Ok(json) = serde_json::to_string(&get_items) {
                let content_type = Mime(TopLevel::Application, SubLevel::Json, Vec::new());
                return Ok(Response::with((content_type, status::Ok, json)));
            }

            return Ok(Response::with((status::InternalServerError,
                                      "couldn't convert records to JSON")));
        },
        Err(er) => { let err_mes = format!("get items command execute error {}",er);
                     return Ok(Response::with((status::InternalServerError, err_mes)));
        }
    }
}

pub fn insert_item(sdb: &Mutex<PostgresSqlData>, req: &mut Request) -> IronResult<Response> {
    let mut bd_data = sdb.lock().unwrap();

    let mut body = String::new();
    if let Err(er) = req.body.read_to_string(&mut body) {
        return Ok(Response::with((status::InternalServerError,
                                  "couldn't read request body")));
    }

    let mut add_item: PostgresInsertItem = serde_json::from_str(&body).expect("can't parse body");
    let mut commands: Vec<Box<PostgresCommand>> = vec![Box::new(add_item.make_valid()),
                                                       Box::new(PostgresGetItems::new())];

    if let Err(er) = bd_data.doCommands(&mut commands) {
        let err_mes = format!("insert item command execute error {}",er);
        return Ok(Response::with((status::InternalServerError, err_mes)));
    }

    let mut bget_item_types = commands.pop().unwrap();
    let mut aget = Box::leak(bget_item_types);
    if let Some(get_item_types) = aget.downcast_mut::<PostgresGetItems>() {
        if let Ok(json) = serde_json::to_string(get_item_types) {
            let content_type = Mime(TopLevel::Application, SubLevel::Json, Vec::new());
            return Ok(Response::with((content_type, status::Ok, json)));
        }
        return Ok(Response::with((status::InternalServerError,
                                  "couldn't convert records to JSON")));
    }

    panic!("couldn't downcast PostgresGetItems");
}
