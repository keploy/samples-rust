mod config;
mod handler;
mod schemas;

use async_graphql::{
    http::{ playground_source, GraphQLPlaygroundConfig },
    EmptySubscription,
    Schema,
};
use async_graphql_rocket::{ GraphQLQuery, GraphQLRequest, GraphQLResponse };
use config::mongo::DBMongo;
use handler::graphql_handler::{ Mutation, ProjectSchema, Query };
use rocket::{ response::content, routes, State };

#[rocket::get("/graphql?<query..>")]
async fn graphql_query(schema: &State<ProjectSchema>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema.inner()).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_mutation(
    schema: &State<ProjectSchema>,
    request: GraphQLRequest
) -> GraphQLResponse {
    request.execute(schema.inner()).await
}

#[rocket::get("/")]
async fn graphql_playground() -> content::RawHtml<String> {
    content::RawHtml(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[rocket::launch]
fn rocket() -> _ {
    let db = DBMongo::init();
    let schema = Schema::build(Query, Mutation, EmptySubscription).data(db).finish();
    rocket
        ::build()
        .manage(schema)
        .mount("/", routes![graphql_query, graphql_mutation, graphql_playground])
}