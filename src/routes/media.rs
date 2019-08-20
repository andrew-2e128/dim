use crate::core::DbConnection;
use crate::dim_database::media::{Media, UpdateMedia};
use rocket::http::Status;
use rocket_contrib::json::Json;

#[get("/<id>")]
pub fn get_media_by_id(conn: DbConnection, id: i32) -> Result<Json<Media>, Status> {
    match Media::get(&conn, id) {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(Status::NotFound),
    }
}

#[patch("/<id>", format = "application/json", data = "<data>")]
pub fn update_media_by_id(
    conn: DbConnection,
    id: i32,
    data: Json<UpdateMedia>,
) -> Result<Status, Status> {
    match data.update(&conn, id) {
        Ok(_) => Ok(Status::NoContent),
        Err(_) => Err(Status::NotModified),
    }
}

#[delete("/<id>")]
pub fn delete_media_by_id(
    conn: DbConnection,
    id: i32,
) -> Result<Status, Status> {
    match Media::delete(&conn, id) {
        Ok(_) => Ok(Status::Ok),
        Err(_) => Err(Status::NotFound),
    }
}
