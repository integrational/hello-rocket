#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::json::Json;

#[derive(Serialize, Deserialize, Debug)]
struct Greeting {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GreetingWithId {
    id: usize,
    message: String,
}

#[get("/")]
fn get_greetings() -> String {
    format!("TODO all greetings")
}

#[post("/", format = "application/json", data = "<greeting>")]
fn add_greeting(greeting: Json<Greeting>) -> Json<GreetingWithId> {
    Json(GreetingWithId {
        id: 1,
        message: greeting.message.clone(),
    })
}

#[get("/<id>", format = "application/json")]
fn get_greeting(id: usize) -> String {
    format!("TODO: get {} as JSON", id)
}

#[get("/<id>", format = "application/greeting+xml", rank = 2)]
fn get_greeting_as_greeting_xml(id: usize) -> String {
    format!("TODO: get {} as XML with application/greeting+xml", id)
}

#[get("/<id>", format = "application/xml", rank = 3)]
fn get_greeting_as_app_xml(id: usize) -> String {
    get_greeting_as_greeting_xml(id)
}

#[get("/<id>", format = "text/xml", rank = 4)]
fn get_greeting_as_xml(id: usize) -> String {
    get_greeting_as_greeting_xml(id)
}

#[put("/<id>")]
fn update_greeting(id: usize) -> String {
    format!("TODO: update {}", id)
}

#[delete("/<id>")]
fn delete_greeting(id: usize) -> String {
    format!("TODO: delete {}", id)
}

fn main() {
    rocket::ignite()
        .mount(
            "/api/v1/greetings",
            routes![
                get_greetings,
                add_greeting,
                get_greeting,
                get_greeting_as_greeting_xml,
                get_greeting_as_app_xml,
                get_greeting_as_xml,
                update_greeting,
                delete_greeting
            ],
        )
        .launch();
}
