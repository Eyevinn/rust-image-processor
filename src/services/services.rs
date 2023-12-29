use std::fmt;

use image::{self, DynamicImage, ImageError, ImageResult};
use reqwest::{self, Client, Error as ReqwestError};

#[derive(Debug)]
pub enum ImageDownloadError {
    Image(ImageError),
    Reqwest(ReqwestError),
    String(String),
}

impl fmt::Display for ImageDownloadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ImageDownloadError::Image(e) => write!(f, "Image error: {}", e),
            ImageDownloadError::Reqwest(e) => write!(f, "Reqwest error: {}", e),
            ImageDownloadError::String(e) => write!(f, "Reqwest error: {}", e),
        }
    }
}

impl From<ImageError> for ImageDownloadError {
    fn from(err: ImageError) -> ImageDownloadError {
        ImageDownloadError::Image(err)
    }
}

impl From<ReqwestError> for ImageDownloadError {
    fn from(err: ReqwestError) -> ImageDownloadError {
        ImageDownloadError::Reqwest(err)
    }
}

pub struct ImageService {
    url: String,
    client: Option<Client>,
    img: Option<ImageResult<DynamicImage>>,
}

impl ImageService {
    pub fn new() -> Self {
        Self {
            url: String::new(),
            client: None,
            img: None,
        }
    }

    pub async fn download_img(&mut self, url: String) -> Result<(), ImageDownloadError> {
        if self.client.is_none() {
            self.client = Some(reqwest::Client::new());
        }
        self.url = url.clone();
        let response = match self
            .client
            .as_ref()
            .expect("Failed to get client")
            .get(&url)
            .send()
            .await
        {
            Ok(res) => res,
            Err(e) => return Err(ImageDownloadError::Reqwest(e)),
        };
        let img_bytes = match response.bytes().await {
            Ok(bytes) => bytes.as_ref().to_vec(),
            Err(e) => return Err(ImageDownloadError::Reqwest(e)),
        };
        let image = match image::load_from_memory(&img_bytes) {
            Ok(img) => img,
            Err(e) => return Err(ImageDownloadError::Image(e)),
        };
        self.img = Some(Ok(image));
        Ok(())
    }

    pub fn resize_image(
        &mut self,
        width: u32,
        height: u32,
    ) -> Result<DynamicImage, ImageDownloadError> {
        let mut temp: Option<Result<DynamicImage, ImageError>> =
            Some(Ok(image::DynamicImage::new_luma8(0, 0)));
        std::mem::swap(&mut self.img, &mut temp);
        match temp {
            Some(Ok(image)) => {
                let resized = image.resize(width, height, image::imageops::FilterType::Nearest);
                self.img = Some(Ok(resized.clone()));
                Ok(resized)
            }
            Some(Err(e)) => Err(ImageDownloadError::Image(e)),
            None => Err(ImageDownloadError::String(String::from("Got None"))),
        }
    }
}
