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
        let trans = connect.transaction().unwrap();
        let res = trans.batch_execute("
            CREATE TABLE IF NOT EXISTS item_types (
            id SERIAL PRIMARY KEY,
            label VARCHAR NOT NULL
            );

            CREATE TABLE IF NOT EXISTS items (
            id SERIAL PRIMARY KEY,
            label VARCHAR NOT NULL,
            type SERIAL REFERENCES item_types (id)
            );

DO $$ BEGIN
            CREATE TYPE item_probability AS (
            id INTEGER,
            probability NUMERIC(7,6)
            );
            EXCEPTION
               WHEN duplicate_object THEN null;
END $$;

            CREATE TABLE IF NOT EXISTS bosses (
            id SERIAL PRIMARY KEY,
            label VARCHAR NOT NULL,
            level INTEGER NOT NULL DEFAULT '0',
            drop item_probability[]
            );
           ");
        trans.commit().unwrap();
        res
    }
}
