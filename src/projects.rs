use actix_web::{web, HttpResponse};
use crate::{projects_ops, DBPool};
use crate::response::Response;

#[get("/projects")]
pub async fn list(pool: web::Data<DBPool>) -> HttpResponse {
    let conn = &mut pool.get().expect("Couldn't get DB connection");
    HttpResponse::Ok().json(Response { results: projects_ops::get_all_projects(conn) })
}