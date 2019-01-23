use BdLayer::PostgresDealer::PostgresCommand;
use ::postgres::{Connection, error::Error};
use std::collections::LinkedList;
use ::serde_derive;

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromSql, ToSql)]
#[postgres(name="item_probability")]
pub struct ItemProbability {
    #[postgres(name="id")]
    pub _id: i32,
    #[postgres(name="probability")]
    pub _probability: f32,
}

impl ItemProbability {
    pub fn new(id: i32, probability: f32) -> ItemProbability {
        ItemProbability { _id: id, _probability: probability.into() }
    }
}

#[derive(Serialize)]
pub struct PostgresGetItemTypes {
    _items: LinkedList<String>,
}

impl PostgresGetItemTypes {
    pub fn new() -> PostgresGetItemTypes {
        PostgresGetItemTypes { _items: LinkedList::new() }
    }
}

impl PostgresCommand for PostgresGetItemTypes {
    fn execute(&mut self,connect: &Connection) -> Result<(),Error> {
        let trans = connect.transaction().unwrap();
        let statement = trans.prepare("SELECT id, label FROM item_types ORDER BY id ASC;").unwrap();
        match statement.query(&[]) {
            Ok(rows) => {    let mut iter = rows.iter();
                             while let Some(row) = iter.next() {
                                 self._items.push_back(row.get("label"));
                             }

                             trans.commit().unwrap();
                             return Ok(());
            },
            Err(er) =>  return Err(er);
        }
    }
}

#[derive(Deserialize)]
pub struct PostgresInsertItemType {
    _label: String,
}

impl PostgresInsertItemType {
    pub fn new(label: &str) -> PostgresInsertItemType {
        PostgresInsertItemType { _label: label.to_string() }
    }
}

impl PostgresCommand for PostgresInsertItemType {
    fn execute(&mut self,connect: &Connection) -> Result<(),Error> {
        let trans = connect.transaction().unwrap();
        let statement = trans.prepare("INSERT INTO item_types VALUES(default, $1);")
            .unwrap();

        match statement.query(&[&self._label]) {
            Ok(rows) => {    trans.commit().unwrap();
                             return Ok(());
            },
            Err(er) =>  return Err(er);
        }
    }
}

// select * from test as bt where id = ANY(select unnest(list) from test as ids where label = 'борщ');
