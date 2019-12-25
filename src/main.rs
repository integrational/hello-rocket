#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket::http::Status;
use rocket::response::status::Created;
use rocket::Request;
use rocket_contrib::json::Json;

#[derive(Serialize, Debug)]
struct ErrorResponse {
    status: u16,
    reason: String,
}

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
fn get_greetings() -> Json<Vec<GreetingWithId>> {
    let grs = vec![
        GreetingWithId {
            id: 1,
            message: "Seavas du".to_string(),
        },
        GreetingWithId {
            id: 2,
            message: "Howdy pahdner".to_string(),
        },
    ];
    Json(grs)
}

#[post("/", format = "application/json", data = "<greeting>")]
fn add_greeting(greeting: Json<Greeting>) -> Created<Json<GreetingWithId>> {
    let id: usize = 1;
    let gr = GreetingWithId {
        id,
        message: greeting.message.clone(),
    };
    Created(greeting_url_path(id), Some(Json(gr)))
}

#[get("/<id>", format = "application/json")]
fn get_greeting(id: usize) -> Json<GreetingWithId> {
    Json(GreetingWithId {
        id,
        message: format!("This is greeting {}", id),
    })
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
fn delete_greeting(id: usize) -> Result<String, Status> {
    if id == 42 {
        Err(Status::Forbidden)
    } else {
        Ok(format!("TODO: delete {}", id))
    }
}

#[catch(403)]
fn forbidden(requ: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        status: 403,
        reason: format!("{} on resource {} is forbidden", requ.method(), requ.uri()),
    })
}

#[catch(404)]
fn not_found(requ: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        status: 404,
        reason: format!("Resource {} not found", requ.uri()),
    })
}

#[catch(500)]
fn internal_error() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        status: 500,
        reason: "Internal server error".to_string(),
    })
}

fn greeting_url_path(id: usize) -> String {
    format!("/api/v1/greetings/{}", id)
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
        .register(catchers![forbidden, not_found, internal_error])
        .launch();
}
