use crate::{
    config::mongo::DBMongo,
    schemas::project_schema::{
        CreateOwner,
        CreateProject,
        FetchProject,
        Owner,
        Project,
    },
};
use async_graphql::{ Context, EmptySubscription, FieldResult, Object, Schema };

pub struct Query;

#[Object(extends)]
impl Query {
    async fn owner(&self, ctx: &Context<'_>) -> FieldResult<Vec<Owner>> {
        let db = &ctx.data_unchecked::<DBMongo>();
        let owners = db.get_owners().unwrap();
        Ok(owners)
    }

    async fn project(&self, ctx: &Context<'_>, input: FetchProject) -> FieldResult<Project> {
        let db = &ctx.data_unchecked::<DBMongo>();
        let project = db.single_project(&input._id).unwrap();
        Ok(project)
    }

    async fn get_projects(&self, ctx: &Context<'_>) -> FieldResult<Vec<Project>> {
        let db = &ctx.data_unchecked::<DBMongo>();
        let projects = db.get_projects().unwrap();
        Ok(projects)
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_owner(&self, ctx: &Context<'_>, input: CreateOwner) -> FieldResult<Owner> {
        let db = &ctx.data_unchecked::<DBMongo>();
        let new_owner = Owner {
            _id: None,
            email: input.email,
            name: input.name,
            phone: input.phone,
        };
        let owner = db.create_owner(new_owner).unwrap();
        Ok(owner)
    }

    async fn create_project(
        &self,
        ctx: &Context<'_>,
        input: CreateProject
    ) -> FieldResult<Project> {
        let db = &ctx.data_unchecked::<DBMongo>();
        let new_project = Project {
            _id: None,
            owner_id: input.owner_id,
            name: input.name,
            description: input.description,
            status: input.status,
        };
        let project = db.create_project(new_project).unwrap();
        Ok(project)
    }
}

pub type ProjectSchema = Schema<Query, Mutation, EmptySubscription>;