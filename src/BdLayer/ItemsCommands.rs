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
        match trans.query("SELECT id, label FROM item_types ORDER BY id ASC;") {
            Ok(row) => {    let mut iter = rows.iter();
                            while let Ok(row) = iter.next() {
                                self._items.append(row.get("label"));
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

// pub struct PostgresInsertItemTypes {
//     _items: LinkedList<String>,
// }

// impl PostgresInsertItemTypes {
//     pub fn new() -> PostgresInsertItemTypes {
//         PostgresInsertItemTypes { }
//     }

//     pub fn setData(&mut self, items: LinkedList<String>) {
//         self._items = items;
//     }
// }

// impl PostgresCommand for PostgresInsertItemTypes {
//     fn execute(&mut self,connect: &Connection) -> Result<CommandResult,Error> {
//         let trans = connect.transaction().unwrap();
//         let res = trans.execute("
//             SELECT id, label FROM item_types ORDER BY id ASC;
//            ");
//         trans.commit().unwrap();
//         match res {
//             Ok(v) => return Ok(CommandResult::HAS_DATA(true)),
//             Err(e) => return Err(e),
//         };
//     }
// }
