pub extern crate postgres;

use self::postgres::{Connection, TlsMode};
use self::postgres::params::ConnectParams;
use self::postgres::error::Error;
use BdLayer::Settings;


#[derive(Debug)]
pub struct PostgresSqlData {
    _connection: Option<Connection>,
    _name: String,
}

impl PostgresSqlData {
    pub fn new() -> PostgresSqlData {
        PostgresSqlData { _connection: None, _name: "".to_string() }
    }
}

pub trait PostgresCommand {
    fn execute(&mut self, connect: &Connection) -> Result<(), Error> where Self: Sized;
}

pub trait PostgresDealer {

    /// Подключиться к БД (создать коннект).
    fn connect(&mut self) -> Result<(), Error>;

    /// Закрыть коннект к БД
    fn finish(&mut self) -> Result<(), Error>;

    /// Определелить активно ли подключение к БД.
    fn isOpen(&self) -> bool;

    /// Выполнить комманду
    fn doCommand<T: PostgresCommand>(&mut self, command: &mut T) -> Result<(),Error>;
}

impl PostgresDealer for PostgresSqlData
{
    /// Подключиться к БД (создать коннект).
    fn connect(&mut self) -> Result<(), Error> {
        if let Some(s) = &self._connection {
            panic!("Trying to open new connect when old is not closed Yet/");
        }

        Settings::initConfig().unwrap();
        let (connect_par, ssl_mode) = Settings::readConfig();

        if let Some(name) = connect_par.database() {
            self._name = name.to_string();
        };

        match Connection::connect(connect_par, ssl_mode) {
            Ok(connection) => {
                self._connection = Some(connection);
                return Ok(());
            },
            Err(e) => return Err(e),
        };
    }

    /// Закрыть коннект к БД
    fn finish(&mut self) -> Result<(), Error> {
        match self._connection.take() {
            Some(connect) => {
                match connect.finish() {
                    Ok(res) => res,
                    Err(err) => return Err(err)
                };
            },
            None => panic!("рун!!!"),
        };

        self._name = "".to_string();
        Ok(())
    }

    /// Определелить активно ли подключение к БД.
    fn isOpen(&self) -> bool {
        if let Some(s) = &self._connection {
            return true;
        }
        return false;
    }

    /// Выполнить комманду
    fn doCommand<T: PostgresCommand>(&mut self, command: &mut T) -> Result<(),Error> {
        if self.isOpen() == false {
            panic!("no connect to Bd");
        }

        match &mut self._connection {
            Some(ref mut c) => return command.execute(c),
            None => panic!("Trying to exec command to BD when there are no connection")
        };
    }
}
