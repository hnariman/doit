/* DB Connection to Setup */
pub mod setup {
    const CREATE_DB: &str =
        "CREATE TABLE todo ( id INTEGER PRIMARY KEY, name TEXT NOT NULL, is_deleted BOOLEAN)";

    #[derive(Debug)]
    #[allow(dead_code)] // until we aren't using this struct fully
    pub struct Item {
        name: String,
        id: i32,
        is_deleted: bool,
        // created: DateTime<Utc>,
    }

    pub fn init() -> rusqlite::Result<rusqlite::Connection> {
        let db = rusqlite::Connection::open_in_memory()?;
        db.execute(CREATE_DB, ())?;

        // populate DB with test data
        {
            const INSERT: &str = "INSERT INTO todo (id, name,is_deleted) VALUES (?1, ?2, ?3)";
            let id: u8 = 1;
            let name: String = "test user".to_string();
            let is_deleted: bool = false;
            // let created: String = chrono::Utc::now().to_string();

            db.execute(INSERT, (&id, &name, &is_deleted))?;
        }

        // assert test data

        {
            const SELECT_ALL: &str = "SELECT * FROM todo";
            let mut query = db.prepare(SELECT_ALL)?;

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
}

/* basic CRUD operations */
pub mod crud {
    // const CREATE: &str = "INSERT INTO todo (id, name, created)";
    pub fn create(item: &str) {
        //-> Result<()> {
        // rusqlite doesn't support INSERT IF NOT EXIST??? - TODO: Check API Docs!!

        // db.execute(CREATE,(item.id, &item.name, chrono::Utc::now().timestamp()))?;
        println!("CREATE ITEM: {:?}", item);
        // Ok(())
    }

    // const READ  : &str = "SELECT * FROM todo WHERE ID in (?1)";
    pub fn read(id: u8) {
        // -> Result<()>{
        println!("READ ITEM: {}", id);
        // db.execute(READ,(&id))?;
        // Ok(())
    }

    // const UPDATE: &str = "UPDATE todo SET name = ?1 WHERE id = ?1";
    pub fn update(id: u8, text: &str) {
        // -> Result<()> {
        // db.execute(UPDATE, (&name, &id))?;
        println!("UPDATE ITEM {} with TEXT {}", id, text);
        // Ok(())
    }

    // const DELETE: &str = "DELETE FROM todo WHERE ID in (?1) ";
    pub fn delete(id: u8) {
        // -> Result<()> {
        println!("DELETE ITEM WITH ID: {}", id);
        // Do we need more than 1 value? - WHERE ID in (1,2,3,4)
        // db.execute(DELETE,(&id))?;
        // Ok(())
    }

    pub fn get_all() {
        // -> Result<()> {
        println!("GET ALL ITEMS");
        // const GET_ALL: &str = "SELECT * FROM todo";
        // Do we need more than 1 value? - WHERE ID in (1,2,3,4)
        // let query = db.prepare(GET_ALL);
        // Ok(())
        // let data = query.query_map([], |r| {
        //     Ok(Item {
        //         id: r.get(0)?,
        //         name: r.get(1)?,
        //         is_deleted: r.get(2)?,
        //         // created: r.get(3)?,
        //     })
        // })?;
        // data
    }
}

// pub enum Operations {
//     Create,
//     Read,
//     Update,
//     Delete,
// }
