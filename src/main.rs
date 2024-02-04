use chrono::prelude::*;
use db::DB;
use serde::{Deserialize,Serialize};
use std::convert::Infallible;

use warp::{Filter,Rejection};

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> =std::result::Result<T, Rejection>;

mod db;
mod error;
mod handler;

#[derive(Serialize,Deserialize,Debug)]
pub struct Book{
    pub id: String,
    pub name: String,
    pub author: String,
    pub num_page: usize,
    pub added_at: DateTime<Utc>,
    pub tags: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()>{
    let db = DB::init().await?;
    let book = warp::path("Book");

    let book_router = book
    .and(warp::post())
    .and(warp::body::json())
    .and(with_db(db.clone())) // with_db is function.
    .and_then(handler::create_book_handler) //forward the handler request.
    .or(book
        .and(warp::put())
        .and(warp::path::param())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handler::edit_book_handler))
    .or(book
        .and(warp::delete())
        .and(warp::path::param())
        .and(with_db(db.clone()))
        .and_then(handler::delete_book_handler))
    .or(book
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(handler::book_list_handler));

    let router = book_router.recover(error::handler_rejection);
    println!("Started on port 8080");
    warp::serve(router).run(([0, 0, 0, 0],8080)).await;
    Ok(())
}

fn with_db(db:DB) ->impl Filter<Extract =(DB,), Error =Infallible> + Clone{
    warp::any().map(move || db.clone())
}
