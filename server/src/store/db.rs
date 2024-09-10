use indradb::{
  RocksdbDatastore, Datastore,
  Vertex, Result as DbResult, BulkInsertItem,
  Identifier, VertexProperties, RangeVertexQuery,
  Error as DbError, EdgeKey, EdgeProperties, SpecificVertexQuery, VertexQueryExt,
};
use juniper::Context;
use serde_json::value::Value;
use serde_json::Map;
use rocket::serde::uuid::{Uuid};

pub struct GraphStore {
    pub store: RocksdbDatastore
}

impl Context for GraphStore {}

impl GraphStore {
    pub(crate) fn new(path: Option<&str>) -> Self {
        let db_path = match path {
          Some(str) => str,
          None => "./store",
        };
        let store = RocksdbDatastore::new(db_path, None).unwrap();

        store.index_property(Identifier::new("name").unwrap()).unwrap();

        Self {
          store
        }
    }

    #[deprecated]
    pub fn insert(&self, name: &str, love: &str) -> DbResult<String> {
      let vertex = Vertex::new(Identifier::new(name).unwrap());
      let vertex_id = vertex.id;
      let id_string = vertex.id.hyphenated().to_string();

      match self.store.bulk_insert(vec!{BulkInsertItem::Vertex(vertex)}) {
        Ok(_) => {
          let vertex_property = BulkInsertItem::VertexProperty(
            vertex_id,
            Identifier::new("love").unwrap(),
            Value::String(love.to_string())
          );
          self.store.bulk_insert(vec!{vertex_property}).map(|_| id_string)
        },
        Err(e) => Err(e)
      }

    }

    pub fn create_vertex(&self, t: Value, properties: Option<Map<String, Value>>) -> DbResult<Uuid> {
      match t.as_str() {
        Some(str) => {
          let uuid = self.store.create_vertex_from_type(Identifier::new(str).unwrap()).unwrap();
          match properties {
            Some(obj) => {
              let item_list: Vec<_> = obj.iter().map(|(key, value)| {
                BulkInsertItem::VertexProperty(
                  uuid,
                  Identifier::new(key).unwrap(),
                  value.clone(),
                )
              }).collect();
              self.store.bulk_insert(item_list).map(|_| uuid)
            },
            _ => Ok(uuid)
          }
        },
        None => Err(DbError::Datastore("No type given".into()))
      }
    }
    

    pub fn get_all_vertices(&self) -> Vec<VertexProperties> {
      let vertex_query = RangeVertexQuery::new();
      self.store.get_all_vertex_properties(vertex_query.into()).unwrap()
    }

    pub fn create_edge(
      &self,
      t: &str, outbound_id: Uuid, inbound_id: Uuid, properties: Option<Map<String, Value>>,
    ) -> DbResult<bool> {
      let edge_key = EdgeKey::new(outbound_id, Identifier::new(t).unwrap(), inbound_id);
      match self.store.create_edge(&edge_key) {
        Ok(result) => {
          if result {
            match properties {
              Some(obj) => {
                let item_list: Vec<_> = obj.iter().map(|(key, value)| {
                  BulkInsertItem::EdgeProperty(
                    edge_key.to_owned(),
                    Identifier::new(key).unwrap(),
                    value.clone(),
                  )
                }).collect();
                self.store.bulk_insert(item_list).map(|_| true)
              },
              _ => Ok(true)
            }
          } else {
            Ok(result)
          }
        },
        Err(e) => Err(e)
      }
    }

    pub fn get_all_edges(&self, t: &str, vertex_id: Uuid) -> Vec<EdgeProperties> {
      let edge_query = SpecificVertexQuery::single(vertex_id).outbound().t(Identifier::new(t).unwrap());
      self.store.get_all_edge_properties(edge_query.into()).unwrap()
    }

    pub fn delete_vertex(&self, vertex_id: Uuid) -> DbResult<()> {
      let vertex_query = SpecificVertexQuery::single(vertex_id);
      self.store.delete_vertices(vertex_query.into())
    }
}
