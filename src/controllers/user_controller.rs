use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use diesel::prelude::*;
use serde::Deserialize;
use crate::db::DbPool;
use crate::models::{User, NewUser};
use crate::schema::users;

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
}

#[get("/users")]
pub async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    
    let users = users::table
        .load::<User>(&mut conn)
        .expect("Error loading users");

    HttpResponse::Ok().json(users)
}

#[get("/users/{id}")]
pub async fn get_user(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let user = users::table
        .filter(users::id.eq(path.into_inner()))
        .first::<User>(&mut conn)
        .expect("Error loading user");

    HttpResponse::Ok().json(user)
}

#[post("/users")]
pub async fn create_user(
    pool: web::Data<DbPool>,
    new_user: web::Json<NewUser>
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    match diesel::insert_into(users::table)
        .values(&*new_user)
        .execute(&mut conn) {
            Ok(_) => {
                let user = users::table
                    .order(users::id.desc())
                    .first::<User>(&mut conn)
                    .expect("Error loading created user");
                HttpResponse::Created().json(user)
            }
            Err(err) => HttpResponse::InternalServerError().json(err.to_string())
    }
}

#[put("/users/{id}")]
pub async fn update_user(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
    user_data: web::Json<UpdateUser>
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let user_id = path.into_inner();

    match diesel::update(users::table.filter(users::id.eq(user_id)))
        .set(user_data.into_inner())
        .execute(&mut conn) {
            Ok(_) => {
                let user = users::table
                    .filter(users::id.eq(user_id))
                    .first::<User>(&mut conn)
                    .expect("Error loading updated user");
                HttpResponse::Ok().json(user)
            }
            Err(err) => HttpResponse::InternalServerError().json(err.to_string())
    }
}

#[delete("/users/{id}")]
pub async fn delete_user(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    diesel::delete(users::table.filter(users::id.eq(path.into_inner())))
        .execute(&mut conn)
        .expect("Error deleting user");

    HttpResponse::NoContent().finish()
}
