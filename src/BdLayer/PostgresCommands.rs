// pub mod PostgresDealer;

use BdLayer::PostgresDealer::postgres::{Connection, error::Error};

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
            CREATE TABLE subperson (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL );

            CREATE TABLE person (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL );
           ")
    }
}
