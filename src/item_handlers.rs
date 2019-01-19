use iron::*;
use iron::mime::{Mime, TopLevel, SubLevel};

use std::io::Read;
use std::sync::Mutex;

use BdLayer::ItemsCommands::*;
use BdLayer::PostgresDealer::*;


pub fn get_item_types(sdb: &Mutex<PostgresSqlData>, req: &mut Request) -> IronResult<Response> {
    let mut bd_data = sdb.lock().unwrap();

    let mut get_item_types = PostgresGetItemTypes::new();
    match bd_data.doCommand(&mut get_item_types) {
        Ok(res) => {  println!("get item types"); },
        Err(er) => {  println!("{}",er); }
    }
    println!("start {:?}",get_item_types.getData());

    Ok(Response::with((status::Ok,"Get Item Types executed")))
}

pub fn insert_item_type(sdb: &Mutex<PostgresSqlData>, req: &mut Request) -> IronResult<Response> {
    let mut bd_data = sdb.lock().unwrap();

    let mut add_item_type = PostgresInsertItemType::new("tip1");
    match bd_data.doCommand(&mut add_item_type) {
        Ok(res) => {  println!("item added"); },
        Err(er) => {  println!("{}",er); }
    };
    Ok(Response::with((status::Ok,"insert item type executed")))
}
