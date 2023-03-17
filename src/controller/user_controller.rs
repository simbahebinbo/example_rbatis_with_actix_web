use actix_web::{dev::HttpServiceFactory, HttpResponse, post, get, Responder, Result, web};
use actix_web::http::StatusCode;
use rbatis::rbdc::db::ExecResult;

use crate::common::{ApiResult, ApiResultErr, AppState};
use crate::model::User;

rbatis::crud!(User {});

#[utoipa::path(
request_body = User,
responses(
(status = 201, description = "User created successfully", body = ApiResult < User >),
(status = 409, description = "User with id already exists", body = ApiResultErr)
)
)]
#[post("/save")]
pub async fn user_save(state: web::Data<AppState>, body: web::Json<User>) -> Result<impl Responder> {
    let mut db = &state.pool.clone();
    let mut user = body.to_owned();
    let result: ExecResult = User::insert(&mut db, &user).await.unwrap();
    user.set_id(result.last_insert_id.as_u64());

    let response = HttpResponse::Ok()
        .status(StatusCode::CREATED)
        .json(ApiResult::new(0, "", user));
    Ok(response)
}

#[utoipa::path(
responses(
(status = 200, description = "Update user by id", body = ApiResult < User >)
)
)]
#[post("/update")]
async fn user_update(state: web::Data<AppState>, body: web::Json<User>) -> Result<impl Responder> {
    let db = &mut state.pool.clone();
    let user = body.to_owned();

    let result: ExecResult = User::update_by_column(db, &user, "id").await.unwrap();
    let response = if result.rows_affected == 1 {
        HttpResponse::Ok().json(ApiResult::new(0, "", user))
    } else {
        let s = format!("The updated resource {} not found", user.id.unwrap());
        HttpResponse::Ok()
            .status(StatusCode::GONE)
            .json(ApiResultErr::new(0, &s))
    };
    Ok(response)
}


#[utoipa::path(
responses(
(status = 200, description = "List all users", body = ApiResult < User >)
)
)]
#[get("/list")]
async fn user_list(state: web::Data<AppState>) -> Result<impl Responder> {
    let db = &mut state.pool.clone();
    let result: Vec<User> = User::select_all(db).await.unwrap();
    let response = HttpResponse::Ok().json(ApiResult::new(0, "", result));
    Ok(response)
}


#[utoipa::path(
responses(
(status = 200, description = "Show user by id", body = ApiResult < User >)
)
)]
#[post("/show")]
async fn user_show(state: web::Data<AppState>, body: web::Json<User>) -> Result<impl Responder> {
    let db = &mut state.pool.clone();
    let user = body.to_owned();

    let result: Vec<User> = User::select_by_column(db, "id", user.id).await.unwrap();
    let response = HttpResponse::Ok().json(ApiResult::new(0, "", result));
    Ok(response)
}


#[utoipa::path(
responses(
(status = 200, description = "Delete user by id", body = ApiResult < User >)
)
)]
#[post("/delete")]
async fn user_delete(state: web::Data<AppState>, body: web::Json<User>) -> Result<impl Responder> {
    let db = &mut state.pool.clone();
    let user = body.to_owned();

    let result: ExecResult = User::delete_by_column(db, "id", user.id).await.unwrap();
    let response = if result.rows_affected > 0 {
        HttpResponse::Ok().json(ApiResult::new(0, "", user.id))
    } else {
        let s = format!("The delete resource {} not found", user.id.unwrap());
        HttpResponse::Ok()
            .status(StatusCode::GONE)
            .json(ApiResultErr::new(0, &s))
    };
    Ok(response)
}


pub fn api_routes() -> impl HttpServiceFactory {
    web::scope("/user")
        .service(user_save)
        .service(user_update)
        .service(user_list)
        .service(user_show)
        .service(user_delete)
}

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(api_routes());
}
