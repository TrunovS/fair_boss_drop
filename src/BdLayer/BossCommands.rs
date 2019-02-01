use BdLayer::PostgresDealer::PostgresCommand;
use ::postgres::{transaction::Transaction, error::Error};
use std::collections::LinkedList;
use BdLayer::ItemsCommands::ItemProbability;

enum GetBossBy {
    ID,
    LABEL,
    NONE
}

#[derive(Serialize)]
pub struct BossRow {
    _id: i32,
    _label: String,
    _level: i32,
    _drop: Vec<ItemProbability>,
}

pub struct PostgresGetBoss {
    _boss: Option<BossRow>,
    _id: Option<i32>,
    _label: Option<String>,
    _opt: GetBossBy,
}

impl PostgresGetBoss {
    pub fn new() -> PostgresGetBoss {
        PostgresGetBoss { _boss: None, _label: None, _id: None, _opt: GetBossBy::NONE }
    }
    pub fn with_label(mut self,label: &str) -> PostgresGetBoss {
        self._label = Some(String::from(label));
        self._id = None;
        self._opt = GetBossBy::LABEL;
        self
    }
    pub fn with_id(mut self,id: i32) -> PostgresGetBoss {
        self._label = None;
        self._id = Some(id);
        self._opt = GetBossBy::ID;
        self
    }
    pub fn getBoss(&self) -> &Option<BossRow> {
        &self._boss
    }
}

impl PostgresCommand for PostgresGetBoss {
    fn execute(&mut self,connect: &Transaction) -> Result<(),Error> {
        let trans = connect.transaction().unwrap();
        match &self._opt {
            GetBossBy::ID => {
                let statement = trans.prepare("SELECT * FROM bosses WHERE id=$1;").unwrap();
                match statement.query(&[&self._id]) {
                    Ok(rows) => {    let mut iter = rows.iter();
                                     while let Some(row) = iter.next() {
                                         self._boss = Some(BossRow { _id: row.get("id"),
                                                                _label: row.get("label"),
                                                                _level: row.get("level"),
                                                                _drop: row.get("drop") });
                                     }

                                     trans.commit().unwrap();
                                     return Ok(());
                    },
                    Err(er) =>  Err(er)
                }


            },
            GetBossBy::LABEL => {
                let statement = trans.prepare("SELECT * FROM bosses WHERE label=$1;").unwrap();
                match statement.query(&[&self._label]) {
                    Ok(rows) => {    let mut iter = rows.iter();
                                     while let Some(row) = iter.next() {
                                         self._boss = Some(BossRow { _id: row.get("id"),
                                                                 _label: row.get("label"),
                                                                 _level: row.get("level"),
                                                                 _drop: row.get("drop") });
                                     }

                                     trans.commit().unwrap();
                                     return Ok(());
                    },
                    Err(er) =>  Err(er)
                }
            },
            _ => { println!("cant decide command context (GetBossBy::LABEL or GetBossBy::ID)");
                   return Ok(());
            }
        }
    }
}



#[derive(Serialize)]
pub struct ShortBossRow {
    _id: i32,
    _label: String,
}

pub struct PostgresGetBosses {
    _bosses: LinkedList<ShortBossRow>
}

impl PostgresGetBosses {
    pub fn new() -> PostgresGetBosses {
        PostgresGetBosses { _bosses: LinkedList::new() }
    }
    pub fn getBosses(&self) -> &LinkedList<ShortBossRow> {
        &self._bosses
    }
}

impl PostgresCommand for PostgresGetBosses {
    fn execute(&mut self,connect: &Transaction) -> Result<(),Error> {
        let trans = connect.transaction().unwrap();
        let statement = trans.prepare("SELECT id, label FROM bosses;").unwrap();
        match statement.query(&[]) {
            Ok(rows) => {  let mut iter = rows.iter();
                           while let Some(row) = iter.next() {
                               let boss = ShortBossRow { _id: row.get("id"),
                                                          _label: row.get("label") };
                               self._bosses.push_back(boss);
                           }

                           trans.commit().unwrap();
                           return Ok(());
            },
            Err(er) =>  Err(er)
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
    fn execute(&mut self,connect: &Transaction) -> Result<(),Error> {
        let trans = connect.transaction().unwrap();

        let statement = trans.prepare("INSERT INTO bosses VALUES(default, $1, $2, $3);").unwrap();

        match statement.query(&[&self._label,&self._level,&self._drop]) {
            Ok(rows) => {    trans.commit().unwrap();
                             return Ok(());
            },
            Err(er) =>  Err(er)
        }
    }
}
