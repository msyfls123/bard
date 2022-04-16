use rocket::{Route};

mod graph;
mod file;
mod auth;
mod utils;

pub fn get_routes() -> Vec<Route> {
  routes!{
    graph::vertex_list,
    graph::get_vertex,
    graph::post_vertex,
    graph::create_vertex,
    graph::create_edge,
    graph::get_edge,
    graph::get_graphql_handler,
    graph::post_graphql_handler,
    graph::graphiql,
    file::upload,
    auth::signup,
    auth::login,
    auth::logout,
    auth::me_api,
    auth::me,
    auth::login_page,
  }
}
