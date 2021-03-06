use BdLayer::PostgresDealer::PostgresCommand;
use ::postgres::{transaction::Transaction, error::Error};

pub struct PostgresInitTables;
impl PostgresInitTables {
    pub fn new() -> PostgresInitTables {
        PostgresInitTables { }
    }
}

impl PostgresCommand for PostgresInitTables {
    fn execute(&mut self,transaction: &Transaction) -> Result<(),Error> {
        let nest_trans = transaction.transaction().unwrap();
        let res = nest_trans.batch_execute("
            CREATE TABLE IF NOT EXISTS item_types (
            id SERIAL PRIMARY KEY,
            label VARCHAR NOT NULL UNIQUE
            );

            CREATE TABLE IF NOT EXISTS items (
            id SERIAL PRIMARY KEY,
            label VARCHAR NOT NULL UNIQUE,
            type SERIAL REFERENCES item_types (id),
            exchangable BOOL,
            equals REAL
            );

DO $$ BEGIN
            CREATE TYPE item_probability AS (
            id INTEGER,
            probability REAL
            );
            EXCEPTION
               WHEN duplicate_object THEN null;
END $$;

            CREATE TABLE IF NOT EXISTS bosses (
            id SERIAL PRIMARY KEY,
            label VARCHAR NOT NULL UNIQUE,
            level INTEGER NOT NULL DEFAULT '0',
            drop item_probability[]
            );
           ");

        match res {
            Ok(var) => {
                nest_trans.commit().unwrap();
                return Ok(())
            },
            Err(er) => return Err(er),
        };
    }
}
