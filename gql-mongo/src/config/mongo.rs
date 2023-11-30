use dotenv::dotenv;
use std::{ env, io::Error };
use mongodb::{ bson::{ doc, oid::ObjectId }, sync::{ Client, Collection, Database } };
use crate::schemas::project_schema::{ Owner, Project };

pub struct DBMongo {
    db: Database,
}

impl DBMongo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variables"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("projectMngt");
        DBMongo { db }
    }

    fn col_helper<T>(data_source: &Self, collection_name: &str) -> Collection<T> {
        data_source.db.collection(collection_name)
    }

    pub fn create_owner(&self, new_owner: Owner) -> Result<Owner, Error> {
        let new_doc = Owner {
            _id: None,
            name: new_owner.name.clone(),
            email: new_owner.email.clone(),
            phone: new_owner.phone.clone(),
        };
        let col = DBMongo::col_helper::<Owner>(&self, "owner");
        let data = col.insert_one(new_doc, None).ok().expect("Error creating owner");
        let new_owner = Owner {
            _id: data.inserted_id.as_object_id(),
            name: new_owner.name.clone(),
            email: new_owner.email.clone(),
            phone: new_owner.phone.clone(),
        };
        Ok(new_owner)
    }

    pub fn get_owners(&self) -> Result<Vec<Owner>, Error> {
        let col = DBMongo::col_helper::<Owner>(&self, "owner");
        let cursors = col.find(None, None).ok().expect("Error getting list of owners");
        let owners: Vec<Owner> = cursors.map(|doc| doc.unwrap()).collect();
        Ok(owners)
    }

    pub fn single_owner(&self, id: &String) -> Result<Owner, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! { "_id": obj_id };
        let col = DBMongo::col_helper::<Owner>(&self, "owner");
        let owner_detail = col.find_one(filter, None).ok().expect("Error getting owner's detail");
        Ok(owner_detail.unwrap())
    }

    pub fn create_project(&self, new_project: Project) -> Result<Project, Error> {
        let new_doc = Project {
            _id: None,
            owner_id: new_project.owner_id.clone(),
            name: new_project.name.clone(),
            description: new_project.description.clone(),
            status: new_project.status.clone(),
        };
        let col = DBMongo::col_helper::<Project>(&self, "project");
        let data = col.insert_one(new_doc, None).ok().expect("Error creating project");
        let new_project = Project {
            _id: data.inserted_id.as_object_id(),
            owner_id: new_project.name.clone(),
            name: new_project.name.clone(),
            description: new_project.description.clone(),
            status: new_project.status.clone(),
        };
        Ok(new_project)
    }

    pub fn get_projects(&self) -> Result<Vec<Project>, Error> {
        let col = DBMongo::col_helper::<Project>(&self, "project");
        let cursors = col.find(None, None).ok().expect("Error getting list of projects");
        let projects: Vec<Project> = cursors.map(|doc| doc.unwrap()).collect();
        Ok(projects)
    }

    pub fn single_project(&self, id: &String) -> Result<Project, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! { "_id": obj_id };
        let col = DBMongo::col_helper::<Project>(&self, "project");
        let project_detail = col
            .find_one(filter, None)
            .ok()
            .expect("Error getting project's detail");
        Ok(project_detail.unwrap())
    }
}