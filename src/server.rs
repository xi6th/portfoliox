use actix_web::{ HttpServer, App};
use actix_web::middleware::Logger;
use env_logger::Env;
use std::env;
use actix_files::Files;
use actix_web;


#[actix_web::main]
pub async fn server() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    // Get the port number to listen on.
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    // Start a server, configuring the resources to serve.
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent} %{{}}i"))
            .service(Files::new("/", "./static/root").index_file("index.html"))
    })
    .bind(("0.0.0.0", port))
    .expect("Can not bind to port 8000")
    .run()
    .await
}
