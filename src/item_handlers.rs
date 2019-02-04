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
    if let Err(er) = bd_data.doCommand(&mut get_item_types) {
        let err_mes = format!("get item_types command execute error {}",er);
        return Ok(Response::with((status::InternalServerError, err_mes)));
    }

    if let Ok(json) = serde_json::to_string(get_item_types.getItemTypes()) {
        let content_type = Mime(TopLevel::Application, SubLevel::Json, Vec::new());
        return Ok(Response::with((content_type, status::Ok, json)));
    }

    Ok(Response::with((status::InternalServerError,"couldn't convert records to JSON")))
}

pub fn insert_item_type(sdb: &Mutex<PostgresSqlData>, req: &mut Request) -> IronResult<Response> {
    let mut bd_data = sdb.lock().unwrap();

    let mut body = String::new();
    if let Err(_) = req.body.read_to_string(&mut body) {
        return Ok(Response::with((status::BadRequest,"couldn't read request body")));
    }

    let add_item_type: PostgresInsertItemType;
    match serde_json::from_str(&body) {
        Ok(res) => add_item_type = res,
        Err(_) =>  return Ok(Response::with((status::NotAcceptable,
                                             "can't deserialize body"))),
    }

    let mut commands: Vec<Box<PostgresCommand>> = vec![
        Box::new(add_item_type), Box::new(PostgresGetItemTypes::new())];

    if let Err(er) = bd_data.doCommands(&mut commands) {
        let err_mes = format!("insert item_type command execute error {}",er);
        return Ok(Response::with((status::InternalServerError, err_mes)));
    }

    let bget_item_types = commands.pop().unwrap();
    let aget = Box::leak(bget_item_types);
    if let Some(get_item_types) = aget.downcast_mut::<PostgresGetItemTypes>()
    {
        if let Ok(json) = serde_json::to_string(get_item_types.getItemTypes())  {
            let content_type = Mime(TopLevel::Application, SubLevel::Json, Vec::new());
            return Ok(Response::with((content_type, status::Created, json)));
        }
        return Ok(Response::with((status::InternalServerError,
                                  "couldn't convert records to JSON")));
    }

    Ok(Response::with((status::InternalServerError,"some error happened :(")))
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
    if let Err(er) = bd_data.doCommand(&mut get_item) {
        let err_mes = format!("get items command execute error:\n {}",er);
        return Ok(Response::with((status::InternalServerError, err_mes)));
    }

    if get_item.getItem().is_none() {
        let err_mes = format!("id={} - doesn't exist",id);
        return Ok(Response::with((status::BadRequest, err_mes)));
    }

    if let Ok(json) = serde_json::to_string(&get_item.getItem()) {
        let content_type = Mime(TopLevel::Application, SubLevel::Json, Vec::new());
        return Ok(Response::with((content_type, status::Ok, json)));
    }

    Ok(Response::with((status::InternalServerError,"couldn't convert records to JSON")))
}

pub fn get_items(sdb: &Mutex<PostgresSqlData>, req: &mut Request) -> IronResult<Response> {
    let mut bd_data = sdb.lock().unwrap();

    let mut get_items = PostgresGetItems::new();
    if let Err(er) = bd_data.doCommand(&mut get_items) {
        let err_mes = format!("get items command execute error:\n {}",er);
        return Ok(Response::with((status::InternalServerError, err_mes)));
    }

    if let Ok(json) = serde_json::to_string(get_items.getItems()) {
        let content_type = Mime(TopLevel::Application, SubLevel::Json, Vec::new());
        return Ok(Response::with((content_type, status::Ok, json)));
    }

    Ok(Response::with((status::InternalServerError,"couldn't convert records to JSON")))
}

pub fn insert_item(sdb: &Mutex<PostgresSqlData>, req: &mut Request) -> IronResult<Response> {
    let mut bd_data = sdb.lock().unwrap();

    let mut body = String::new();
    if let Err(_) = req.body.read_to_string(&mut body) {
        return Ok(Response::with((status::BadRequest,"couldn't read request body")));
    }

    let add_item: PostgresInsertItem;
    match serde_json::from_str(&body) {
        Ok(res) => add_item = res,
        Err(_) =>  return Ok(Response::with((status::NotAcceptable,
                                             "couldn't deserialize body"))),
    }

    let mut commands: Vec<Box<PostgresCommand>> = vec![Box::new(add_item.make_valid()),
                                                       Box::new(PostgresGetItems::new())];

    if let Err(er) = bd_data.doCommands(&mut commands) {
        let err_mes = format!("insert item command execute error:\n {}",er);
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

    Ok(Response::with((status::InternalServerError,"some error happened :(")))
}
