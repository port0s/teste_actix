use actix_web::{web, App, HttpServer};

mod routes;
use routes::ping::*;
use routes::info::*;
use routes::catalogo::*;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let api = HttpServer::new(|| {
        App::new()
            .route("/ping", web::get().to(ping))
            .route("/info", web::get().to(info))
            .route("/cat", web::get().to(catalogo))
    });
    let porta = 10020;

    let api = api.bind(format!("127.0.0.1:{}", porta))
        .expect("Não consegui conectar");

    println!("Conectado com sucesso \n http://localhost:{}/ping", porta);

    api.run()
    .await
}

