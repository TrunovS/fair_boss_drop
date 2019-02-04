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
    id: i32,
    label: String,
    level: i32,
    drop: Vec<ItemProbability>,
}


#[derive(Serialize)]
pub struct PostgresGetBoss {
    payload: Option<BossRow>,
    #[serde(skip)]
    id: Option<i32>,
    #[serde(skip)]
    label: Option<String>,
    #[serde(skip)]
    opt: GetBossBy,
}

impl PostgresGetBoss {
    pub fn new() -> PostgresGetBoss {
        PostgresGetBoss { payload: None, label: None, id: None, opt: GetBossBy::NONE }
    }
    pub fn with_label(mut self,label: &str) -> PostgresGetBoss {
        self.label = Some(String::from(label));
        self.id = None;
        self.opt = GetBossBy::LABEL;
        self
    }
    pub fn with_id(mut self,id: i32) -> PostgresGetBoss {
        self.label = None;
        self.id = Some(id);
        self.opt = GetBossBy::ID;
        self
    }
    pub fn getPayload(&self) -> &Option<BossRow> {
        &self.payload
    }
}

impl PostgresCommand for PostgresGetBoss {
    fn execute(&mut self,connect: &Transaction) -> Result<(),Error> {
        let trans = connect.transaction().unwrap();
        match &self.opt {
            GetBossBy::ID => {
                let statement = trans.prepare("SELECT * FROM bosses WHERE id=$1;").unwrap();
                match statement.query(&[&self.id]) {
                    Ok(rows) => {    let mut iter = rows.iter();
                                     while let Some(row) = iter.next() {
                                         self.payload = Some(BossRow { id: row.get("id"),
                                                                label: row.get("label"),
                                                                level: row.get("level"),
                                                                drop: row.get("drop") });
                                     }

                                     trans.commit().unwrap();
                                     return Ok(());
                    },
                    Err(er) =>  Err(er)
                }


            },
            GetBossBy::LABEL => {
                let statement = trans.prepare("SELECT * FROM bosses WHERE label=$1;").unwrap();
                match statement.query(&[&self.label]) {
                    Ok(rows) => {    let mut iter = rows.iter();
                                     while let Some(row) = iter.next() {
                                         self.payload = Some(BossRow { id: row.get("id"),
                                                                 label: row.get("label"),
                                                                 level: row.get("level"),
                                                                 drop: row.get("drop") });
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
    id: i32,
    label: String,
}

#[derive(Serialize)]
pub struct PostgresGetBosses {
    payload: LinkedList<ShortBossRow>
}

impl PostgresGetBosses {
    pub fn new() -> PostgresGetBosses {
        PostgresGetBosses { payload: LinkedList::new() }
    }
}

impl PostgresCommand for PostgresGetBosses {
    fn execute(&mut self,connect: &Transaction) -> Result<(),Error> {
        let trans = connect.transaction().unwrap();
        let statement = trans.prepare("SELECT id, label FROM bosses;").unwrap();
        match statement.query(&[]) {
            Ok(rows) => {  let mut iter = rows.iter();
                           while let Some(row) = iter.next() {
                               let boss = ShortBossRow { id: row.get("id"),
                                                          label: row.get("label") };
                               self.payload.push_back(boss);
                           }

                           trans.commit().unwrap();
                           return Ok(());
            },
            Err(er) =>  Err(er)
        }
    }
}

#[derive(Deserialize)]
pub struct PostgresInsertBoss {
    label: String,
    level: i32,
    drop: Vec<ItemProbability>,
}

impl PostgresInsertBoss {
    pub fn new(label: &str, level: i32, drop: Vec<ItemProbability>) -> PostgresInsertBoss {
        PostgresInsertBoss { label: label.to_string(), level: level, drop: drop }
    }
}

impl PostgresCommand for PostgresInsertBoss {
    fn execute(&mut self,connect: &Transaction) -> Result<(),Error> {
        let trans = connect.transaction().unwrap();

        let statement = trans.prepare("INSERT INTO bosses VALUES(default, $1, $2, $3);").unwrap();

        match statement.query(&[&self.label,&self.level,&self.drop]) {
            Ok(rows) => {    trans.commit().unwrap();
                             return Ok(());
            },
            Err(er) =>  Err(er)
        }
    }
}
