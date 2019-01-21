use BdLayer::PostgresDealer::PostgresCommand;
use ::postgres::{Connection, error::Error};
use std::collections::LinkedList;
use BdLayer::ItemsCommands::ItemProbability;

enum GetBossBy {
    ID,
    LABEL,
    NONE
}

pub struct PostgresGetBoss {
    _id: Option<u32>,
    _label: Option<String>,
    _level: Option<i32>,
    _drop: Option<Vec<ItemProbability>>,
    _opt: GetBossBy,
}

impl PostgresGetBoss {
    pub fn new() -> PostgresGetBoss {
        PostgresGetBoss { _id: None, _label: None, _level: None, _drop: None, _opt: GetBossBy::NONE }
    }
    pub fn with_label(mut self,label: &str) -> PostgresGetBoss {
        self._label = Some(String::from(label));
        self._id = None;
        self._opt = GetBossBy::LABEL;
        self
    }
    pub fn with_id(mut self,id: u32) -> PostgresGetBoss {
        self._label = None;
        self._id = Some(id);
        self._opt = GetBossBy::ID;
        self
    }
}

impl PostgresCommand for PostgresGetBoss {
    fn execute(&mut self,connect: &Connection) -> Result<(),Error> {
        let trans = connect.transaction().unwrap();
        match &self._opt {
            GetBossBy::ID => {
                let statement = trans.prepare("SELECT * FROM bosses WHERE id=$1;").unwrap();
                match statement.query(&[&self._id]) {
                    Ok(rows) => {    let mut iter = rows.iter();
                                     while let Some(row) = iter.next() {
                                         self._label = Some(row.get("label"));
                                         self._level = Some(row.get("level"));
                                         self._drop = Some(row.get("drop"));

                                         println!("{:?}",self._drop);
                                     }

                                     trans.commit().unwrap();
                                     return Ok(());
                    },
                    Err(er) =>  {
                        trans.commit().unwrap();
                        return Err(er);
                    }
                }


            },
            GetBossBy::LABEL => {
                let statement = trans.prepare("SELECT * FROM bosses WHERE label=$1;").unwrap();
                match statement.query(&[&self._label]) {
                    Ok(rows) => {    let mut iter = rows.iter();
                                     while let Some(row) = iter.next() {
                                         self._id = Some(row.get("id"));
                                         self._level = Some(row.get("level"));
                                         self._drop = Some(row.get("drop"));

                                         println!("{:?}",self._drop);
                                     }

                                     trans.commit().unwrap();
                                     return Ok(());
                    },
                    Err(er) =>  {
                        trans.commit().unwrap();
                        return Err(er);
                    }
                }
            },
            _ => { println!("cant decide command context (GetBossBy::LABEL or GetBossBy::ID)");
                   return Ok(());
            }
        }
    }

    // pub fn getData(&self) -> &LinkedList<String> {
    //     return &self._bosses;
    // }
}

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

pub struct PostgresInsertBoss {
    _label: String,
    _level: i32,
    _drop: Vec<ItemProbability>,
}

impl PostgresInsertBoss {
    pub fn new(label: &str, level: i32, drop: Vec<ItemProbability>) -> PostgresInsertBoss {
        PostgresInsertBoss { _label: label.to_string(), _level: level, _drop: drop }
    }
}

impl PostgresCommand for PostgresInsertBoss {
    fn execute(&mut self,connect: &Connection) -> Result<(),Error> {
        let trans = connect.transaction().unwrap();

        let statement = trans.prepare("INSERT INTO bosses VALUES(default, $1, $2, $3);").unwrap();

        match statement.query(&[&self._label,&self._level,&self._drop]) {
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
