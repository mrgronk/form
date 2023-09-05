use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{InsertOneResult, UpdateResult, DeleteResult},
    sync::{Client, Collection},
};
use crate::models::post_model::Post;

pub struct MongoRepo {
    col: Collection<Post>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col: Collection<Post> = db.collection("Post");
        MongoRepo { col }
    }

    pub fn create_post(&self, new_post: Post) -> Result<InsertOneResult, Error> {
        let new_doc = Post {
            id: None,
            name: new_post.name,
            location: new_post.location,
            title: new_post.title,
        };
        let post = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating post");
        Ok(post)
    }

    pub fn get_post(&self, id: &String) -> Result<Post, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let post_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting post's detail");
        Ok(post_detail.unwrap())
    }

    pub fn update_post(&self, id: &String, new_post: Post) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_post.id,
                    "name": new_post.name,
                    "location": new_post.location,
                    "title": new_post.title,
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating post");
        Ok(updated_doc)
    }

    pub fn delete_post(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let post_detail = self
            .col
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting post");
        Ok(post_detail)
    }

    pub fn get_all_posts(&self) -> Result<Vec<Post>, Error> {
        let cursors = self
            .col
            .find(None, None)
            .ok()
            .expect("Error getting list of posts");
        let users = cursors.map(|doc| doc.unwrap()).collect();
        Ok(users)
    }
}