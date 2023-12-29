# Rust Image Processor

It's just a POC for now. The output is hard-coded to be PNG.

The Rust Image Processor is a lightweight API that offers endpoints for scaling and manipulating images. It is designed to be efficient and easy to use for image-processing tasks.

## Usage

To use the Rust Image Processor, send HTTP requests to its endpoints for scaling and manipulating images.

### Example

````bash
curl -X GET "http://localhost:8080/resize?url=https://www.example.com/img/img.png&width=100&height=100" --output resized_image.png
````
