use actix_web::{get, http::StatusCode, web, App, HttpResponse, HttpServer, Responder};
use image::ImageOutputFormat;
use serde::Deserialize;
use std::{io::Cursor, sync::Arc};
use url::Url;

use crate::services;

#[derive(Clone)]
pub struct Server {
    ip: String,
    port: u16,
}

#[derive(Deserialize)]
pub struct ResizeInfo {
    url: String,
    width: u32,
    height: u32,
}

impl Server {
    pub fn new(ip: String, port: u16) -> Self {
        Self { ip, port }
    }

    pub async fn init(&self) -> std::io::Result<()> {
        let server = Arc::new(self.clone());
        HttpServer::new(move || {
            App::new()
                .app_data(server.clone())
                .service(health_check)
                .service(resize_image)
        })
        .bind((&self.ip[..], self.port))?
        .run()
        .await
    }
}

#[get("/")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Healthy 💖")
}

#[get("/resize")]
async fn resize_image(info: web::Query<ResizeInfo>) -> impl Responder {
    let params = info.into_inner();
    let parsed_url = Url::parse(&params.url);
    if parsed_url.is_err() {
        return HttpResponse::BadRequest().body("Invalid URL");
    }
    if params.width == 0 || params.height == 0 {
        return HttpResponse::BadRequest().body("Width and height must be non-zero");
    }
    let mut service = services::ImageService::new();
    _ = service.download_img(params.url.clone()).await;
    match service.resize_image(params.width.clone(), params.height.clone()) {
        Ok(image) => {
            let mut response = HttpResponse::build(StatusCode::OK);
            let mut body = web::BytesMut::new();
            let mut cursor = Cursor::new(Vec::new());
            image
                .write_to(&mut cursor, ImageOutputFormat::Png)
                .expect("Failed to write image");
            body.extend_from_slice(&cursor.into_inner());
            response.content_type("image/png").body(body)
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to resize image"),
    }
}
