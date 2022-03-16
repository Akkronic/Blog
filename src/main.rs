use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use askama::Template;
use actix_files as fs;

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {

}

#[get("/")]
async fn home() -> impl Responder {
    let tpl = HomeTemplate {};
    HttpResponse::Ok().body(tpl.render().unwrap())
}

#[get("/page/")]
async fn page() -> impl Responder {
    HttpResponse::Ok().body("hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(fs::Files::new("/imgs", "./imgs"))
            .service(fs::Files::new("/js", "./js"))
            .service(fs::Files::new("/css", "./css"))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}