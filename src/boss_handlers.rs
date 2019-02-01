use iron::*;
use iron::mime::{Mime, TopLevel, SubLevel};

use std::io::Read;
use std::sync::Mutex;

use BdLayer::BossCommands::*;
use BdLayer::PostgresDealer::*;
use BdLayer::ItemsCommands::ItemProbability;
use fair_boss_drop_server::serde_json;

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

    let url: Url = req.url.clone();
    let path = url.path();
    let sid: &str = &path.iter().last().unwrap();
    let id;
    if let Ok(r) = sid.parse() {
        id = r;
    } else {
        return Ok(Response::with((status::BadRequest, "bad id")));
    }

    let mut get_boss = PostgresGetBoss::new().with_id(id);
    match bd_data.doCommand(&mut get_boss) {
        Ok(res) => {
            println!("get boss");
            if get_boss.getBoss().is_none() {
                let err_mes = format!("No boss found");
                return Ok(Response::with((status::InternalServerError, err_mes)));
            }

            if let Ok(json) = serde_json::to_string(&get_boss.getBoss()) {
                let content_type = Mime(TopLevel::Application, SubLevel::Json, Vec::new());
                return Ok(Response::with((content_type, status::Ok, json)));
            }

            return Ok(Response::with((status::InternalServerError,
                                      "couldn't convert records to JSON")));
        },
        Err(er) => { let err_mes = format!("get boss command execute error {}",er);
                     return Ok(Response::with((status::InternalServerError, err_mes)));
        }
    }
}

pub fn get_bosses(sdb: &Mutex<PostgresSqlData>, req: &mut Request) -> IronResult<Response> {
    let mut bd_data = sdb.lock().unwrap();

    let mut get_bosses = PostgresGetBosses::new();
    match bd_data.doCommand(&mut get_bosses) {
        Ok(res) => {
            println!("get bosses");
            if let Ok(json) = serde_json::to_string(&get_bosses.getBosses()) {
                let content_type = Mime(TopLevel::Application, SubLevel::Json, Vec::new());
                return Ok(Response::with((content_type, status::Ok, json)));
            }

            return Ok(Response::with((status::InternalServerError,
                                      "couldn't convert records to JSON")));
        },
        Err(er) => { let err_mes = format!("get bosses command execute error {}",er);
                     return Ok(Response::with((status::InternalServerError, err_mes)));
        }
    }
}
