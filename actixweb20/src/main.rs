use actix_web::{web, App, HttpServer};

mod controller;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Actix-web app!");

    HttpServer::new(|| {
        App::new()
            .route("/helloWorld", web::get().to(
                controller::hello_world_controller::hello_world))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
