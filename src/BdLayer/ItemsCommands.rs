use BdLayer::PostgresDealer::PostgresCommand;
use ::postgres::{transaction::Transaction, error::Error};
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
struct ItemTypeRow {
    _id: i32,
    _label: String,
}

#[derive(Serialize)]
pub struct PostgresGetItemTypes {
    _items: LinkedList<ItemTypeRow>,
}

impl PostgresGetItemTypes {
    pub fn new() -> PostgresGetItemTypes {
        PostgresGetItemTypes { _items: LinkedList::new() }
    }
}

impl PostgresCommand for PostgresGetItemTypes {
    fn execute(&mut self,transaction: &Transaction) -> Result<(),Error> {
        let nest_trans = transaction.transaction().unwrap();
        let statement = nest_trans.prepare("SELECT id, label FROM item_types ORDER BY id ASC;")
            .unwrap();

        match statement.query(&[]) {
            Ok(rows) => {    let mut iter = rows.iter();
                             while let Some(row) = iter.next() {
                                 let item = ItemTypeRow { _id: row.get("id"),
                                                          _label: row.get("label") };
                                 self._items.push_back(item);
                             }

                             nest_trans.commit().unwrap();
                             return Ok(());
            },
            Err(er) =>  Err(er)
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
    fn execute(&mut self,transaction: &Transaction) -> Result<(),Error> {
        let nest_trans = transaction.transaction().unwrap();
        let statement = nest_trans.prepare("INSERT INTO item_types VALUES(default, $1);")
            .unwrap();

        match statement.query(&[&self._label]) {
            Ok(rows) => {    nest_trans.commit().unwrap();
                             return Ok(());
            },
            Err(er) =>  Err(er)
        }
    }
}


#[derive(Serialize)]
struct ItemRow {
    _id: i32,
    _label: String,
    _type: i32,
    _equals: f32
}

#[derive(Serialize)]
pub struct PostgresGetItems {
    _items: LinkedList<ItemRow>,
}

impl PostgresGetItems {
    pub fn new() -> PostgresGetItems {
        PostgresGetItems { _items: LinkedList::new() }
    }
}

impl PostgresCommand for PostgresGetItems {
    fn execute(&mut self,transaction: &Transaction) -> Result<(),Error> {
        let nest_trans = transaction.transaction().unwrap();
        let statement = nest_trans.prepare("SELECT * FROM items;")
            .unwrap();

        match statement.query(&[]) {
            Ok(rows) => { let mut iter = rows.iter();
                          while let Some(row) = iter.next() {
                              let item = ItemRow { _id: row.get("id"),
                                                   _label: row.get("label"),
                                                   _type: row.get("type"),
                                                   _equals: row.get("equals") };
                              self._items.push_back(item);
                          }

                          nest_trans.commit().unwrap();
                          return Ok(());
            },
            Err(er) =>  Err(er)
        }
    }
}



#[derive(Deserialize)]
pub struct PostgresInsertItem {
    _label: String,
    _type: i32,
    _equals: Option<f32>
}

impl PostgresInsertItem {
    pub fn new(label: &str,itype: i32,equals: Option<f32>) -> PostgresInsertItem {
        PostgresInsertItem { _label: String::from(label),
                              _type: itype,
                              _equals: equals }
    }
}

impl PostgresCommand for PostgresInsertItem {
    fn execute(&mut self,transaction: &Transaction) -> Result<(),Error> {
        let nest_trans = transaction.transaction().unwrap();
        let statement = nest_trans.prepare("INSERT INTO items VALUES(default, $1, $2, $3);")
            .unwrap();

        match statement.query(&[&self._label, &self._type, &self._equals]) {
            Ok(rows) => {    nest_trans.commit().unwrap();
                             return Ok(());
            },
            Err(er) =>  Err(er)
        }
    }
}

// select * from test as bt where id = ANY(select unnest(list) from test as ids where label = 'борщ');
