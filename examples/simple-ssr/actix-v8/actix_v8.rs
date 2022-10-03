use actix_files as fs;
use actix_web::{get, http::StatusCode, middleware::Logger, App, HttpResponse, HttpServer};
use std::fs::read_to_string;

use ssr_react::Ssr;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(
                fs::Files::new("/styles", "./examples/simple-ssr/source/dist/ssr/styles/")
                    .show_files_listing(),
            )
            .service(
                fs::Files::new("/images", "./examples/simple-ssr/source/dist/ssr/images/")
                    .show_files_listing(),
            )
            .service(
                fs::Files::new("/scripts", "./examples/simple-ssr/source/dist/client/")
                    .show_files_listing(),
            )
            .service(index)
    })
    .bind("0.0.0.0:8088")?
    .run()
    .await
}

#[get("/")]
async fn index() -> HttpResponse {
    let source = read_to_string("./examples/simple-ssr/source/dist/ssr/index.js").unwrap();

    let js = Ssr::new(source, "SSR");
    let html = js.render_to_string(None);

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(html)
}
