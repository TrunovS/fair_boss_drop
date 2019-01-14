use BdLayer::PostgresDealer::{PostgresCommand, CommandResult};
use BdLayer::PostgresDealer::postgres::{Connection, error::Error};
use std::collections::LinkedList;

pub struct PostgresGetItemTypes {
    _items: LinkedList<String>,
}

impl PostgresGetItemTypes {
    pub fn new() -> PostgresGetItemTypes {
        PostgresGetItemTypes { _items: LinkedList::new() }
    }

    pub fn getData(&self) -> &LinkedList<String> {
        return &self._items;
    }
}

impl PostgresCommand for PostgresGetItemTypes {
    fn execute(&mut self,connect: &Connection) -> Result<CommandResult,Error> {
        let trans = connect.transaction().unwrap();
        let statement = trans.prepare("SELECT id, label FROM item_types ORDER BY id ASC;").unwrap();
        match statement.query(&[]) {
            Ok(rows) => {    let mut iter = rows.iter();
                             while let Some(row) = iter.next() {
                                 self._items.push_back(row.get("label"));
                             }

                             trans.commit().unwrap();
                             return Ok(CommandResult::HAS_DATA(true));
            },
            Err(er) =>  {
                trans.commit().unwrap();
                return Err(er);
            }
        }
    }
}

pub struct PostgresInsertItemTypes {
    _label: String,
}

impl PostgresInsertItemTypes {
    pub fn new() -> PostgresInsertItemTypes {
        PostgresInsertItemTypes { _label: String::new("") }
    }

    pub fn setData(&mut self, label: String) {
        self._label = label;
    }
}

impl PostgresCommand for PostgresInsertItemTypes {
    fn execute(&mut self,connect: &Connection) -> Result<CommandResult,Error> {
        let trans = connect.transaction().unwrap();
        let statement = trans.prepare("INSERT INTO item_types VALUES(default, $1);").unwrap();
        match statement.query(&[self._label]) {
            Ok(rows) => {    let mut iter = rows.iter();
                             while let Some(row) = iter.next() {
                                 self._items.push_back(row.get("label"));
                             }

                             trans.commit().unwrap();
                             return Ok(CommandResult::HAS_DATA(true));
            },
            Err(er) =>  {
                trans.commit().unwrap();
                return Err(er);
            }
        }
    }
}
