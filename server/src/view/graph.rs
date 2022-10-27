use std::collections::BTreeMap;

use rocket::{State, response::content::RawHtml, serde::uuid::Uuid};
use rocket_dyn_templates::Template;
use rocket::form::{Form};
use rocket::serde::json::Json;
use serde_json::value::Value;
use serde_json::Map;
use serde::{Deserialize};

use crate::store::{GraphStore, Schema};

#[derive(FromForm)]
pub struct VertexForm<'v> {
    name: &'v str,
    love: &'v str,
}

#[deprecated]
#[post("/vertex", format="form", data="<form>")]
pub async fn post_vertex(graph_store: &State<GraphStore>, form: Form<VertexForm<'_>>) -> Result<String, String> {
    let name = form.name;
    match graph_store.insert(name, form.love) {
        Ok(id) => {
            Ok(format!("{} is inserted as {}", name, id))
        },
        Err(e) => {
            Err(format!("db error{}", e))
        }
    }
}

#[deprecated]
#[get("/vertex", format="any", rank=1)]
pub fn vertex_list(graph_store: &State<GraphStore>) -> Template {
    let vertices = graph_store.get_all_vertices();
    let texts: Vec<BTreeMap<String, _>> = vertices.iter().map(|v| {
        let mut item = BTreeMap::new();
        item.insert(String::from("id"), Value::String(v.vertex.id.hyphenated().to_string()));
        item.insert(String::from("type"), Value::String(v.vertex.t.as_str().to_owned()));
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

#[derive(Deserialize)]
pub struct CreateVertexPayload {
    t: Value,
    properties: Option<Map<String, Value>>
}

#[post("/vertex", format="json", data="<obj>")]
pub fn create_vertex(graph_store: &State<GraphStore>, obj: Json<CreateVertexPayload>) -> Json<Value> {
    let data = obj.into_inner();
    let res = graph_store.create_vertex(data.t, data.properties);
    match res {
        Ok(uuid) => {
            let response = json!({
                "code": 0u32,
                "res": uuid.hyphenated().to_string(),
            });
            Json(response)
        },
        Err(err) => {
            let response = json!({
                "code": 1u32,
                "err": format!("{}", err),
            });
            Json(response)
        }
    }
    
}

#[get("/vertex", format="json")]
pub fn get_vertex(graph_store: &State<GraphStore>) -> Json<Value> {
    let vertices = graph_store.get_all_vertices();
    let texts: Vec<BTreeMap<String, _>> = vertices.iter().map(|v| {
        let mut item = BTreeMap::new();
        item.insert(String::from("id"), Value::String(v.vertex.id.hyphenated().to_string()));
        item.insert(String::from("type"), Value::String(v.vertex.t.as_str().to_owned()));
        let mut props = Map::new();
        v.props.iter().for_each(|p| {
            let cloned = p.to_owned();
            props.insert(cloned.name.to_string(), cloned.value);
        });
        item.insert(String::from("props"), Value::Object(props));
        item
    }).collect();
    let mut data = BTreeMap::new();
    data.insert("vertices".to_string(), texts);
    let response = json!({
        "code": 0u32,
        "data": data,
    });
    Json(response)
}

#[derive(Deserialize)]
pub struct CreateEdgePayload {
    t: String,
    outbound_id: Uuid,
    inbound_id: Uuid,
    properties: Option<Map<String, Value>>
}

#[post("/edge/create", format="json", data="<payload>")]
pub fn create_edge(graph_store: &State<GraphStore>, payload: Json<CreateEdgePayload>) -> Json<Value> {
    let data = payload.into_inner();
    let result = graph_store.create_edge(&data.t, data.outbound_id, data.inbound_id, data.properties);
    match result {
        Ok(true) => {
            let response = json!({
                "code": 0u32,
            });
            Json(response)
        },
        Ok(false) => {
            let response = json!({
                "code": 1u32,
                "err": "failed to create edge",
            });
            Json(response)
        },
        Err(err) => {
            let response = json!({
                "code": 1u32,
                "err": format!("{}", err),
            });
            Json(response)
        }
    }
}

#[derive(Deserialize)]
pub struct GetEdgePayload {
    t: String,
    vertex_id: Uuid,
}

#[post("/edge/get", format="json", data="<payload>")]
pub fn get_edge(graph_store: &State<GraphStore>, payload: Json<GetEdgePayload>) -> Json<Value> {
    let data = payload.into_inner();
    let edges = graph_store.get_all_edges(&data.t, data.vertex_id);
    let texts: Vec<BTreeMap<String, _>> = edges.iter().map(|e| {
        let mut item = BTreeMap::new();
        item.insert(String::from("t"), Value::String(e.edge.key.t.to_owned().into_string()));
        item.insert(String::from("inbound_id"), Value::String(e.edge.key.inbound_id.to_string()));
        item.insert(String::from("outbound_id"), Value::String(e.edge.key.outbound_id.to_string()));
        let mut props = Map::new();
        e.props.iter().for_each(|p| {
            let cloned = p.to_owned();
            props.insert(cloned.name.to_string(), cloned.value);
        });
        item.insert(String::from("props"), Value::Object(props));
        item
    }).collect();
    let mut data = BTreeMap::new();
    data.insert("vertices".to_string(), texts);
    let response = json!({
        "code": 0u32,
        "data": data,
    });
    Json(response)
}

/** GraphQL **/

#[get("/graphql", format = "any", rank = 1)]
pub fn graphiql() -> RawHtml<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[get("/graphql?<request>", format = "json")]
pub fn get_graphql_handler(
    context: &State<GraphStore>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&*schema, &*context)
}

#[post("/graphql", data = "<request>")]
pub fn post_graphql_handler(
    context: &State<GraphStore>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&*schema, &*context)
}
