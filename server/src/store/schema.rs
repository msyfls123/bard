use super::db::GraphStore;
use juniper::{ graphql_object, GraphQLObject, EmptyMutation, EmptySubscription, RootNode, };
use indradb::{PropertyValueVertexQuery, Identifier, Datastore, RangeVertexQuery};
use rocket::serde::uuid::Uuid;
use serde_json::Value;

pub struct Query;

#[derive(GraphQLObject, Clone)]
struct Man {
    id: String,
    name: Option<String>,
}

#[derive(GraphQLObject, Clone)]
struct Note {
    id: String,
    text: Option<String>,
}

#[graphql_object(context = GraphStore)]
impl Query {
    fn man(
        #[graphql(context)] database: &GraphStore,
        #[graphql(description = "id of the human")] name: String,
    ) -> Option<Man> {
        // TODO ugly implement, maybe use dataloader
        let vertex_query = PropertyValueVertexQuery::new(Identifier::new("name").unwrap(), Value::String(name));
        let result = database.store.get_all_vertex_properties(vertex_query.into());
        match result {
            Ok(list) => {
                if list.len() == 0 { return None }
                let man = Man {
                    id: list[0].vertex.id.hyphenated().to_string(),
                    name: list[0].props.iter().find_map(|i| {
                        if i.name.as_str() == "name" {
                            Some(i.value.to_string())
                        } else {
                            None
                        }
                    }),
                };
                Some(man)
            },
            Err(_) => None,
        }
    }

    fn notes(
        #[graphql(context)] database: &GraphStore,
        #[graphql(description = "start_id")] start_id: Option<String>,
        #[graphql(description = "limit")] limit: Option<i32>,
    ) -> Vec<Note> {
        let mut vertex_query = RangeVertexQuery::new()
            .t(Identifier::new("note").unwrap())
            .limit(limit.unwrap_or(10) as u32);
        if start_id.is_some() {
            vertex_query = vertex_query.start_id(start_id.map(|id| Uuid::parse_str(&id).unwrap()).unwrap());
        }
        let result = database.store.get_all_vertex_properties(vertex_query.into());
        match result {
            Ok(list) => {
                list.iter().map(|v| {
                    Note {
                        id: v.vertex.id.hyphenated().to_string(),
                        text: v.props.iter().find_map(|i| {
                            if i.name.as_str() == "text" {
                                Some(i.value.to_string())
                            } else {
                                None
                            }
                        }),
                    }
                }).collect()
            },
            Err(_) => vec![],
        }
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<GraphStore>, EmptySubscription<GraphStore>>;

pub fn init_schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<GraphStore>::new(),
        EmptySubscription::<GraphStore>::new(),
    )
}
