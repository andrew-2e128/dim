use diesel::prelude::*;
use rocket::fairing::AdHoc;
use rocket::Request;
use rocket::Rocket;
use rocket_contrib::json::JsonValue;
use rocket_cors;
use rocket::http::Method;

#[allow(unused_imports)]
use crate::routes;

embed_migrations!();

#[database("openflix")]
pub struct DbConnection(PgConnection);

#[catch(503)]
fn service_not_available(_req: &Request) -> JsonValue {
    json!({
        "type": 503,
        "error": "Database is down"
    })
}

#[catch(404)]
fn service_not_found(_req: &Request) -> JsonValue {
    json!({
        "type": 404,
        "error": "Endpoint not found"
    })
}

#[catch(422)]
fn unprocessable_entity() -> JsonValue {
    json!({
        "type": 422,
        "error": "Invalid json supplied"
    })
}

fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let conn = DbConnection::get_one(&rocket).expect("Database Connection Failed");
    match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    }
}

pub fn rocket() -> Rocket {
    let allowed_origins = rocket_cors::AllowedOrigins::all();

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Delete, Method::Patch].into_iter().map(From::from).collect(),
        allowed_headers: rocket_cors::AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors().unwrap();


    rocket::ignite()
        .attach(DbConnection::fairing())
        .attach(AdHoc::on_attach(
            "Running Database Migrations",
            run_db_migrations,
        ))
        .register(catchers![
            service_not_found,
            service_not_available,
            unprocessable_entity
        ])
        .mount(
            "/api/v1/",
            routes![
                routes::general::dashboard,
                routes::general::banners,
            ],
        )
        .mount(
            "/api/v1/library",
            routes![
                routes::library::library_get,
                routes::library::get_self,
                routes::library::library_post,
                routes::library::library_delete,
                routes::library::get_all_library
            ],
        )
        .mount(
            "/api/v1/media",
            routes![
                routes::media::get_media_by_id,
                routes::media::update_media_by_id,
                routes::media::delete_media_by_id,
            ],
        )
        .mount(
            "/api/v1/tv",
            routes![
                routes::tv::get_tv_by_id,
                routes::tv::get_tv_seasons,
                routes::tv::get_season_by_num,
                routes::tv::patch_season_by_num,
                routes::tv::delete_season_by_num,
                routes::tv::get_episode_by_id,
                routes::tv::patch_episode_by_id,
                routes::tv::delete_episode_by_id,
            ],
        )
        .attach(cors)
}
