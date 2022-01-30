use rocket::{Route};

mod graph;
mod file;

pub fn get_routes() -> Vec<Route> {
  routes!{
    graph::vertex_list,
    graph::get_vertex,
    graph::post_vertex,
    graph::create_vertex,
    graph::create_edge,
    graph::get_edge,
    file::upload,
  }
}
