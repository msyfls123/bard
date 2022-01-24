#[macro_use] extern crate rocket;

use std::collections::BTreeMap;
use rocket::fs::{FileServer};
use rocket_dyn_templates::Template;

mod store;
mod view;
mod state;
mod helpers;

use state::AppState;

#[get("/")]
fn index() -> Template {
    let mut data = BTreeMap::new();
    data.insert("name".to_string(), "kimi".to_string());
    Template::render("index", &data)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(AppState {
            graph_store: store::GraphStore::new(None)
        })
        .attach(helpers::template::get_template())
        .mount("/", [
            routes![index],
            view::get_routes(),
        ].concat())
        .mount("/public", FileServer::from("static/"))
}
