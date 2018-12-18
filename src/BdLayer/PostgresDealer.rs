pub extern crate postgres;
use self::postgres::{Connection, SslMode};
// mod BdDealer;



#[derive(Debug)]
struct PostgresSqlData {
    _connection: Option<Connection>,
    _name: String,
}

use BdLayer::PostgresDealer::postgres::error::{ConnectError,Error};
use BdLayer::PostgresCommands::PostgresCommand;

trait PostgresDealer {
    /// Подключиться к БД (создать коннект).
    fn connect(&mut self, name: &str) -> Result<(), ConnectError>;

    /// Закрыть коннект к БД
    fn finish(&mut self) -> Result<(), Error>;

    /// Определелить активно ли подключение к БД.
    fn isOpen(&self) -> bool;

    /// Выполнить комманду
    fn doCommand<T: PostgresCommand>(&mut self, command: Box<T>) -> Result<(),Error>;
}

impl PostgresDealer for PostgresSqlData
{
    /// Подключиться к БД (создать коннект).
    fn connect(&mut self, name: &str) -> Result<(), ConnectError> {
        if let Some(s) = &self._connection {
            panic!("Trying to open new connect when old is not closed Yet/");
        }

        self._name = name.to_string();
        match Connection::connect(name, &SslMode::None) {
            Ok(connection) => {
                self._connection = Some(connection);
                return Ok(());
            },
            Err(e) => return Err(e),
        };
    }

    /// Закрыть коннект к БД
    fn finish(&mut self) -> Result<(), Error> {
        let result = ();
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
        Ok(result)
    }

    /// Определелить активно ли подключение к БД.
    fn isOpen(&self) -> bool {
        if let Some(s) = &self._connection {
            return true;
        }
        return false;
    }

    /// Выполнить комманду
    fn doCommand<T: PostgresCommand>(&mut self, command: Box<T>) -> Result<(),Error> {
        if self.isOpen() == false {
            panic!("no connect to Bd");
        }

        match &mut self._connection {
            Some(ref mut c) => return command.execute(c),
            None => panic!("Trying to exec command to BD when there are no connection")
        };
    }
}
