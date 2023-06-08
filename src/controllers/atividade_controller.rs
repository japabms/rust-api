use std::ops::DerefMut;

use actix_web::{delete, get, post, put, web, Responder};

use crate::{models::atividade::*, services::atividade_service, db::DbPool};

#[utoipa::path(
    tag = "Atividade",
)]
#[get("/atividade")]
async fn get_atividades(pool: web::Data<DbPool>) -> impl Responder {
    match atividade_service::find_all(pool.get().unwrap().deref_mut()){
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(
    tag = "Atividade",
)]
#[get("/atividade/{id}")]
async fn get_atividade_by_id(id: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    match atividade_service::find_by_id(id.into_inner(), pool.get().unwrap().deref_mut()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(
    tag = "Atividade",
    request_body = AtividadeDTO,
)]
#[post("/atividade")]
async fn post_atividade(json: web::Json<AtividadeDTO>, pool: web::Data<DbPool>) -> impl Responder {
    match atividade_service::insert(json.into_inner(), pool.get().unwrap().deref_mut()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(
    tag = "Atividade",
    request_body = AtividadeDTO,
)]
#[put("/atividade/{id}")]
async fn put_atividade(id: web::Path<i32>, json: web::Json<AtividadeDTO>, pool: web::Data<DbPool>) -> impl Responder {
    match atividade_service::update(id.into_inner(), json.into_inner(), pool.get().unwrap().deref_mut()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(
    tag = "Atividade",
)]
#[delete("/atividade/{id}")]
async fn delete_atividade(id: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    match atividade_service::delete(id.into_inner(), pool.get().unwrap().deref_mut()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}
