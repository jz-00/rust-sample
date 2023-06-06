use std::fs::File;
use std::io::prelude::*;

pub use actix_web::Resource as Factory;
use actix_web::{web, HttpResponse, Result};
use image::{imageops, GenericImageView};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::enabled_true;

pub const ROUTE: &str = "/upload/image";

#[derive(Serialize)]
struct ImageInfo {
    url: String,
    thumb_url: String,
    width: u32,
    height: u32,
    size: usize,
}

#[derive(Serialize)]
struct ImageUploadResult {
    image: Option<ImageInfo>,
    err: Option<String>,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct UploadConfig {
    #[serde(default = "enabled_true")]
    pub enabled: bool,
    pub upload_dir: String,
    pub serving_url: String,
}

#[derive(Debug, Clone)]
pub struct UploadService {
    config: UploadConfig,
}

impl UploadService {
    pub fn new(config: UploadConfig) -> UploadService {
        UploadService { config }
    }

    async fn upload_image(
        upload_dir: String,
        serving_url: String,
        image_data: web::Bytes,
    ) -> Result<HttpResponse> {
        let mut err = None;
        let mut image = None;

        // get image dimensions
        let (width, height) = match image::load_from_memory(&image_data) {
            Ok(img) => {
                img.resize(1024, 1024, imageops::FilterType::Triangle);
                // img.resize(1024, 1024, imageops::FilterType::Lanczos3);
                img.dimensions()
            }
            Err(e) => {
                err = Some(format!("{}", e));
                (0, 0)
            }
        };

        // create unique filename
        let uuid = Uuid::new_v4();
        let filename = format!("{:}.jpg", uuid.to_simple());
        let filepath = format!("{}/{}", upload_dir, filename);

        // write file
        let size = match File::create(filepath)?.write(&image_data) {
            Ok(size) => size,
            Err(e) => {
                err = Some(format!("{}", e));
                0
            }
        };

        // create url
        let url = format!("{}/{}", serving_url, filename);

        if err.is_none() {
            image = Some(ImageInfo {
                url: url.clone(),
                thumb_url: url,
                width,
                height,
                size,
            })
        }

        Ok(HttpResponse::Ok().json(ImageUploadResult { image, err }))
    }

    pub fn factory(&self) -> Factory {
        let upload_dir = self.config.upload_dir.clone();
        let serving_url = self.config.serving_url.clone();
        let upload_image = move |image_data: web::Bytes| {
            UploadService::upload_image(upload_dir.clone(), serving_url.clone(), image_data)
        };

        web::resource(ROUTE)
            .app_data(web::PayloadConfig::new(3_000_000).mimetype(mime::IMAGE_JPEG))
            // .app_data(web::PayloadConfig::new(3_000_000).mimetype(mime::IMAGE_PNG))
            // .app_data(web::PayloadConfig::new(3_000_000).mimetype(mime::IMAGE_GIF))
            // .app_data(web::PayloadConfig::new(3_000_000).mimetype(mime::IMAGE_BMP))
            .route(web::post().to(upload_image))
    }
}
