use BdLayer::PostgresDealer::PostgresCommand;
use ::postgres::{Connection, error::Error};
use std::collections::LinkedList;

// #[derive(Debug, FromSql, ToSql)]
// #[postgres(name="item_probability")]
pub struct item_probability {
    pub _id: i32,
    pub _probability: f32,
}

// impl ToSql for item_probability {
//     fn to_sql<W: Write+?Sized>(&self,_:&Type,mut w: &mut W, _: &SessionInfo) -> Result<IsNull> {

//     }
// }


pub struct PostgresGetBoss {
    _label: String,
    _level: Option<i32>,
    _drop: Option<LinkedList<item_probability>>,
}

impl PostgresGetBoss {
    pub fn new(label: &str) -> PostgresGetBoss {
        PostgresGetBoss { _label: String::from(label), _level: None, _drop: None  }
    }
    // pub fn getData(&self) -> &LinkedList<String> {
    //     return &self._bosses;
    // }
}

impl PostgresCommand for PostgresGetBoss {
    fn execute(&mut self,connect: &Connection) -> Result<(),Error> {
        let trans = connect.transaction().unwrap();
        let statement = trans.prepare("SELECT * FROM bosses WHERE label=$1;").unwrap();
        match statement.query(&[]) {
            Ok(rows) => {    let mut iter = rows.iter();
                             while let Some(row) = iter.next() {
                                 self._level = Some(row.get("level"));
                                 // self._drop = Some(row.get("drop"));
                                 // let vd: Vec<item_probability> = row.get("drop");
                                 // println!("{:?}",vd);
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
        sql
    }
}

impl PostgresCommand for PostgresInsertBoss {
    fn execute(&mut self,connect: &Connection) -> Result<(),Error> {
        let trans = connect.transaction().unwrap();
        let sql = format!("INSERT INTO bosses VALUES(default, '{}', {}, {});",
                          self._label,self._level, self.convertDropToSql());
        let statement = trans.prepare(&sql).unwrap();

        match statement.query(&[]) {
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
