use crate::{
    models::post_model::Post,
    repository::mongodb_repo::MongoRepo,
};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{http::Status, serde::json::Json, State};

#[post("/post", data = "<new_post>")]
pub fn create_post(
    db: &State<MongoRepo>,
    new_post: Json<Post>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = Post {
        id: None,
        name: new_post.name.to_owned(),
        location: new_post.location.to_owned(),
        title: new_post.title.to_owned(),
    };
    let post_detail = db.create_post(data);
    match post_detail {
        Ok(post) => Ok(Json(post)),
        Err(_) => Err(Status::InternalServerError), 
    }
}

#[get("/post/<path>")]
pub fn get_post(db: &State<MongoRepo>, path: String) -> Result<Json<Post>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let post_detail = db.get_post(&id);
    match post_detail {
        Ok(post) => Ok(Json(post)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/post/<path>", data = "<new_post>")]
pub fn update_post(
    db: &State<MongoRepo>,
    path: String,
    new_post: Json<Post>,
) -> Result<Json<Post>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest)
    };
    let data = Post {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_post.name.to_owned(),
        location: new_post.location.to_owned(),
        title: new_post.title.to_owned(),
    };
    let update_result = db.update_post(&id, data);
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_post_info = db.get_post(&id);
                return match updated_post_info {
                    Ok(post) => Ok(Json(post)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/post/<path>")]
pub fn delete_post(db: &State<MongoRepo>, path: String) -> Result<Json<&str>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = db.delete_post(&id);
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("User deleted"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/posts")]
pub fn get_all_posts(db: &State<MongoRepo>) -> Result<Json<Vec<Post>>, Status> {
    let posts = db.get_all_posts();
    match posts {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::InternalServerError),
    }
}