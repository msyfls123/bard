use rocket::{Route};

mod graph;
mod file;

pub fn get_routes() -> Vec<Route> {
  routes!{
    graph::get_vertex,
    graph::post_vertex,
    file::upload,
  }
}
