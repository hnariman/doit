/*
    Simple Cli ToDo Manager with sqlite:

    Storage  - https://docs.rs/rusqlite/latest/rusqlite/
    Cli base - https://docs.rs/clap/latest/clap/

    for the first time its planned as a single updatable list,
    in future doit will grow into multiple list application with Tauri GUI client
*/

// use chrono::{DateTime, Utc};
use rusqlite::Result;

// enum Operations {
//     Create,
//     Read,
//     Update,
//     Delete,
// }

// #[derive(Parser, Debug)]
// struct Args {
//     #[clap(short, long, value_parser)]
//     create: String,
//     // #[clap(short, long, value_parser)]
//     // read: String,
//     // #[clap(short, long, value_parser)]
//     // update: String,
//     // #[clap(short, long, value_parser)]
//     // delete: String,
// }

#[derive(Debug)]
#[allow(dead_code)] // until we aren't using this struct
struct Item {
    name: String,
    id: i32,
    is_deleted: bool,
    // created: DateTime<Utc>,
}

fn main() -> Result<()> {
    // let db = setup_storage().unwrap(); - to pass DB further
    setup_storage().unwrap();

    let create: clap::Arg = clap::Arg::with_name("create")
        .short('c')
        .long("create")
        .takes_value(true)
        .value_name("TEXT")
        .help("Create new todo item")
        .required(false);

    let read = clap::Arg::with_name("read")
        .short('r')
        .long("read")
        .takes_value(true)
        .value_name("ID")
        .help("Read todo item details")
        .required(false);

    let update = clap::Arg::with_name("update")
        .short('u')
        .long("update")
        .takes_value(true)
        .multiple(true)
        .value_name("TEXT")
        .help("Update todo item with new text")
        .required(false);

    let delete = clap::Arg::with_name("delete")
        .short('d')
        .long("delete")
        .takes_value(true)
        .value_name("ID")
        .help("Delete todo item")
        .required(false);

    let list = clap::Arg::with_name("list")
        .short('l')
        .long("list")
        .help("list all todo item")
        .required(false);

    let app = clap::Command::new("doit")
        .version("0.0.1")
        .about("Portable CLI Task Manager in Rust")
        .author("Nariman Huseynov <github.com/hnariman>")
        .arg(create)
        .arg(read)
        .arg(update)
        .arg(delete)
        .arg(list)
        .get_matches();

    // Enumerate App flags & options for easier maintenance & debugging
    // let options = vec!["create", "read", "update", "delete"];

    // flags validation:
    if app.is_present("list") {
        // list_items();
        println!("LIST ALL ITEMS!");
    }

    // options validation:
    if let Some(val) = app.value_of("create") {
        create_item(val);
    }

    if let Some(val) = app.value_of("delete") {
        let id = val.parse().unwrap();
        delete_item(id);
    }

    if let Some(val) = app.value_of("update") {
        if let Ok(id) = val.parse() {
            update_item(id, val);
        } else {
            println!("Item number must be digit!");
        }
    }

    if let Some(val) = app.value_of("read") {
        let id = val.parse().unwrap();
        read_item(id);
    }

    // let args = command!()
    //     .args(arg!(-c --create <TEXT> "Create new todo" ))
    //     .args(arg!(-r --read <ID> "Read full todo"))
    //     .args(arg!(-u --update <ID> <TEXT> "Update todo"))
    //     .args(arg!(-d --delete<ID> "Delete todo"))
    //     .get_matches();

    Ok(())
}

// printer/logger for single/multiple (polymorph) events
/////// - ??

fn setup_storage() -> Result<rusqlite::Connection> {
    let db = rusqlite::Connection::open_in_memory()?;
    db.execute(
        "CREATE TABLE todo (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL, 
        is_deleted BOOLEAN
    )",
        (),
    )?;

    // populate DB with test data
    {
        let id: u8 = 1;
        let name: String = "test user".to_string();
        let is_deleted: bool = false;
        // let created: String = chrono::Utc::now().to_string();

        db.execute(
            "INSERT INTO todo (id, name,is_deleted) VALUES (?1, ?2, ?3)",
            (&id, &name, &is_deleted),
        )?;
    }

    // assert test data
    {
        let mut query = db.prepare("SELECT * FROM todo")?;
        let data = query.query_map([], |r| {
            Ok(Item {
                id: r.get(0)?,
                name: r.get(1)?,
                is_deleted: r.get(2)?,
                // created: r.get(3)?,
            })
        })?;
        for item in data {
            println!("{:?}", item.unwrap());
        }
    }
    Ok(db)
}

// basic CRUD operations
fn create_item(item: &str) {
    //-> Result<()> {
    // rusqlite doesn't support INSERT IF NOT EXIST??? - TODO: Check API Docs!!

    // db.execute(
    //     "INSERT INTO todo (id, name, created)",
    //     (item.id, &item.name, chrono::Utc::now().timestamp()),
    // )?;
    println!("CREATE ITEM: {:?}", item);
    // Ok(())
}
fn read_item(id: u8) {
    // -> Result<()>{
    println!("READ ITEM: {}", id);
    // db.execute("SELECT * FROM todo WHERE ID in (?1) ",(&id))?;
    // Ok(())
}
fn update_item(id: u8, text: &str) {
    // -> Result<()> {
    // db.execute( "UPDATE todo SET name = ?1 WHERE id = ?1" (&name, &id))?;
    println!("UPDATE ITEM {} with TEXT {}", id, text);
    // Ok(())
}
fn delete_item(id: u8) {
    println!("DELETE ITEM WITH ID: {}", id);
    // Do we need more than 1 value? - WHERE ID in (1,2,3,4)

    // db.execute("DELETE FROM todo WHERE ID in (?1) ",(&id))?;
    // Ok(())
}
