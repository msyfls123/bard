use indradb::{
  RocksdbDatastore, Datastore,
  Vertex, Result as DbResult, BulkInsertItem,
  Identifier, VertexProperties, RangeVertexQuery,
  Error as DbError,
};
use serde_json::value::{Value};
use uuid::{Uuid};

pub struct GraphStore {
    store: RocksdbDatastore
}

impl GraphStore {
    pub(crate) fn new(path: Option<&str>) -> Self {
        let db_path = match path {
          Some(str) => str,
          None => "./store",
        };
        let store = RocksdbDatastore::new(db_path, None).unwrap();

        Self {
          store
        }
    }

    pub fn insert(&self, name: &str, love: &str) -> DbResult<String> {
      let vertex = Vertex::new(Identifier::new(name).unwrap());
      let vertex_id = vertex.id;
      let id_string = vertex.id.to_hyphenated().to_string();

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

    pub fn create_vertex(&self, t: Value, properties: Option<Value>) -> DbResult<Uuid> {
      match t.as_str() {
        Some(str) => {
          let uuid = self.store.create_vertex_from_type(Identifier::new(str).unwrap()).unwrap();
          match properties {
            Some(obj) => {
              let item_list: Vec<_> = obj.as_object().unwrap().iter().map(|(key, value)| {
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
}
