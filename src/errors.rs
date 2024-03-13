use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use derive_more::{Display, From};
use pg::error::Error as PGError;
use serde_json::Error as SerdeError;
use std::io::Error as IOError;
//use actix_web::Error as ActixError;
//use tokio_postgres::error::Error as PGError;
//use askama::Error as AskamaError;

#[derive(Display, From, Debug)]
pub enum MyError {
    NotFound,
    PGError(PGError),
    PoolError(PoolError),
    IOError(IOError),
    SerdeError(SerdeError),
    // ActixError(ActixError),
    // AskamaError(AskamaError),
}

impl std::error::Error for MyError {}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            MyError::NotFound => HttpResponse::NotFound().finish(),
            MyError::PoolError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
            MyError::SerdeError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
            MyError::IOError(ref err) => HttpResponse::InternalServerError().body(err.to_string()),
            //MyError::AskamaError(ref err) => {
            //HttpResponse::InternalServerError().body(err.to_string())
            //}
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
