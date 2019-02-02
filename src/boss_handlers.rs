use iron::*;
use iron::mime::{Mime, TopLevel, SubLevel};

use std::io::Read;
use std::sync::Mutex;

use BdLayer::BossCommands::*;
use BdLayer::PostgresDealer::*;
use fair_boss_drop_server::serde_json;

pub fn insert_boss(sdb: &Mutex<PostgresSqlData>, req: &mut Request) -> IronResult<Response> {
    let mut bd_data = sdb.lock().unwrap();

    let mut body = String::new();
    if let Err(_) = req.body.read_to_string(&mut body) {
        return Ok(Response::with((status::InternalServerError,
                                  "couldn't read request body")));
    }

    let insert_boss: PostgresInsertBoss = serde_json::from_str(&body).
        expect("can't parse body");

    let mut commands: Vec<Box<PostgresCommand>> = vec![
        Box::new(insert_boss), Box::new(PostgresGetBosses::new())];

    if let Err(er) = bd_data.doCommands(&mut commands) {
        let err_mes = format!("insert boss command execute error {}",er);
        return Ok(Response::with((status::InternalServerError, err_mes)));
    }

    let bget_result = commands.pop().unwrap();
    let aget = Box::leak(bget_result);
    if let Some(get_result) = aget.downcast_mut::<PostgresGetBosses>() {
        if let Ok(json) = serde_json::to_string(get_result.getBosses()) {
            let content_type = Mime(TopLevel::Application, SubLevel::Json, Vec::new());
            return Ok(Response::with((content_type, status::Ok, json)));
        }
    }

    return Ok(Response::with((status::InternalServerError,
                              "couldn't convert records to JSON")));
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
        Ok(_) => {
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
        Ok(_) => {
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
