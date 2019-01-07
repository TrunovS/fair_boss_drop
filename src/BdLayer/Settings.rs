extern crate ini;
use self::ini::Ini;
use std::io;
use std::path::Path;
use std::str::FromStr;

use BdLayer::PostgresDealer::postgres::{ConnectParams, ConnectTarget, UserInfo, SslMode};

const cfg_file: &'static str = "db_conf.ini";

pub fn initConfig() -> Result<(),io::Error> {
    if Path::new(cfg_file).exists() == true {
        return Ok(());
    }

    let mut conf = Ini::new();
    conf.with_section(None)
        .set("encoding", "utf-8")
        .set("host", "localhost")
        .set("port", "5432")
        .set("sslmode", "disable")
        .set("dbname", "postgres")
        .set("user", "postres")
        .set("pass", "qwerty");

    return conf.write_to_file(cfg_file);
}


/// Прочитать конфиг подключения
pub fn readConfig() -> (ConnectParams, SslMode) {
    let conf = Ini::load_from_file(cfg_file).unwrap();
    let general = conf.general_section();

    let host = general.get("host").unwrap();
    let port = general.get("port").unwrap();
    let sslmode = general.get("sslmode").unwrap();
    let dbname = general.get("dbname").unwrap();
    let user = general.get("user").unwrap();
    let pass = general.get("pass").unwrap();

    let s = match sslmode.as_ref() {
        "disable" => SslMode::None,
        "enable" => unimplemented!(),
        _ => panic!("Wrong sslmode"),
    };

    (ConnectParams {
        target: ConnectTarget::Tcp(host.clone()),
        port: Some(FromStr::from_str(port).unwrap()),
        user: Some(UserInfo {
            user: user.clone(),
            password: Some(pass.clone()),
        }),
        database: Some(dbname.clone()),
        options: vec![],
    },
     s)
}
