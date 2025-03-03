use std::error::Error;
use actix_web::{web::{self, get, method}, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::Row;

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
}

async fn update(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "UPDATE book SET title = $1, author = $2 WHERE isbn = $3";

    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;

    Ok(())
}
async fn delete(book:&Book,pool:&sqlx::PgPool) -> Result<(),Box<dyn Error>>{
    let query = "DELETE from book WHERE title = $1 AND author = $2 AND isbn = $3";

    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;

    Ok(())
}

async fn create(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO book (title, author, isbn) VALUES ($1, $2, $3)";

    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;

    Ok(())
}

async fn get_all(pool: &sqlx::PgPool) -> Result<Vec<Book>, Box<dyn Error>> {
    let query = "SELECT title, author, isbn FROM book";

    let rows = sqlx::query(query).fetch_all(pool).await?;

    let books = rows.iter().map(|row| {
        Book {
            title: row.get("title"),
            author: row.get("author"),
            isbn: row.get("isbn"),
        }
    }).collect();

    Ok(books)
}

async fn get_all_endpoint(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    match get_all(pool.get_ref()).await {
        Ok(books) => HttpResponse::Ok().json(books),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn create_endpoint(books: web::Json<Vec<Book>>, pool : web::Data<sqlx::PgPool>) -> impl Responder{
    
    for book in books.iter(){
        match create(&book, pool.get_ref()).await{
            Ok(_) => (),
            Err(_) => return HttpResponse::InternalServerError().finish(),
        }
    }
    HttpResponse::Ok().finish()
    
    
}

async fn delete_endpoint(books: web::Json<Vec<Book>>, pool: web::Data<sqlx::PgPool>) -> impl Responder{

    for book in books.iter(){
        match delete(&book, pool.get_ref()).await{
            Ok(_) => (),
            Err(_) => return HttpResponse::InternalServerError().finish()
        }
    }

    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://postgres:45GtO723@localhost:5432/bookstore";

    let pool = sqlx::postgres::PgPool::connect(url).await?;
    println!(">>>SERVER RUNNING");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/books")
                    .route("/", web::get().to(get_all_endpoint))
                    .route("/post", web::post().to(create_endpoint))
                    .route("/delete",web::delete().to(delete_endpoint))

            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}


