#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_json;

use std::collections::BTreeMap;
use rocket::fs::{FileServer, Options};
use rocket_dyn_templates::Template;

mod store;
mod view;
mod helpers;

#[get("/")]
fn index() -> Template {
    let mut data = BTreeMap::new();
    data.insert("name".to_string(), "kimi".to_string());
    Template::render("index", &data)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(store::GraphStore::new(None))
        .attach(helpers::template::get_template())
        .mount("/", [
            routes![index],
            view::get_routes(),
        ].concat())
        .mount("/public", FileServer::new("static/", Options::NormalizeDirs | Options::Index))
}
