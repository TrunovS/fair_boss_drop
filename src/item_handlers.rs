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
        Ok(_) => {
            println!("get item_types");
            if let Ok(json) = serde_json::to_string(get_item_types.getItemTypes()) {
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
    if let Err(_) = req.body.read_to_string(&mut body) {
        return Ok(Response::with((status::InternalServerError,
                                  "couldn't read request body")));
    }

    let add_item_type: PostgresInsertItemType = serde_json::from_str(&body).
        expect("can't parse body");
    let mut commands: Vec<Box<PostgresCommand>> = vec![
        Box::new(add_item_type), Box::new(PostgresGetItemTypes::new())];

    if let Err(er) = bd_data.doCommands(&mut commands) {
        let err_mes = format!("insert item_type command execute error {}",er);
        return Ok(Response::with((status::InternalServerError, err_mes)));
    }

    let bget_item_types = commands.pop().unwrap();
    let aget = Box::leak(bget_item_types);
    if let Some(get_item_types) = aget.downcast_mut::<PostgresGetItemTypes>() {
        if let Ok(json) = serde_json::to_string(get_item_types.getItemTypes()) {
            let content_type = Mime(TopLevel::Application, SubLevel::Json, Vec::new());
            return Ok(Response::with((content_type, status::Ok, json)));
        }
    }

    return Ok(Response::with((status::InternalServerError,
                              "couldn't convert records to JSON")));
}

pub fn get_item(sdb: &Mutex<PostgresSqlData>, req: &mut Request) -> IronResult<Response> {
    let mut bd_data = sdb.lock().unwrap();

    let url: Url = req.url.clone();
    let path = url.path();
    let sid: &str = &path.iter().last().unwrap();
    let id;
    if let Ok(r) = sid.parse() {
        id = r;
    } else {
        return Ok(Response::with((status::BadRequest, "bad id")));
    }

    let mut get_item = PostgresGetItem::new().with_id(id);
    match bd_data.doCommand(&mut get_item) {
        Ok(_) => {
            println!("get item with id {}",id);
            if !get_item.isFound() {
                let err_mes = format!("get item command execute error, id not exists.");
                return Ok(Response::with((status::InternalServerError, err_mes)));
            }

            if let Ok(json) = serde_json::to_string(&get_item.getItem()) {
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




pub fn get_items(sdb: &Mutex<PostgresSqlData>, req: &mut Request) -> IronResult<Response> {
    let mut bd_data = sdb.lock().unwrap();

    let mut get_items = PostgresGetItems::new();
    match bd_data.doCommand(&mut get_items) {
        Ok(_) => {
            println!("get items");
            if let Ok(json) = serde_json::to_string(get_items.getItems()) {
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
    if let Err(_) = req.body.read_to_string(&mut body) {
        return Ok(Response::with((status::InternalServerError,
                                  "couldn't read request body")));
    }

    let add_item: PostgresInsertItem = serde_json::from_str(&body).expect("can't parse body");
    let mut commands: Vec<Box<PostgresCommand>> = vec![Box::new(add_item.make_valid()),
                                                       Box::new(PostgresGetItems::new())];

    if let Err(er) = bd_data.doCommands(&mut commands) {
        let err_mes = format!("insert item command execute error {}",er);
        return Ok(Response::with((status::InternalServerError, err_mes)));
    }

    let bget_items = commands.pop().unwrap();
    let aget = Box::leak(bget_items);
    if let Some(get_items) = aget.downcast_mut::<PostgresGetItems>() {
        if let Ok(json) = serde_json::to_string(get_items.getItems()) {
            let content_type = Mime(TopLevel::Application, SubLevel::Json, Vec::new());
            return Ok(Response::with((content_type, status::Ok, json)));
        }
        return Ok(Response::with((status::InternalServerError,
                                  "couldn't convert records to JSON")));
    }

    panic!("couldn't downcast PostgresGetItems");
}
