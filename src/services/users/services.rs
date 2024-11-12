use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use super::models::{AllUser, RegisterUser, UpdateUser};
use crate::AppState;
use bcrypt::{DEFAULT_COST, hash, verify};
use sqlx::{Pool, Postgres};

#[get("/users")]
async fn get_all_users(app_state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query!("SELECT * FROM users")
        .fetch_all(&app_state.postgres_client)
        .await;

    match result{
        Ok(users) => {
            HttpResponse::Ok().json(
                users
                    .iter()
                    .map(|user| AllUser {
                        id: user.id,
                        name: user.name.clone(),
                        email: user.email.clone(),
                        password: user.password.clone(),
                    })
                    .collect::<Vec<AllUser>>()
            )
        },
        Err(_) => HttpResponse::InternalServerError().body("Error trying to get all users from database.")
    }
}

pub fn users_routes(cfg: &mut web::ServiceConfig){
    cfg.service(get_all_users);
}