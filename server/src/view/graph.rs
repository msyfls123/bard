use std::collections::BTreeMap;

use rocket::State;
use rocket_dyn_templates::Template;
use rocket::form::{Form};
use rocket::serde::json::Json;
use serde_json::value::Value;

use crate::state::AppState;

#[derive(FromForm)]
pub struct VertexForm<'v> {
    name: &'v str,
    love: &'v str,
}

#[post("/vertex", format="form", data="<form>")]
pub async fn post_vertex(state: &State<AppState>, form: Form<VertexForm<'_>>) -> Result<String, String> {
    let name = form.name;
    match state.graph_store.insert(name, form.love) {
        Ok(id) => {
            Ok(format!("{} is inserted as {}", name, id))
        },
        Err(e) => {
            Err(format!("db error{}", e))
        }
    }
}

#[get("/vertex")]
pub fn get_vertex(state: &State<AppState>) -> Template {
    let vertices = state.graph_store.get_all_vertices();
    let texts: Vec<BTreeMap<String, _>> = vertices.iter().map(|v| {
        let mut item = BTreeMap::new();
        item.insert(String::from("id"), Value::String(v.vertex.id.to_hyphenated().to_string()));
        item.insert(String::from("name"), Value::String(v.vertex.t.as_str().to_owned()));
        v.props.iter().for_each(|p| {
            let cloned = p.to_owned();
            item.insert(cloned.name.to_string(), cloned.value);
        });
        item
    }).collect();
    let mut data = BTreeMap::new();
    data.insert("texts".to_string(), texts);
    Template::render("vertex", &data)
}

#[post("/vertex", format="json", data="<obj>")]
pub fn create_vertex(state: &State<AppState>, obj: Json<Value>) -> Json<Value> {
    let data = obj.into_inner();
    let response = json!({
        "code": 0u32,
        "res": data["name"]
    });
    Json(response)
}
