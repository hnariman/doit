/*
    Simple Cli ToDo Manager with sqlite:

    Storage  - https://docs.rs/rusqlite/latest/rusqlite/
    Cli base - https://docs.rs/clap/latest/clap/

    for the first time its planned as a single updatable list,
    in future doit will grow into multiple list application with Tauri GUI client
*/

// use chrono::{DateTime, Utc};
use rusqlite::Result;
mod db;
mod cli;



fn main() -> Result<()> {
    // let db = db::setup::init().unwrap(); - to pass DB further
    db::setup::init().unwrap();
    let app = cli::setup();


    // Enumerate App flags & options for easier maintenance & debugging
    // let options = vec!["create", "read", "update", "delete"];

    // flags validation:
    if app.is_present("list") {
        db::crud::get_all();
    }

    // options validation:
    if let Some(val) = app.value_of("create") {
        db::crud::create(val);
    }

    if let Some(val) = app.value_of("delete") {
        db::crud::delete(val.trim().parse::<u8>().unwrap());
    }

    if let Some(val) = app.value_of("update") {
        if let Ok(id) = val.parse() {
            db::crud::update(id, val);
        } else {
            println!("Item number must be digit!");
        }
    }

    if let Some(val) = app.value_of("read") {
        let id = val.parse().unwrap();
        db::crud::read(id);
    }

    Ok(())
}

// printer/logger for single/multiple (polymorph) events
/////// - ??