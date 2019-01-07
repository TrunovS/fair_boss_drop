mod BdLayer;
use BdLayer::PostgresDealer::*;
use BdLayer::PostgresCommands::*;


fn main() {
    let mut bd_data = PostgresSqlData::new();
    // bd_data.connect("postgres://postgres:qwerty@localhost").unwrap();
    bd_data.connect().unwrap();
    bd_data.doCommand(PostgresInitTables::new()).unwrap();

    println!("Hello, world!");
}
