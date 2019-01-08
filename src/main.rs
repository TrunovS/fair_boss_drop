mod BdLayer;
use BdLayer::PostgresDealer::*;
use BdLayer::PostgresCommands::*;


fn main() {
    let mut bd_data = PostgresSqlData::new();
    bd_data.connect().unwrap();
    bd_data.doCommand(PostgresInitTables::new()).unwrap();
    println!("status of connection: {}", bd_data.isOpen());
    bd_data.finish();
    println!("status of connection: {}", bd_data.isOpen());

    println!("Hello, world!");
}
