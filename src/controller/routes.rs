extern crate mongodb;
use mongodb::{bson, doc};

use rocket::response::status;

#[path = "../db_helper/mod.rs"]
mod db_helper;

#[get("/", format = "application/json")]
pub fn get_all() -> status::Accepted<String> {
    let coll = db_helper::mongo::establish_connection();
    let mut resp = Vec::new();

    for result in coll.find(None, None).unwrap() {
        if let Ok(item) = result {
            // println!("{}", item);
            resp.push(item);
        }
    }

    status::Accepted(Some(format!("{:?}", resp)))
}

#[get("/get/<name>")]
pub fn get(name: String) -> status::Accepted<String> {
    let coll = db_helper::mongo::establish_connection();

    let _response_document = coll.find_one(Some(doc! {"name": name}), None)
        .ok().expect("Failed to execute find.");

    //let debug = _response_document.unwrap().to_owned();
    //println!("{:?}", debug.get("title").unwrap().to_string());

    status::Accepted(Some(format!("{}", _response_document.unwrap().to_owned())))
}

// should be post
#[get("/add/<name>")]
pub fn add(name: String) -> status::Accepted<String> {
    let coll = db_helper::mongo::establish_connection();
    let todo = doc! {
        "name": name,
    };

    coll.insert_one(todo.clone(), None)
        .ok().expect("Failed to insert document.");

    status::Accepted(Some(format!("added: '{}'", todo.get("name").unwrap().to_string())))
}

// should be delete
#[get("/delete/<name>")]
pub fn delete(name: String) -> status::Accepted<String> {
    let coll = db_helper::mongo::establish_connection();
    let todo = doc! {
        "name": name,
    };

    coll.delete_many(todo.clone(), None).ok().expect("Failed to insert document.");

    status::Accepted(Some(format!("{} deleted", todo.get("name").unwrap().to_string())))
}

// should be delete
#[get("/deleteall")]
pub fn delete_all() -> status::Accepted<String> {
    let coll = db_helper::mongo::establish_connection();

    coll.delete_many(doc!{}, None).ok().expect("Failed to insert document.");

    status::Accepted(Some(format!("all deleted")))
}

// should be put
#[get("/update/<original_name>/<new_name>")]
pub fn update(original_name: String, new_name: String) -> status::Accepted<String> {
    let coll = db_helper::mongo::establish_connection();
    let old_todo = doc! {
        "name": original_name,
    };

    let new_todo = doc! {
        "name": new_name,
    };

    coll.update_one(old_todo.clone(), doc!{ "$set" => new_todo.clone()}, None).ok().expect("Failed to update document.");

    status::Accepted(Some(format!("{} updated to name: {}", old_todo.get("name").unwrap().to_string(), new_todo.get("name").unwrap().to_string())))
}