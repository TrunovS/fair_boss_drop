// pub mod PostgresDealer;

use BdLayer::PostgresDealer::postgres::Connection;
use BdLayer::PostgresDealer::postgres::error::Error;

pub trait PostgresCommand {
    fn execute(&self, connect: &Connection) -> Result<(), Error> where Self: Sized;
}

pub struct PostgresInitTables;
impl PostgresInitTables {
    pub fn new() -> PostgresInitTables {
        PostgresInitTables { }
    }
}

impl PostgresCommand for PostgresInitTables {
    fn execute(&self,connect: &Connection) -> Result<(),Error> {
        connect.batch_execute("
            CREATE TABLE person (
            id SERIAL PRIMARY KEY,
            name NOT NULL
            );

            CREATE TABLE purchase (
            id SERIAL PRIMARY KEY,
            person INT NOT NULL REFERENCES person (id),
            time TIMESTAMPTZ NOT NULL,
            );

            CREATE INDEX ON purchase (time);
           ")
    }
}
