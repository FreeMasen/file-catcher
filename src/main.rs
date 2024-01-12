use std::collections::HashMap;

use actix_web::HttpResponse;
use bytes::Bytes;
use futures::{Stream, StreamExt};

#[tokio::main]
async fn main() {
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(health)
            .service(upload)
            .wrap(actix_web::middleware::Logger::new("piper"))
    })
    .bind(("127.0.0.1", 9876))
    .unwrap()
    .run()
    .await
    .unwrap();
}

#[actix_web::get("/health")]
async fn health() -> HttpResponse {
    println!("GET /health");
    HttpResponse::Ok().json(1)
}

#[actix_web::put("/upload")]
async fn upload(mut payload: actix_web::web::Payload) -> HttpResponse {
    read_payload(&mut payload).await;
    HttpResponse::Ok().json(&HashMap::<(), ()>::new())
}

async fn read_payload(payload: &mut actix_web::web::Payload) {
    while let Some(chunk) = payload.next().await {
        let chunk = chunk.unwrap();
        println!("read {}", chunk.len());
    }
}
