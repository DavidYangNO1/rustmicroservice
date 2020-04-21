extern crate postgres;

use postgres::Client;
use postgres::tls::NoTls;

mod migrations;
use migrations::*;

fn main() -> Result<(),postgres::Error> {
    println!("migration --->!");
    
    let mut client = Client::connect("postgres://postgres:docker@172.17.0.2:5432/postgres",NoTls).unwrap();
    let mut migrations = Vec::new();
    migrations.push(CreateTableNewsMigration::new().run(&mut client));
    migrations.push(AddNewsRecordsMigration::new().run(&mut client));

    for result in migrations.iter(){
        match result {
            Ok(changes) =>  println!("migration success:{} !",changes),
            Err(error) => println!("migration failure:{} !",error),
        }
    }
    Ok(())
}
