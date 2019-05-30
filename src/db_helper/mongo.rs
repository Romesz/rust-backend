
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

pub fn establish_connection() -> mongodb::coll::Collection {

    let client = Client::connect("mongo", 27017)
        .expect("Failed to initialize standalone client.");

    client.db("test").collection("tmp")
}