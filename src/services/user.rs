use crate::errors::MyError;
use crate::jresult::JResult;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    id: Option<i32>,
    name: String,
    email: String,
    phone: String,
}

impl User {
    fn from_row(row: &pg::row::Row) -> User {
        User {
            id: Some(row.get("id")),
            name: row.get("name"),
            email: row.get("email"),
            phone: row.get("phone"),
        }
    }
}

async fn get_all(pool: web::Data<deadpool_postgres::Pool>) -> Result<HttpResponse, MyError> {
    log::info!("get_all get isteği geldi");

    let client = pool.get().await?;
    let rows = client
        .query("SELECT * FROM users ORDER BY id ", &[])
        .await?;
    let users: Vec<User> = rows.iter().map(|r| User::from_row(r)).collect();
    let res = JResult::ok(users);
    Ok(HttpResponse::Ok().json(&res))
}

async fn get_by_id(
    pool: web::Data<deadpool_postgres::Pool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    log::info!("get_by_id get isteği geldi");

    let id = path.into_inner();

    let client = pool.get().await?;

    let row = client
        .query_opt("SELECT * FROM users WHERE id = $1 ", &[&id])
        .await?;

    let res: JResult<_>;
    if let Some(row) = row {
        res = JResult::ok(User::from_row(&row));
    } else {
        res = JResult::err("User not found");
    }

    Ok(HttpResponse::Ok().json(&res))
}

async fn insert(
    pool: web::Data<deadpool_postgres::Pool>,
    body: web::Bytes,
) -> Result<HttpResponse, MyError> {
    log::info!("post isteği geldi");
    let user = serde_json::from_slice::<User>(&body)?;

    let client = pool.get().await?;
    client
        .execute(
            " INSERT INTO users(name,email,phone) VALUES($1,$2,$3) ",
            &[&user.name, &user.email, &user.phone],
        )
        .await?;

    Ok(HttpResponse::Ok().finish())
}

async fn update(
    pool: web::Data<deadpool_postgres::Pool>,
    body: web::Bytes,
    path: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    log::info!("put isteği geldi");

    let id = path.into_inner();

    let user = serde_json::from_slice::<User>(&body)?;

    let client = pool.get().await?;
    client
        .execute(
            " UPDATE users SET name = $1, email = $2, phone = $3 WHERE id = $4 ",
            &[&user.name, &user.email, &user.phone, &id],
        )
        .await?;

    Ok(HttpResponse::Ok().json(JResult::ok(&user)))
}

async fn delete(
    pool: web::Data<deadpool_postgres::Pool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    log::info!("delete isteği geldi");

    let id = path.into_inner();

    let client = pool.get().await?;
    client
        .execute(" DELETE FROM users WHERE id = $1 ", &[&id])
        .await?;

    Ok(HttpResponse::Ok().finish())
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/user")
            .route(web::get().to(get_all))
            .route(web::post().to(insert)),
    );

    cfg.service(
        web::resource("/user/{id}")
            .route(web::delete().to(delete))
            .route(web::get().to(get_by_id))
            .route(web::put().to(update)),
    );
}
