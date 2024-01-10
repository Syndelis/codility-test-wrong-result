use rocket::post;
use rocket::response::Responder;
use rocket::serde::json::{from_str, Json};
use rocket::State;
use rocket::{launch, routes};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Responder)]
#[response(status = 400, content_type = "text")]
pub struct UserBadRequest(String);

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct UserCreated(Json<Record>);

#[derive(Responder)]
pub enum UserResponder {
    Created(UserCreated),
    Err(UserBadRequest),
}

#[post("/", data = "<user>")]
pub fn users(database: &State<Mutex<Box<dyn Database>>>, user: String) -> UserResponder {
    if let Ok(user) = from_str::<User>(&user) {
        if user.name.len() > 32 || user.age < 16 {
            UserResponder::Err(UserBadRequest(String::from("Invalid Payload")))
        } else {
            let record = database.lock().unwrap().save(user);
            UserResponder::Created(UserCreated(Json(record)))
        }
    } else {
        UserResponder::Err(UserBadRequest(String::from("Missing fields")))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub name: String,
    pub age: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct Record {
    pub id: usize,
    pub user: User,
}

pub trait Database: Send {
    fn save(&mut self, user: User) -> Record;
}

pub struct Db;

impl Database for Db {
    fn save(&mut self, user: User) -> Record {
        Record { id: 1, user }
    }
}

#[launch]
fn rocket() -> _ {
    let db: Mutex<Box<dyn Database>> = Mutex::new(Box::new(Db));

    rocket::build().mount("/", routes![users]).manage(db)
}
