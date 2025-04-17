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

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, Rust!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(Files::new("/snake", "/Users/zilong/coding/weblab/catbook-react/").index_file("index.html"))
            .service(Files::new("/", "/Users/zilong/coding/weblab/").index_file("hello.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
