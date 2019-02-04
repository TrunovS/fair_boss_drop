use BdLayer::PostgresDealer::PostgresCommand;
use ::postgres::{transaction::Transaction, error::Error};
use std::collections::LinkedList;
use ::serde_derive;

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromSql, ToSql)]
#[postgres(name="item_probability")]
pub struct ItemProbability {
    pub id: i32,
    pub probability: f32,
}

impl ItemProbability {
    pub fn new(id: i32, probability: f32) -> ItemProbability {
        ItemProbability { id: id, probability: probability.into() }
    }
}

#[derive(Serialize)]
pub struct ItemTypeRow {
    id: i32,
    label: String,
}

#[derive(Serialize)]
pub struct PostgresGetItemTypes {
    payload: LinkedList<ItemTypeRow>,
}

impl PostgresGetItemTypes {
    pub fn new() -> PostgresGetItemTypes {
        PostgresGetItemTypes { payload: LinkedList::new() }
    }
    pub fn getPayload(&self) -> &LinkedList<ItemTypeRow> {
        &self.payload
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
                                 let item = ItemTypeRow { id: row.get("id"),
                                                          label: row.get("label") };
                                 self.payload.push_back(item);
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
    label: String,
}

impl PostgresInsertItemType {
    pub fn new(label: &str) -> PostgresInsertItemType {
        PostgresInsertItemType { label: label.to_string() }
    }
}

impl PostgresCommand for PostgresInsertItemType {
    fn execute(&mut self,transaction: &Transaction) -> Result<(),Error> {
        let nest_trans = transaction.transaction().unwrap();
        let statement = nest_trans.prepare("INSERT INTO item_types VALUES(default, $1);")
            .unwrap();

        match statement.query(&[&self.label]) {
            Ok(rows) => {    nest_trans.commit().unwrap();
                             return Ok(());
            },
            Err(er) =>  Err(er)
        }
    }
}


#[derive(Serialize)]
pub struct ItemRow {
    id: i32,
    label: String,
    #[serde(rename = "type")]
    item_type: i32,
    exchangable: bool,
    equals: f32
}

#[derive(Serialize)]
pub struct PostgresGetItem {
    payload: Option<ItemRow>,
    id: i32,
}

impl PostgresGetItem {
    pub fn new() -> PostgresGetItem {
        PostgresGetItem { id: 0, payload: None }
    }
    pub fn with_id(mut self,id: i32) -> PostgresGetItem {
        self.id = id;
        self
    }
    pub fn getPayload(&self) -> Option<&ItemRow> {
        self.payload.as_ref()
    }
}

impl PostgresCommand for PostgresGetItem {
    fn execute(&mut self,transaction: &Transaction) -> Result<(),Error> {
        let nest_trans = transaction.transaction().unwrap();
        let statement = nest_trans.prepare("SELECT * FROM items where id=$1;")
            .unwrap();

        match statement.query(&[&self.id]) {
            Ok(rows) => { let mut iter = rows.iter();
                          while let Some(row) = iter.next() {
                              let item = ItemRow { id: row.get("id"),
                                                   label: row.get("label"),
                                                   item_type: row.get("type"),
                                                   exchangable: row.get("exchangable"),
                                                   equals: row.get("equals") };
                              self.payload = Some(item);
                          }

                          nest_trans.commit().unwrap();
                          return Ok(());
            },
            Err(er) =>  Err(er)
        }
    }
}

pub struct PostgresGetItems {
    payload: LinkedList<ItemRow>,
}

impl PostgresGetItems {
    pub fn new() -> PostgresGetItems {
        PostgresGetItems { payload: LinkedList::new() }
    }
    pub fn getPayload(&self) -> &LinkedList<ItemRow> {
        &self.payload
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
                              let item = ItemRow { id: row.get("id"),
                                                   label: row.get("label"),
                                                   item_type: row.get("type"),
                                                   exchangable: row.get("exchangable"),
                                                   equals: row.get("equals") };
                              self.payload.push_back(item);
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
    label: String,
    #[serde(rename = "type")]
    item_type: i32,
    exchangable: bool,
    equals: f32
}

impl PostgresInsertItem {
    pub fn make_valid(mut self) -> PostgresInsertItem {
        if !self.exchangable {
            self.equals = 0.0;
        }

        self
    }
}


impl PostgresCommand for PostgresInsertItem {
    fn execute(&mut self,transaction: &Transaction) -> Result<(),Error> {
        let nest_trans = transaction.transaction().unwrap();
        let statement = nest_trans.prepare("INSERT INTO items VALUES(default, $1, $2, $3, $4);")
            .unwrap();

        match statement.query(&[&self.label, &self.item_type,
                                &self.exchangable, &self.equals]) {
            Ok(rows) => {    nest_trans.commit().unwrap();
                             return Ok(());
            },
            Err(er) =>  Err(er)
        }
    }
}

// select * from test as bt where id = ANY(select unnest(list) from test as ids where label = 'борщ');
