//任务一：使用 hyper 或 actix-web 库编写一个简单的 HTTP 服务器，
//当访问/hello 路径时返回"Hello, Rust!"文本响应。
//任 务 二 ： 扩 展 基 础 HTTP 服 务 器 ， 使 其 能 托 管 静 态 文 件 （ 如
//HTML/CSS），当访问根路径/时返回一个自定义的欢迎页面。
//任务三：用 serde 和 actix-web 实现一个图书管理 API，支持 GET
///books（获取所有书籍）和 POST /books（添加新书籍），数据暂存内存中。
//任务四：为 Web 服务器添加日志中间件，记录每个请求的路径、响
//应状态码和耗时，使用 tracing 或 log 库输出到控制台。

use actix_web::{get, App, HttpServer, Responder, HttpResponse};
use actix_files::Files;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use actix_web::web;

#[derive(Serialize, Deserialize, Clone)]
struct Book {
    //id: u32,
    title: String,
    author: String,
}
struct AppState {
    books: Mutex<Vec<Book>>,
}


// GET /books
async fn get_books(data: web::Data<AppState>) -> impl Responder {
    let books = data.books.lock().unwrap();
    HttpResponse::Ok().json(&*books)
}

// POST /books
async fn add_book(book: web::Json<Book>, data: web::Data<AppState>) -> impl Responder {
    let mut books = data.books.lock().unwrap();
    books.push(book.into_inner());
    HttpResponse::Created().body("Book added successfully")
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, Rust!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        books: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone()) // 👈 克隆 Data 的 Arc 指针
            .service(hello)
            .route("/books", web::get().to(get_books))
            .route("/books", web::post().to(add_book))
            .service(Files::new("/snake", "/Users/zilong/coding/weblab/catbook-react/").index_file("index.html"))
            .service(Files::new("/", "/Users/zilong/coding/weblab/").index_file("hello.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

