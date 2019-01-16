use BdLayer::PostgresDealer::PostgresCommand;
use BdLayer::PostgresDealer::postgres::{Connection, error::Error};
use std::collections::LinkedList;

pub struct PostgresGetBosses {
    _bosses: LinkedList<String>
}

impl PostgresGetBosses {
    pub fn new() -> PostgresGetBosses {
        PostgresGetBosses { _bosses: LinkedList::new() }
    }
    pub fn getData(&self) -> &LinkedList<String> {
        return &self._bosses;
    }
}

impl PostgresCommand for PostgresGetBosses {
    fn execute(&mut self,connect: &Connection) -> Result<(),Error> {
        let trans = connect.transaction().unwrap();
        let statement = trans.prepare("SELECT id, label FROM bosses ORDER BY id ASC;").unwrap();
        match statement.query(&[]) {
            Ok(rows) => {    let mut iter = rows.iter();
                             while let Some(row) = iter.next() {
                                 self._bosses.push_back(row.get("label"));
                             }

                             trans.commit().unwrap();
                             return Ok(());
            },
            Err(er) =>  {
                trans.commit().unwrap();
                return Err(er);
            }
        }
    }
}

pub struct item_probability {
    pub _id: i32,
    pub _probability: f32,
}

pub struct PostgresInsertBoss {
    _label: String,
    _level: i32,
    _drop: LinkedList<item_probability>,
}

impl PostgresInsertBoss {
    pub fn new(label: &str, level: i32, drop: LinkedList<item_probability>) -> PostgresInsertBoss {
        PostgresInsertBoss { _label: label.to_string(), _level: level, _drop: drop }
    }
    fn convertDropToSql(&self) -> String {
        let mut sql = String::from("ARRAY[ ");

        for el in self._drop.iter() {
            sql.push_str(&format!("cast(({}, {}) as item_probability),", el._id, el._probability));
        }
        sql.pop();
        sql.push_str(" ]");
        println!("{}",sql);
        sql
    }
}

impl PostgresCommand for PostgresInsertBoss {
    fn execute(&mut self,connect: &Connection) -> Result<(),Error> {
        let trans = connect.transaction().unwrap();
        let dbg = format!("INSERT INTO bosses VALUES(default, {}, {}, {});",self._label,self._level, self.convertDropToSql());
        println!("{}",dbg);
        let statement = trans.prepare("INSERT INTO bosses VALUES(default, $1, $2, $3);")
            .unwrap();

        match statement.query(&[&self._label, &self._level, &self.convertDropToSql()]) {
            Ok(rows) => {    trans.commit().unwrap();
                             return Ok(());
            },
            Err(er) =>  {
                trans.commit().unwrap();
                return Err(er);
            }
        }
    }
}
