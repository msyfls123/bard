use indradb::{
  RocksdbDatastore, Datastore,
  Vertex, Result, BulkInsertItem,
  Identifier, VertexProperties, RangeVertexQuery,
};
use serde_json::value::Value;

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

    pub fn insert(&self, name: &str, love: &str) -> Result<String> {
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

    pub fn get_all_vertices(&self) -> Vec<VertexProperties> {
      let vertex_query = RangeVertexQuery::new().limit(4);
      self.store.get_all_vertex_properties(vertex_query.into()).unwrap()
    }
}
