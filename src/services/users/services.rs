use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use super::models::{AllUser, RegisterUser, UpdateUser};
use crate::AppState;
use bcrypt::{DEFAULT_COST, hash};

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
                    })
                    .collect::<Vec<AllUser>>()
            )
        },
        Err(_) => HttpResponse::InternalServerError().body("Error trying to get all users from database.")
    }
}

#[post("/users")]
async fn create_user(app_state: web::Data<AppState>, user: web::Json<RegisterUser>) -> impl Responder {
    let hasded = hash(&user.password, DEFAULT_COST).expect("Failed to hash password");
    let email_check = sqlx::query!("SELECT id FROM users WHERE email = $1", user.email)
        .fetch_optional(&app_state.postgres_client)
        .await;

    match email_check {
        Ok(Some(_)) => {
            return HttpResponse::Conflict().body("Email ja cadastrado");
        },
        Ok(None) => {},
        Err(_) => {
            return HttpResponse::InternalServerError().body("Erro ao verificar email");
        }
    }

    if !(hasded != user.password){
        return HttpResponse::InternalServerError().body("Error trying to hash password");
    }

    let result = sqlx::query!(
        "INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING id, name, email, password",
        user.name, 
        user.email, 
        hasded
    ).fetch_one(&app_state.postgres_client).await;

    match result {
        Ok(user) => HttpResponse::Ok().json(AllUser {
            id: user.id,
            name: user.name,
            email: user.email
        }),
        Err(_) => HttpResponse::InternalServerError().body("Error trying to create user.")
    }
}

#[put("/users/{id}")]
async fn update_user(app_state: web::Data<AppState>, user: web::Json<UpdateUser>, id: web::Path<i32>) -> impl Responder {
    let hasded = hash(&user.password, DEFAULT_COST).expect("Failed to hash password");

    if !(hasded != user.password){
        return HttpResponse::InternalServerError().body("Error trying to hash password");
    }

    let result = sqlx::query!(
        "UPDATE users SET name = $1, email = $2, password = $3 WHERE id = $4 RETURNING id, name, email, password",
        user.name, 
        user.email,
        hasded,
        id.into_inner()
    )
    .fetch_optional(&app_state.postgres_client)
    .await;

    match result {
        Ok(Some(_user)) => HttpResponse::Ok().body("Usuário atualizado com sucesso"),
        Ok(None) => HttpResponse::NotFound().body("Usuário não encontrado"),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao tentar atualizar o usuário"),
    }

}

#[delete("/users/{id}")]
async fn delete_user(app_state: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query!(
        "DELETE FROM users WHERE id = $1 RETURNING id, name, email, password",
        id.into_inner()
    ).fetch_optional(&app_state.postgres_client).await;

    match result {
        Ok(Some(_user)) => HttpResponse::Ok().body("Usuario excluido com sucesso"),
        Ok(None) => HttpResponse::NotFound().body("Usuario não encontrado"),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao tentar excluir o usuario")
    }

}

pub fn users_routes(cfg: &mut web::ServiceConfig){
    cfg.service(get_all_users);
    cfg.service(create_user);
    cfg.service(update_user);
    cfg.service(delete_user);
}