use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use super::models::{AllUsers, RegisterUser, UpdateUser};
use crate::AppState;
use bcrypt::{DEFAULT_COST, hash, verify};
use sqlx::{Pool, Postgres};

#[get("/users")]
async fn get_all_users(app_state: web::Data<AppState>) -> impl Responder {
    let pool = app_state.postgres_client.clone();
    let result = sqlx::query!("SELECT * FROM users")
        .fetch_all(&pool)
        .await;

    match result {
        Ok(users) => {
          HttpResponse::Ok().json(
            users
              .iter()
              .map(|user| AllUsers {
                id: user.id,
                name: user.name.clone(),
                email: user.email.clone(),
                password: user.password.clone(),
              })
              .collect::<Vec<AllUsers>>()
          )
        },
        Err(_) => HttpResponse::InternalServerError().body("Error in get users"),
    }
}

#[post("/users")]
async fn create_user(app_state: web::Data<AppState>, user: web::Json<RegisterUser>) -> impl Responder {
    let hashed = hash(&user.password, DEFAULT_COST).expect("Filed to hash password");

    if !(hashed != user.password) {
      return HttpResponse::InternalServerError().body("Error in hash password");
    }

    let result = sqlx::query!(
        r#"
        INSERT INTO users (name, email, password)
        VALUES ($1, $2, $3)
        RETURNING id, name, email, password
        "#,
        user.name,
        user.email,
        hashed
    )
    .fetch_one(&app_state.postgres_client)
    .await;

    match result {
        Ok(user) => {
          HttpResponse::Ok().json(AllUsers {
            id: user.id,
            name: user.name.clone(),
            email: user.email.clone(),
            password: user.password.clone(),
          })
        },
        Err(_) => HttpResponse::InternalServerError().body("Error in create user"),
    }
}

#[put("/users/{id}")]
async fn update_user(app_state: web::Data<AppState>, user: web::Json<UpdateUser>, id: web::Path<i32>) -> impl Responder {
    let hashed = hash(&user.password, DEFAULT_COST).expect("Filed to hash password");

    if !(hashed != user.password) {
      return HttpResponse::InternalServerError().body("Error in hash password");
    }

    let result = sqlx::query!(
        r#"
        UPDATE users
        SET name = $1, email = $2, password = $3
        WHERE id = $4
        RETURNING id, name, email, password
        "#,
        user.name,
        user.email,
        hashed,
        id.into_inner()
    )
    .fetch_one(&app_state.postgres_client)
    .await;

    match result {
        Ok(user) => {
          HttpResponse::Ok().json(AllUsers {
            id: user.id,
            name: user.name.clone(),
            email: user.email.clone(),
            password: user.password.clone(),
          })
        },
        Err(_) => HttpResponse::InternalServerError().body("Error in update user"),
    }
}

#[delete("/users/{id}")]
async fn delete_user(app_state: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query!(
        r#"
        DELETE FROM users
        WHERE id = $1
        RETURNING id, name, email, password
        "#,
        id.into_inner()
    )
    .fetch_one(&app_state.postgres_client)
    .await;

    match result {
        Ok(user) => {
          HttpResponse::Ok().json(AllUsers {
            id: user.id,
            name: user.name.clone(),
            email: user.email.clone(),
            password: user.password.clone(),
          })
        },
        Err(_) => HttpResponse::InternalServerError().body("Error in delete user"),
    }
}

pub fn users_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_users);
    cfg.service(create_user);
}

