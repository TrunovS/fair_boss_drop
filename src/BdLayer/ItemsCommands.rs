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
pub struct ItemRow {
    _id: i32,
    _label: String,
    _type: i32,
    _exchangable: bool,
    _equals: f32
}

pub struct PostgresGetItem {
    _id: i32,
    _item: Option<ItemRow>,
}

impl PostgresGetItem {
    pub fn new() -> PostgresGetItem {
        PostgresGetItem { _id: 0, _item: None }
    }
    pub fn with_id(mut self,id: i32) -> PostgresGetItem {
        self._id = id;
        self
    }
    pub fn isFound(&self) -> bool {
        self._item.is_some()
    }
    pub fn getItem(&self) -> Option<&ItemRow> {
        self._item.as_ref()
    }
}

impl PostgresCommand for PostgresGetItem {
    fn execute(&mut self,transaction: &Transaction) -> Result<(),Error> {
        let nest_trans = transaction.transaction().unwrap();
        let statement = nest_trans.prepare("SELECT * FROM items where id=$1;")
            .unwrap();

        match statement.query(&[&self._id]) {
            Ok(rows) => { let mut iter = rows.iter();
                          while let Some(row) = iter.next() {
                              let item = ItemRow { _id: row.get("id"),
                                                   _label: row.get("label"),
                                                   _type: row.get("type"),
                                                   _exchangable: row.get("exchangable"),
                                                   _equals: row.get("equals") };
                              self._item = Some(item);
                          }

                          nest_trans.commit().unwrap();
                          return Ok(());
            },
            Err(er) =>  Err(er)
        }
    }
}

pub struct PostgresGetItems {
    _items: LinkedList<ItemRow>,
}

impl PostgresGetItems {
    pub fn new() -> PostgresGetItems {
        PostgresGetItems { _items: LinkedList::new() }
    }
    pub fn getItems(&self) -> &LinkedList<ItemRow> {
        &self._items
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
                                                   _exchangable: row.get("exchangable"),
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
    _exchangable: bool,
    _equals: f32
}

impl PostgresInsertItem {
    pub fn make_valid(mut self) -> PostgresInsertItem {
        if !self._exchangable {
            self._equals = 0.0;
        }

        self
    }
}


impl PostgresCommand for PostgresInsertItem {
    fn execute(&mut self,transaction: &Transaction) -> Result<(),Error> {
        let nest_trans = transaction.transaction().unwrap();
        let statement = nest_trans.prepare("INSERT INTO items VALUES(default, $1, $2, $3, $4);")
            .unwrap();

        match statement.query(&[&self._label, &self._type,
                                &self._exchangable, &self._equals]) {
            Ok(rows) => {    nest_trans.commit().unwrap();
                             return Ok(());
            },
            Err(er) =>  Err(er)
        }
    }
}

// select * from test as bt where id = ANY(select unnest(list) from test as ids where label = 'борщ');
